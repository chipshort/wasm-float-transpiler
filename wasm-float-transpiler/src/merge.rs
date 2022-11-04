use std::collections::{HashMap, HashSet};

use anyhow::*;
use rayon::prelude::*;
use walrus::{
    ir::{
        dfs_in_order, dfs_pre_order_mut, AtomicNotify, AtomicRmw, AtomicWait, Block, Br, BrIf,
        BrTable, Cmpxchg, IfElse, Instr, InstrSeqId, Load, LoadSimd, LocalGet, LocalSet, LocalTee,
        Loop, Store, Visitor, VisitorMut,
    },
    DataKind, FunctionBuilder, FunctionId, FunctionKind, LocalFunction, LocalId, Module,
    ModuleLocals,
};

use crate::traversal::{visit_instructions, TraversalInstr};

/// Merges the two modules into a single module.
/// The combined module will have the functions from both.
/// # WARNING
/// This is very experimental and will probably break in many cases.
/// It works for simple modules that don't use memory or tables.
pub fn merge_modules(mut a: Module, mut b: Module) -> Result<Module> {
    let mem_offset = merge_memories(&mut b, &mut a)?;
    println!("mem_offset: {}", mem_offset);

    merge_functions(&b, &mut a, mem_offset)?;
    // TODO: copy everything else, like globals, tables, elements, data etc.

    Ok(a)
}

/// copies memory from `b` to `a`, returning the offset at which `b` lives after that
fn merge_memories(source: &mut Module, target: &mut Module) -> Result<u32> {
    // TODO: handle imported memories?
    let target_mems = target.memories.iter().count();
    let source_mems = source.memories.iter().count();

    Ok(match (target_mems, source_mems) {
        (0, 0) => 0,
        (0, 1) => {
            let mem = source.memories.iter().next().expect("length checked above");
            target
                .memories
                .add_local(mem.shared, mem.initial, mem.maximum);
            0
        }
        (1, 0) => 0, // nothing to do, since we modify `target` in place
        (1, 1) => {
            // combine memories
            let target_mem = target
                .memories
                .iter_mut()
                .next()
                .expect("length checked above");
            let source_mem = source.memories.iter().next().expect("length checked above");

            let old_target_len = target_mem.initial;
            target_mem.shared = target_mem.shared || source_mem.shared;
            target_mem.initial += source_mem.initial;

            // TODO: no idea if this even works like that?
            target_mem.maximum = Some(
                target_mem.maximum.unwrap_or(target_mem.initial)
                    + source_mem.maximum.unwrap_or(source_mem.initial),
            );
            // TODO: import?

            // copy all data segments from `source` to `target`
            source_mem.data_segments.iter().for_each(|data_id| {
                let data = source.data.get(*data_id);
                target_mem
                    .data_segments
                    .insert(target.data.add(clone_kind(&data.kind), data.value.clone()));
                // remove, so we don't double copy it
                source.data.delete(*data_id);
            });
            old_target_len
        }
        _ => bail!("wasm module with more than one memory is not supported"),
    })
}

/// copies the table from `source` to `target`, returning the offset at which `b` lives after that
fn merge_tables(source: &mut Module, target: &mut Module) -> Result<u32> {
    // TODO: handle imported memories?
    let target_tables = target.tables.iter().count();
    let source_tables = source.tables.iter().count();

    Ok(match (target_tables, source_tables) {
        (0, 0) => 0,
        (0, 1) => {
            let tbl = source.tables.iter().next().expect("length checked above");
            target
                .tables
                .add_local(tbl.initial, tbl.maximum, tbl.element_ty);
            0
        }
        (1, 0) => 0, // nothing to do, since we modify `target` in place
        (1, 1) => {
            // combine tables
            let target_tbl = target
                .tables
                .iter_mut()
                .next()
                .expect("length checked above");
            let source_tbl = source.tables.iter().next().expect("length checked above");

            let old_target_len = target_tbl.initial;
            target_tbl.initial += source_tbl.initial;
            // TODO: no idea if this even works like that?
            target_tbl.maximum = target_tbl
                .maximum
                .map(|a_max| a_max + source_tbl.maximum.unwrap_or_default());
            // TODO: import?

            // copy all element segments from `source` to `target`
            source_tbl.elem_segments.iter().for_each(|elem_id| {
                let elem = source.elements.get(*elem_id);
                target_tbl.elem_segments.insert(target.elements.add(
                    elem.kind,
                    elem.ty,
                    elem.members.clone(),
                ));
                // remove from `b` so we don't double copy it
                source.elements.delete(*elem_id);
            });
            old_target_len
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

fn merge_functions(source: &Module, target: &mut Module, memory_offset: u32) -> Result<()> {
    let source_functions = source.funcs.iter().filter_map(|f| match &f.kind {
        FunctionKind::Local(local) => Some((f.id(), f.name.clone(), local)),
        _ => None,
    });

    let mut copied_functions = HashMap::new();
    let mut copied_functions_ids = HashSet::new();
    // move functions from
    for (id, name, function) in source_functions {
        let new_id = copy_function(source, target, function, name);
        // let new_id = a_funcs.add_local(b_fun.);
        copied_functions.insert(id, new_id);
        copied_functions_ids.insert(new_id);
    }

    target
        .funcs
        .par_iter_local_mut()
        .filter(|(id, _)| copied_functions_ids.contains(id))
        .for_each(|(_, f)| {
            dfs_pre_order_mut(
                &mut RefFixer {
                    func_map: &copied_functions,
                    memory_offset,
                },
                f,
                f.entry_block(),
            )
        });

    struct RefFixer<'a> {
        func_map: &'a HashMap<FunctionId, FunctionId>,
        memory_offset: u32,
    }

    impl<'a> VisitorMut for RefFixer<'a> {
        // fix function calls
        fn visit_call_mut(&mut self, instr: &mut walrus::ir::Call) {
            if let Some(new_id) = self.func_map.get(&instr.func) {
                instr.func = *new_id;
            }
        }
        fn visit_call_indirect_mut(&mut self, _instr: &mut walrus::ir::CallIndirect) {
            // TODO: how does this work?
        }

        // shift all memory accesses by the offset
        fn visit_instr_mut(&mut self, instr: &mut Instr, _: &mut walrus::InstrLocId) {
            match instr {
                Instr::Load(Load { arg, .. })
                | Instr::Store(Store { arg, .. })
                | Instr::AtomicRmw(AtomicRmw { arg, .. })
                | Instr::Cmpxchg(Cmpxchg { arg, .. })
                | Instr::AtomicNotify(AtomicNotify { arg, .. })
                | Instr::AtomicWait(AtomicWait { arg, .. })
                | Instr::LoadSimd(LoadSimd { arg, .. }) => {
                    arg.offset += self.memory_offset;
                }
                _ => {} // ignore all other instructions
            }
        }
    }

    // TODO: go over all *inserted* functions and change ids of call instructions

    Ok(())
}

fn copy_function(
    source: &Module,
    target: &mut Module,
    f: &LocalFunction,
    name: Option<String>,
) -> FunctionId {
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
    function_builder.finish(args, &mut target.funcs)
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
