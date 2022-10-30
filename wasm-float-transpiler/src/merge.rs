use std::collections::HashMap;

use anyhow::*;
use walrus::{
    ir::{
        dfs_in_order, Block, IfElse, Instr, InstrSeqId, LocalGet, LocalSet, LocalTee, Loop, Visitor,
    },
    DataKind, FunctionBuilder, LocalFunction, LocalId, Module, ModuleData, ModuleFunctions,
    ModuleLocals, ModuleMemories, ModuleTypes,
};

use crate::traversal::{visit_instructions, TraversalInstr};

pub fn merge_modules(mut a: Module, mut b: Module) -> Result<Module> {
    let mem_offset = merge_memories(&mut b.memories, &mut a.memories, &mut b.data, &mut a.data)?;
    println!("mem_offset: {}", mem_offset);

    merge_functions(
        &mut b.types,
        &mut a.types,
        &mut b.locals,
        &mut a.locals,
        &mut b.funcs,
        &mut a.funcs,
    )?;

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

fn merge_functions(
    source_types: &mut ModuleTypes,
    target_types: &mut ModuleTypes,
    source_locals: &mut ModuleLocals,
    target_locals: &mut ModuleLocals,
    source_funcs: &mut ModuleFunctions,
    target_funcs: &mut ModuleFunctions,
) -> Result<()> {
    // let function_id_map = HashMap::new();

    for (b_id, b_fun) in source_funcs.iter_local_mut() {
        println!("function id: {:?}", b_id);
        copy_function(
            source_types,
            target_types,
            source_locals,
            target_locals,
            target_funcs,
            b_fun,
        );
        // let new_id = a_funcs.add_local(b_fun.);
        // function_id_map.insert(b_id, new_id);
    }

    Ok(())
}

fn copy_function(
    source_types: &mut ModuleTypes,
    target_types: &mut ModuleTypes,
    source_locals: &mut ModuleLocals,
    target_locals: &mut ModuleLocals,
    target_funcs: &mut ModuleFunctions,
    f: &LocalFunction,
) {
    // TODO: return FunctionId
    // create new builder with same type
    let ty = source_types.get(f.ty());
    let mut function_builder = FunctionBuilder::new(target_types, ty.params(), ty.results());

    // add locals used in function to target module
    let locals = copy_locals(source_locals, target_locals, f);
    for local in locals.iter() {
        println!("local: {:?}", local);
    }

    let mut instr_seqs = vec![function_builder.func_body_id()];

    let visit = |instr: TraversalInstr| {
        fn add_instr(
            function_builder: &mut FunctionBuilder,
            instr_seqs: &[InstrSeqId],
            instr: impl Into<Instr>,
        ) {
            // add instruction to current instruction sequence
            let mut builder =
                function_builder.instr_seq(*instr_seqs.last().expect("stack never empty"));

            builder.instr(instr);
        }

        match instr {
            TraversalInstr::BlockStart(seq, _)
            | TraversalInstr::LoopStart(seq, _)
            | TraversalInstr::IfStart(seq, _)
            | TraversalInstr::ElseStart(seq, _) => {
                instr_seqs.push(function_builder.dangling_instr_seq(seq.ty).id())
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
}

// find all locals used in the function and add them to the new module
fn copy_locals(
    source_locals: &mut ModuleLocals,
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
            println!("vist_local_id: {:?}", id);
            let local = self.source_locals.get(*id);
            let new_local_id = self.target_locals.add(local.ty());
            self.map.insert(*id, new_local_id);
        }
    }
}
