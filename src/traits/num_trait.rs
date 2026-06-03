use std::ops::{Add, Sub, Mul, Div};

pub trait Number: 
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Copy
{}

impl<T> Number for T
where T:
        Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Copy
{}
