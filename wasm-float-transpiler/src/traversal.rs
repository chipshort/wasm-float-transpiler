use std::fmt::Debug;

use walrus::{
    ir::{Block, IfElse, Instr, InstrSeq, InstrSeqId, Loop},
    LocalFunction,
};

/// Variation of `walrus::ir::dfs_in_order` with slightly different interface
pub fn visit_instructions(
    mut visitor: impl FnMut(TraversalInstr),
    func: &LocalFunction,
    start: InstrSeqId,
) {
    // The stack of instruction sequences we still need to visit, and how far
    // along in the instruction sequence we are.
    let func_seq = func.block(start);
    let mut stack: Vec<(InstrSeqId, usize, TraversalInstr)> =
        vec![(start, 0, TraversalInstr::FunctionEnd(func_seq))];

    // visit function start instruction
    visitor(TraversalInstr::FunctionStart(func_seq));

    'traversing_blocks: while let Some((seq_id, index, end)) = stack.pop() {
        let seq = func.block(seq_id);

        for (index, (instr, _)) in seq.instrs.iter().enumerate().skip(index) {
            match instr {
                // Pause iteration through this sequence's instructions and
                // enqueue `seq` to be traversed next before continuing with
                // this one where we left off.
                Instr::Block(b) => {
                    let seq = func.block(b.seq);
                    stack.push((seq_id, index + 1, end.clone()));
                    stack.push((b.seq, 0, TraversalInstr::BlockEnd(seq, b)));
                    // visit the start instruction
                    visitor(TraversalInstr::BlockStart(seq, b));
                    continue 'traversing_blocks;
                }
                Instr::Loop(l) => {
                    let seq = func.block(l.seq);
                    stack.push((seq_id, index + 1, end.clone()));
                    stack.push((l.seq, 0, TraversalInstr::LoopEnd(seq, l)));
                    // visit the start instruction
                    visitor(TraversalInstr::LoopStart(seq, l));
                    // then continue with the instruction sequence
                    continue 'traversing_blocks;
                }
                // Pause iteration through this sequence's instructions.
                // Traverse the consequent and then the alternative.
                Instr::IfElse(ie) => {
                    let cons = func.block(ie.consequent);
                    let alt = func.block(ie.alternative);
                    stack.push((seq_id, index + 1, end.clone()));
                    stack.push((ie.alternative, 0, TraversalInstr::IfEnd(cons, alt, ie)));
                    stack.push((ie.consequent, 0, TraversalInstr::ElseStart(alt, ie)));
                    // visit the start instruction
                    visitor(TraversalInstr::IfStart(cons, ie));
                    // then continue with the instruction sequence
                    continue 'traversing_blocks;
                }
                instr => {
                    // other instructions don't have any children,
                    // so just visit them and continue with inner loop
                    visitor(TraversalInstr::Instr(instr));
                }
            }
        }

        // If we made it through the whole loop above, then we processed every
        // instruction in the sequence, and its nested sequences, so we are
        // finished with it!
        visitor(end);
    }
}

#[derive(Clone)]
pub enum TraversalInstr<'instr> {
    FunctionStart(&'instr InstrSeq),
    FunctionEnd(&'instr InstrSeq),
    BlockStart(&'instr InstrSeq, &'instr Block),
    BlockEnd(&'instr InstrSeq, &'instr Block),
    LoopStart(&'instr InstrSeq, &'instr Loop),
    LoopEnd(&'instr InstrSeq, &'instr Loop),
    IfStart(&'instr InstrSeq, &'instr IfElse),
    ElseStart(&'instr InstrSeq, &'instr IfElse),
    IfEnd(&'instr InstrSeq, &'instr InstrSeq, &'instr IfElse),
    /// Called for all instructions, except for those that are covered by the other variants.
    Instr(&'instr Instr),
}

impl<'instr> Debug for TraversalInstr<'instr> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FunctionStart(_) => f.write_str("FunctionStart"),
            Self::FunctionEnd(_) => f.write_str("FunctionEnd"),
            Self::BlockStart(_, _) => f.write_str("BlockStart"),
            Self::BlockEnd(_, _) => f.write_str("BlockEnd"),
            Self::LoopStart(_, _) => f.write_str("LoopStart"),
            Self::LoopEnd(_, _) => f.write_str("LoopEnd"),
            Self::IfStart(_, _) => f.write_str("IfStart"),
            Self::ElseStart(_, _) => f.write_str("ElseStart"),
            Self::IfEnd(_, _, _) => f.write_str("IfEnd"),
            Self::Instr(arg0) => f.debug_tuple("Instr").field(arg0).finish(),
        }
    }
}
