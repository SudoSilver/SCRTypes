use crate::traits::Number;
use crate::errors::MathErrors;

pub struct MathMatrix<T> {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<T>,
}

impl<T: Number> MathMatrix<T> {
    
    pub fn new() -> Self {
        let matrix = Self {
            rows: 2 as usize,
            cols: 2 as usize,
            data: Vec::new(),
        };
        return matrix;
    }

    pub fn with_dimensions(rows: usize, cols: usize) -> Self {
         let matrix = Self {
            rows: rows,
            cols: cols,
            data: Vec::new(),
        };
        return matrix;
    }

    pub fn from(input: Vec<Vec<T>>) -> Result<Self, MathErrors> {
        let mut rows: usize = 0;
        let mut data: Vec<T> = Vec::new();
        let cols = input.first().map_or(0, |r| r.len());
        
        if cols < 1 {
            return Err(MathErrors::NotAValidMatrix);
        }

        for line in input {
            rows += 1;
            
            if line.len() != cols {
                return Err(MathErrors::NotAValidMatrix);
            }
            
            for item in line {
                data.push(item);
            }
        }

        let matrix = Self {
            rows,
            cols,
            data,
        };
        return Ok(matrix);
    }

    pub fn push(&mut self, data: Vec<T>) -> Result<(), MathErrors> {
        if data.len() != self.cols {
            return Err(MathErrors::NotAValidMatrixRow);
        }

        for item in data {
            self.data.push(item);
        }

        self.rows += 1;

        return Ok(());
    }
}
