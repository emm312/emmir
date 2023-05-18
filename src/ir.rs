use std::collections::HashMap;

use crate::values::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockID(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instruction {
    Add,
    Sub,
    Div,
    Mul,
    Mod,
    /// Pushes a value to the stack
    Push(Value),
    /// Pops from the stack, if the arg is Some(Value) it will pop n elements
    Pop(Option<Value>),
    Jump(BlockID)
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Block {
    pub(crate) instrs: Vec<Instruction>,
}

pub struct Program {
    pub(crate) blocks: HashMap<BlockID, Block>,
}