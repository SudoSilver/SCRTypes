use super::chunk_matrix::ChunkMatrix;

pub trait BlockAllocator<T> {
    fn drop_block(&mut self) -> bool;
    fn get_block(&self, target: usize) -> Option<(usize, usize)>;
}

impl<T: Default + Clone + std::iter::FromIterator<T>> 
BlockAllocator for ChunkMatrix {
    fn drop_block(&mut self) -> bool {
        let end = match self.index.pop() {
            Some(s) => s,
            None => return false,
        };
        let start = match self.index.pop() {
            Some(s) => s,
            None => return false,
        };

        self.data.truncate(start);
        return true;
    }
    fn get_block(&self, target: usize) -> Option<(usize,usize)> {
        if self.index.len() < 2 {
            return None;
        }
        // to get the starting index of a block 
        // we multiply the target block by 2 
        // to get the ending index of that block
        // then we subtract 1 to get an index to 
        // the starting index of the block
        let block_index = target * 2;
        
        // check if the block index possible on 
        // the index table confirming the block ending index
        // exists confirms that both indexes are safe
        // otherwise early return 
        if self.index.len() < block_index + 1 {
            return None; 
        } 

        let block_start = self.index[block_index];
        let block_end = self.index[block_index + 1];

        return Some((block_start, block_end));
    }
}
