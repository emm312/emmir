use crate::ir::{BlockID, Program, Block};

struct Builder {
    current_block: Option<BlockID>,
    prog: Program,
    block_ctr: usize
}

impl Builder {
    pub fn push_block(&mut self) -> BlockID {
        self.block_ctr += 1;
        self.prog.blocks.insert(BlockID(self.block_ctr-1), Block { instrs: Vec::new() });
        BlockID(self.block_ctr)
    }
    pub fn set_block(&mut self, id: BlockID) {
        self.current_block = Some(id);
    }
    pub fn push_add(&mut self) {
        
    }
}