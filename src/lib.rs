pub mod errors;
pub mod string_expansion;
pub mod vector_expansion;
pub mod boolean_expansion;
pub mod hashmap_expansion;
pub mod file_expansion;
pub mod traits;

pub use crate::errors::{ ParseErrors, FileErrors };
pub use crate::string_expansion::string_extension::ExtendedStrings;
pub use crate::string_expansion::smart_strings::SmartStrings;
pub use crate::vector_expansion::smart_vectors::SmartVectors;
pub use crate::boolean_expansion::boolean_extension::BoolExpansion;
pub use crate::hashmap_expansion::hashmap_extension::HashMapExpansion;
pub use crate::hashmap_expansion::hashmap_simplify::SimpleHashMap;
pub use crate::file_expansion::smart_files::read_lines;

pub use crate::vector_expansion::matrix::chunk_matrix::ChunkMatrix;
