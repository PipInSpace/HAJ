# HAJ

HAJ (HAJ Accelerated Jobs) allows for single source OpenCL programming in Rust. Simply annotate Rust
functions as `#[cl_fn]` or `#[cl_kernel]` and they are automatically converted into C and made runnable
over OpenCL.

HAJ orients itself on the programming model of "that proprietary computing platform by this green company"
to make writing highly parallel software as convenient as possible. Rust to C translation is provided
by the crate `rs2c`. Currently only simple Rust language features are supported and all types need to
be annotated. These limitations may will decrease over time.

To use this crate, clone this repository and the rs2c repository. Add the `haj` crate as a dependency
for your project by specifiying the path to the crate.