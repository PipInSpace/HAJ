//! <h1> HAJ Accelerated Jobs <img align="right" src="https://github.com/user-attachments/assets/ff9a905f-ffe7-4c44-90e3-bdab747b1889" width="176px" float="left"></h1>
//! 
//! HAJ (HAJ Accelerated Jobs) allows for single source OpenCL programming in Rust. Simply annotate Rust
//! functions as `#[cl_fn]` or `#[cl_kernel]` and they are automatically converted into C and made runnable
//! over OpenCL.

pub use haj_macros::*;
pub use lazy_static;
pub use ocl;

mod opencl_builtin;

pub use opencl_builtin::*;
pub use half::prelude::f16;

// Wrapper object for OpenCL Buffers
pub enum BufferT {
    U8(ocl::Buffer<u8>),
    U16(ocl::Buffer<u16>),
    U32(ocl::Buffer<u32>),
    U64(ocl::Buffer<u64>),
    I8(ocl::Buffer<i8>),
    I16(ocl::Buffer<i16>),
    I32(ocl::Buffer<i32>),
    I64(ocl::Buffer<i64>),
    F32(ocl::Buffer<f32>),
    F64(ocl::Buffer<f64>),
}

#[allow(unused)]
/// Creates a buffer of the specified size and flags. The type is inferred from the fill value.
pub fn create_buffer<T: ocl::OclPrm, I: Into<ocl::SpatialDims> + Clone>(
    queue: &ocl::Queue,
    size: I,
    fill_value: T,
) -> ocl::Buffer<T> {
    if (size.clone().into() as ocl::SpatialDims).to_len() >= 1 {
        return ocl::Buffer::<T>::builder()
            .queue(queue.clone())
            .len(size)
            .fill_val(fill_value)
            .flags(ocl::flags::MEM_READ_WRITE)
            .build()
            .unwrap();
    }
    // use size of 1 if invalid
    return ocl::Buffer::<T>::builder()
        .queue(queue.clone())
        .len([1])
        .fill_val(fill_value)
        .flags(ocl::flags::MEM_READ_WRITE)
        .build()
        .unwrap();
}

#[macro_export]
/// Launch a definded Kernel
macro_rules! haj_launch {
    ($kernel_name:expr=>$work_size:expr, $( $arg:expr),*) => { // Unnamed arguments
        let kernels = HAJ_OCL_KERNELS.lock().unwrap();
        let kernel = (*kernels).get($kernel_name).expect("kernel");
        let mut i = 0;
        $(
            kernel.set_arg(i, $arg).unwrap();
            i += 1;
        )*
        unsafe {
            kernel.cmd().global_work_size($work_size).enq().unwrap()
        }
    };
}