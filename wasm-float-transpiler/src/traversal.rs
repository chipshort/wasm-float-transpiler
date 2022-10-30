use std::collections::VecDeque;

use walrus::{
    ir::{Block, IfElse, Instr, InstrSeq, InstrSeqId, Loop},
    LocalFunction,
};

pub fn visit_instructions<'a, 'instr: 'a>(
    mut visitor: impl FnMut(TraversalInstr),
    func: &'instr LocalFunction,
    start: InstrSeqId,
) {
    // the queue of instructions that need to be visited
    let mut queue: VecDeque<TraversalInstr> = VecDeque::new();

    // add initial instructions to the stack
    let seq = func.block(start);
    queue.push_back(TraversalInstr::FunctionStart(seq));
    append_instrs(&mut queue, seq);
    queue.push_back(TraversalInstr::FunctionEnd(seq));

    // go through instruction queue
    while let Some(instr) = queue.pop_front() {
        match instr {
            TraversalInstr::Instr(Instr::Block(b)) => {
                let seq = func.block(b.seq);
                queue.push_back(TraversalInstr::BlockStart(seq, b));
                append_instrs(&mut queue, seq);
                queue.push_back(TraversalInstr::BlockEnd(seq, b));
            }
            TraversalInstr::Instr(Instr::Loop(l)) => {
                let seq = func.block(l.seq);
                queue.push_back(TraversalInstr::LoopStart(seq, l));
                append_instrs(&mut queue, seq);
                queue.push_back(TraversalInstr::LoopEnd(seq, l));
            }
            TraversalInstr::Instr(Instr::IfElse(ie)) => {
                let cons = func.block(ie.consequent);
                queue.push_back(TraversalInstr::IfStart(cons, ie));
                append_instrs(&mut queue, cons);
                let alt = func.block(ie.alternative);
                queue.push_back(TraversalInstr::ElseStart(alt, ie));
                append_instrs(&mut queue, alt);
                queue.push_back(TraversalInstr::IfEnd(cons, alt, ie));
            }
            instr => {
                visitor(instr); // other instructions don't have any children
            }
        }
    }
}

fn append_instrs<'instr>(queue: &mut VecDeque<TraversalInstr<'instr>>, seq: &'instr InstrSeq) {
    queue.extend(
        seq.instrs
            .iter()
            .map(|(instr, _)| TraversalInstr::Instr(instr)),
    );
}

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
    /// Called for all instructions, even those that are covered by the other variants.
    Instr(&'instr Instr),
}
