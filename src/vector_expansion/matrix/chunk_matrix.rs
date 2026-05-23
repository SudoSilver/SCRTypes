pub struct ChunkMatrix<T> {
    data: Vec<T>,
    index: Vec<usize>,
}

impl<T: Default + Clone 
+ std::iter::FromIterator<T>> ChunkMatrix<T> {
    pub fn new() -> Self {
        return ChunkMatrix {
            data: Vec::new(),
            index: Vec::new(),
        };
    }

    pub fn push_block(&mut self, block: Vec<T>) {
        if block.is_empty() {
            return;
        }

        let current_max: usize = self.data.len();
        let starting_index: usize = current_max;
        let ending_index: usize = starting_index + block.len() - 1;
        
        for item in block {
            self.data.push(item);
        }

        self.index.push(starting_index);
        self.index.push(ending_index);
    }

    pub fn pop(&mut self) -> Option<Vec<T>> {
        let ending_index: usize = match self.index.pop() {
            Some(index) => index,
            None => return None,
        };

        let starting_index: usize = match self.index.pop() {
            Some(index) => index,
            None => return None,
        };

        let removed = self.data
            .drain(starting_index..=ending_index)
            .collect();

        return Some(removed);
    }
}
