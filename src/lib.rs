//
//   --- Define mylib: src\lib.rs ---
//

// 1. Define sublibraries/moduls
pub mod mathematics;
pub mod dates;
pub mod astrology;
pub mod astronomy;

// Publication
pub use crate::mathematics::prefix::*;
pub use crate::mathematics::utilities::*;

pub use crate::dates::core_dates::*;
pub use crate::astrology::core_astrology::*;
pub use crate::astronomy::core_astronomy::*;
