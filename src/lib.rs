//
//   --- Define mylib: src\lib.rs ---
//

// 1. Define sublibraries/moduls
pub mod mathematics {
    // 2. bind submodules
    pub mod prefix;
    pub mod utilities;
}

// Publication
pub use crate::mathematics::utilities::*;
pub use crate::mathematics::prefix::*;
