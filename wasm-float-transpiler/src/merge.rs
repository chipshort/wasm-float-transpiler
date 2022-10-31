use std::collections::HashMap;

use anyhow::*;
use walrus::{
    ir::{
        dfs_in_order, Block, Br, BrIf, BrTable, IfElse, Instr, InstrSeqId, LocalGet, LocalSet,
        LocalTee, Loop, Visitor,
    },
    DataKind, FunctionBuilder, FunctionKind, LocalFunction, LocalId, Module, ModuleData,
    ModuleFunctions, ModuleLocals, ModuleMemories, ModuleTypes,
};

use crate::traversal::{visit_instructions, TraversalInstr};

pub fn merge_modules(mut a: Module, mut b: Module) -> Result<Module> {
    let mem_offset = merge_memories(&mut b.memories, &mut a.memories, &mut b.data, &mut a.data)?;
    println!("mem_offset: {}", mem_offset);

    merge_functions(&mut b, &mut a)?;

    // println!("{:#?}", a);

    // cleanup unused functions
    // walrus::passes::gc::run(&mut a);
    Ok(a)
}

/// copies memory from `b` to `a`, returning the offset at which `b` lives after that
fn merge_memories(
    source_mem: &mut ModuleMemories,
    target_mem: &mut ModuleMemories,
    source_data: &mut ModuleData,
    target_data: &mut ModuleData,
) -> Result<u32> {
    // TODO: handle imported memories?
    let a_mems = target_mem.iter().count();
    let b_mems = source_mem.iter().count();

    Ok(match (a_mems, b_mems) {
        (0, 0) => 0,
        (0, 1) => {
            let mem = source_mem.iter().next().expect("length checked above");
            target_mem.add_local(mem.shared, mem.initial, mem.maximum);
            0
        }
        (1, 0) => 0, // nothing to do, since we modify `a` in place
        (1, 1) => {
            // combine memories
            let a_mem = target_mem.iter_mut().next().expect("length checked above");
            let b_mem = source_mem.iter().next().expect("length checked above");

            let old_a_len = a_mem.initial;
            a_mem.shared = a_mem.shared || b_mem.shared;
            a_mem.initial += b_mem.initial;

            // TODO: no idea if this even works like that?
            a_mem.maximum = a_mem
                .maximum
                .map(|a_max| a_max + b_mem.maximum.unwrap_or_default());
            // TODO: import?

            // copy all data segments from `b` to `a`
            b_mem.data_segments.iter().for_each(|data_id| {
                let data = source_data.get(*data_id);
                a_mem
                    .data_segments
                    .insert(target_data.add(clone_kind(&data.kind), data.value.clone()));
                // remove from `b` so we don't double copy it
                source_data.delete(*data_id);
            });
            old_a_len
        }
        _ => bail!("wasm module with more than one memory is not supported"),
    })
}

fn clone_kind(kind: &DataKind) -> DataKind {
    match kind {
        DataKind::Active(a) => DataKind::Active(a.clone()),
        DataKind::Passive => DataKind::Passive,
    }
}

fn merge_functions(source: &mut Module, target: &mut Module) -> Result<()> {
    // let function_id_map = HashMap::new();

    let source_functions = source.funcs.iter_mut().filter_map(|f| match &f.kind {
        FunctionKind::Local(local) => Some((f.name.take(), local)),
        _ => None,
    });

    // move functions from
    for (name, function) in source_functions {
        copy_function(source, target, function, name);
        // let new_id = a_funcs.add_local(b_fun.);
        // function_id_map.insert(b_id, new_id);
    }

    // TODO: go over all *inserted* functions and change ids of call instructions

    Ok(())
}

