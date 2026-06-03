#[derive(Debug, PartialEq)]
pub enum ParseErrors {
    NotAValidInt,
    NotAValidFloat,
    NotAValidBool,
}

#[derive(Debug, PartialEq)]
pub enum FileErrors {
    FileNotExists,
    UnableToReadFromFile,
}

#[derive(Debug, PartialEq)]
pub enum MathErrors {
    NotAValidMatrix,
    NotAValidMatrixRow,
}
