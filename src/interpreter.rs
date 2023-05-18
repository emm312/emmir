use std::collections::HashMap;

use crate::{ir::{Program, BlockID, Block, Instruction}, values::Value};

struct Stack(Vec<Value>);

impl Stack {
    pub fn pop(&mut self) -> Value {
        self.0.pop().expect("Popped value off the stack when it was empty")
    }
    pub fn push(&mut self, val: Value) {
        self.0.push(val);
    }
}

pub struct Runner {
    prog: Program,
    current_block: BlockID,
    stack: Stack
}

impl Runner {
    fn run_block(&mut self, blocks: &HashMap<BlockID, Block>) {
        let instrs = &blocks[&self.current_block].instrs;
        for instr in instrs {
            match instr {
                Instruction::Add => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(Value::Int(a.unwrap_int()+b.unwrap_int()));
                }
                Instruction::Push(n) => {
                    self.stack.push(n.clone());
                }
                _ => todo!()
            }
        }
    }
}