fn copy_function(source: &Module, target: &mut Module, f: &LocalFunction, name: Option<String>) {
    // TODO: return FunctionId
    // create new builder with same type
    let ty = source.types.get(f.ty());
    let mut function_builder = FunctionBuilder::new(&mut target.types, ty.params(), ty.results());
    if let Some(name) = name {
        function_builder.name(name);
    }

    // add locals used in function to target module
    let locals = copy_locals(&source.locals, &mut target.locals, f);

    // translates old sequence ids to new ones
    let mut seq_ids = HashMap::new();
    seq_ids.insert(f.entry_block(), function_builder.func_body_id());

    let mut instr_seqs = vec![function_builder.func_body_id()];

    let visit = |instr: TraversalInstr| {
        /// Adds the instruction to the current instruction sequence (last entry in `instr_seqs`)
        fn add_instr(
            function_builder: &mut FunctionBuilder,
            instr_seqs: &[InstrSeqId],
            instr: impl Into<Instr>,
        ) {
            let mut builder =
                function_builder.instr_seq(*instr_seqs.last().expect("stack never empty"));

            builder.instr(instr);
        }

        match instr {
            TraversalInstr::BlockStart(seq, _)
            | TraversalInstr::LoopStart(seq, _)
            | TraversalInstr::IfStart(seq, _)
            | TraversalInstr::ElseStart(seq, _) => {
                // creating a dangling sequence that will be added in the corresponding `TraversalInstr::***End` variant
                let new_seq_id = function_builder.dangling_instr_seq(seq.ty).id();
                seq_ids.insert(seq.id(), new_seq_id);
                instr_seqs.push(new_seq_id);
            }
            TraversalInstr::BlockEnd(_, _) => {
                let seq = instr_seqs.pop().expect("stack never empty");
                add_instr(&mut function_builder, &instr_seqs, Block { seq });
            }
            TraversalInstr::LoopEnd(_, _) => {
                let seq = instr_seqs.pop().expect("stack never empty");
                add_instr(&mut function_builder, &instr_seqs, Loop { seq });
            }
            TraversalInstr::IfEnd(_, _, _) => {
                let else_seq = instr_seqs.pop().expect("stack never empty");
                let if_seq = instr_seqs.pop().expect("stack never empty");
                add_instr(
                    &mut function_builder,
                    &instr_seqs,
                    IfElse {
                        consequent: if_seq,
                        alternative: else_seq,
                    },
                );
            }
            // TODO: replace memory access with new memory offset
            // replace local accesses with new local ids
            TraversalInstr::Instr(Instr::LocalGet(l)) => add_instr(
                &mut function_builder,
                &instr_seqs,
                LocalGet {
                    local: locals[&l.local],
                },
            ),
            TraversalInstr::Instr(Instr::LocalSet(l)) => add_instr(
                &mut function_builder,
                &instr_seqs,
                LocalSet {
                    local: locals[&l.local],
                },
            ),
            TraversalInstr::Instr(Instr::LocalTee(l)) => add_instr(
                &mut function_builder,
                &instr_seqs,
                LocalTee {
                    local: locals[&l.local],
                },
            ),
            // replace instruction sequence ids with new ones
            TraversalInstr::Instr(Instr::Br(Br { block })) => {
                add_instr(
                    &mut function_builder,
                    &instr_seqs,
                    Br {
                        block: seq_ids[block],
                    },
                );
            }
            TraversalInstr::Instr(Instr::BrIf(BrIf { block })) => {
                // check if sequence id is in parents
                // if !instr_seqs.iter().any(|&seq| seq == seq_ids[block]) {
                //     panic!("invalid br_if");
                // }
                add_instr(
                    &mut function_builder,
                    &instr_seqs,
                    BrIf {
                        block: seq_ids[block],
                    },
                );
            }
            TraversalInstr::Instr(Instr::BrTable(BrTable { blocks, default })) => {
                add_instr(
                    &mut function_builder,
                    &instr_seqs,
                    BrTable {
                        blocks: blocks.iter().map(|b| seq_ids[b]).collect(),
                        default: seq_ids[default],
                    },
                );
            }
            // TODO: also change other instructions with new ids
            TraversalInstr::Instr(i) => {
                // all other instructions are copied as is
                add_instr(&mut function_builder, &instr_seqs, i.clone());
            }
            // not insterested in these
            TraversalInstr::FunctionStart(_) => {}
            TraversalInstr::FunctionEnd(_) => {}
        }
    };

    visit_instructions(visit, f, f.entry_block());

    let args = f.args.iter().map(|a| locals[a]).collect();
    function_builder.finish(args, &mut target.funcs);
}

// find all locals used in the function and add them to the new module
fn copy_locals(
    source_locals: &ModuleLocals,
    target_locals: &mut ModuleLocals,
    f: &LocalFunction,
) -> HashMap<LocalId, LocalId> {
    let mut locals = CopyLocals {
        source_locals,
        target_locals,
        map: Default::default(),
    };
    dfs_in_order(&mut locals, f, f.entry_block());

    return locals.map;

    struct CopyLocals<'a> {
        source_locals: &'a ModuleLocals,
        target_locals: &'a mut ModuleLocals,
        map: HashMap<LocalId, LocalId>,
    }
    impl<'a, 'b> Visitor<'a> for CopyLocals<'b> {
        fn visit_local_id(&mut self, id: &LocalId) {
            self.map.entry(*id).or_insert_with(|| {
                let local = self.source_locals.get(*id);
                self.target_locals.add(local.ty())
            });
        }
    }
}
