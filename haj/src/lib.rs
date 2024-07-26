//! <h1> HAJ Accelerated Jobs <img align="right" src="https://github.com/user-attachments/assets/ff9a905f-ffe7-4c44-90e3-bdab747b1889" width="176px" float="left"></h1>
//! 
//! HAJ (HAJ Accelerated Jobs) allows for single source OpenCL programming in Rust. Simply annotate Rust
//! functions as `#[cl_fn]` or `#[cl_kernel]` and they are automatically converted into C and made runnable
//! over OpenCL.

pub use haj_macros::*;
pub use lazy_static;
pub use ocl;