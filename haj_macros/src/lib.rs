//! # HAJ Macros
//! 
//! `haj_macros` provides procedural macros required for the `haj` crate. All macros are reexported through `haj`.

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[cfg(test)]
mod tests;

// Used to collect source code from all annotated Rust functions
static mut CL_SOURCE: Vec<&'static str> = Vec::new();

#[proc_macro_attribute]
/// Mark a Rust function to be included in the OpenCL program source
pub fn cl_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let cl_source = rs2c::rs_to_c_function(&item.to_string());

    unsafe {
        CL_SOURCE.push(Box::leak(cl_source.into_boxed_str()));
    }

    TokenStream::new()
}

#[proc_macro_attribute]
/// Mark a Rust function to be included as a kernel function in the OpenCL program source
pub fn cl_kernel(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let cl_source = rs2c::opencl::rs_to_cl_kernel(&item.to_string());
    //let name = cl_source.split_once('(').expect("some").0.rsplit_once(' ').expect("some").1.to_string();
    //let args: Vec<String> = cl_source.split_once('(').expect("some").0.rsplit_once(')').expect("some").1.split(", ").map(|s| s.to_string()).collect();

    unsafe {
        CL_SOURCE.push(Box::leak(cl_source.into_boxed_str()));
        //KERNEL_NAMES.push(Box::leak(CLKernel {name, args}.into()));
    }

    TokenStream::new()
}

#[proc_macro]
/// Initialize the HAJ context. Collect functions and assemble them into an OpenCL C source file.
/// Create global OpenCL context, device and build program. Initialize buffers and kernels.
pub fn haj_init(_input: TokenStream) -> TokenStream {
    // Collect all converted functions into a source string
    let cl_source = unsafe {
        CL_SOURCE.join("\n")
    };

    // Attempt compilation over OpenCL at build time. Useful for finding errors in programs or during
    // translation. The rust-analyzer hates this and often crashes here during code analysis. Disable
    // during coding.
    #[cfg(feature = "analyse_at_compilation")]
    match ocl::Device::list_all(ocl::Platform::default()) {
        Ok(devices) => {
            if devices.len() > 0 {
                match ocl::Context::builder()
                    .platform(ocl::Platform::default())
                    .devices(devices[0])
                    .build() {
                    Ok(ctx) => {
                        match ocl::Program::builder().devices(devices[0]).src(&cl_source).build(&ctx) {
                            Ok(_) => {/**/}
                            Err(err) => {
                                panic!("Error compiling OpenCL program:  {}", err)
                            }
                        }
                    }
                    Err(_) => {}
                }
            }
        },
        Err(_) => {}
    }

    // Provide globally accessible OpenCL context
    let expanded = quote! {
        const CL_SOURCE: &str = #cl_source;
        haj::lazy_static::lazy_static! {
            static ref HAJ_OCL_CTX: std::sync::Mutex<haj::ocl::Context> = 
                std::sync::Mutex::new(haj::ocl::Context::builder()
                    .platform(haj::ocl::Platform::default())
                    .devices(haj::ocl::Device::list_all(haj::ocl::Platform::default()).expect("Cannot find devices")[0])
                    .build()
                    .unwrap()
                );
        }
        haj::lazy_static::lazy_static! {
            static ref HAJ_OCL_DEVICE: std::sync::Mutex<haj::ocl::Device> = 
                std::sync::Mutex::new(haj::ocl::Device::list_all(haj::ocl::Platform::default()).expect("Cannot find devices")[0]);
        }
        haj::lazy_static::lazy_static! {
            static ref HAJ_OCL_QUEUE: std::sync::Mutex<haj::ocl::Queue> = 
                std::sync::Mutex::new(haj::ocl::Queue::new(&HAJ_OCL_CTX.lock().unwrap(), *HAJ_OCL_DEVICE.lock().unwrap(), None).unwrap());
        }
        haj::lazy_static::lazy_static! {
            static ref HAJ_OCL_PROGRAM: std::sync::Mutex<haj::ocl::Program> = 
                std::sync::Mutex::new(
                    match haj::ocl::Program::builder()
                    .devices(*HAJ_OCL_DEVICE.lock().unwrap())
                    .src(CL_SOURCE)
                    .build(&HAJ_OCL_CTX.lock().unwrap()) {
                        Ok(program) => program,
                        Err(err) => {
                            println!("Program build error: {}", err);
                            std::process::exit(0)
                        }
                    }
                );
        }
        haj::lazy_static::lazy_static! {
            static ref HAJ_OCL_BUFFERS: std::sync::Mutex<std::collections::HashMap<String, haj::BufferT>> = std::sync::Mutex::new({
                let mut buffers = std::collections::HashMap::new();
                let kernel_split: Vec<&str> = CL_SOURCE.split("__kernel void ").skip(1).collect();
                let queue = HAJ_OCL_QUEUE.lock().unwrap();

                for kernel in kernel_split {
                    let kernel_name = kernel.split_once('(').expect("some").0;
                    let args: Vec<&str> = kernel.split_once('(').expect("some").1.split_once(')').expect("some").0.split(", ").collect();

                    for arg in args {
                        if arg.contains('*') {
                            let arg_name = arg.rsplit_once(' ').expect("arg name should be seperated by spaces").1;
                            let unsigned = arg.contains("unsigned ");
                            let buffer: BufferT;
                            //println!("Creating buffer {}", arg_name);

                            if arg.contains("float") {
                                buffer = haj::BufferT::F32(haj::create_buffer(&queue, 1, 0.0f32));
                            } else if arg.contains("double") {
                                buffer = haj::BufferT::F64(haj::create_buffer(&queue, 1, 0.0f64));
                            } else if arg.contains("char") {
                                if unsigned {
                                    buffer = haj::BufferT::U8(haj::create_buffer(&queue, 1, 0u8));
                                } else {
                                    buffer = haj::BufferT::I8(haj::create_buffer(&queue, 1, 0i8));
                                }
                            } else if arg.contains("short") {
                                if unsigned {
                                    buffer = haj::BufferT::U16(haj::create_buffer(&queue, 1, 0u16));
                                } else {
                                    buffer = haj::BufferT::I16(haj::create_buffer(&queue, 1, 0i16));
                                }
                            } else if arg.contains("long") {
                                if unsigned {
                                    buffer = haj::BufferT::U64(haj::create_buffer(&queue, 1, 0u64));
                                } else {
                                    buffer = haj::BufferT::I64(haj::create_buffer(&queue, 1, 0i64));
                                }
                            } else {
                                if unsigned {
                                    buffer = haj::BufferT::U32(haj::create_buffer(&queue, 1, 0u32));
                                } else {
                                    buffer = haj::BufferT::I32(haj::create_buffer(&queue, 1, 0i32));
                                }
                            }
                            buffers.insert(format!("{}_{}", kernel_name, arg_name), buffer);
                        }
                    }
                }
        
                buffers
            });
        }
        haj::lazy_static::lazy_static! {
            static ref HAJ_OCL_KERNELS: std::sync::Mutex<std::collections::HashMap<String, haj::ocl::Kernel>> = std::sync::Mutex::new({
                let mut kernels = std::collections::HashMap::new();
                let kernel_split: Vec<&str> = CL_SOURCE.split("__kernel void ").skip(1).collect();
                let buffers = HAJ_OCL_BUFFERS.lock().unwrap();

                for kernel in kernel_split {
                    let kernel_name = kernel.split_once('(').expect("some").0;
                    let args: Vec<&str> = kernel.split_once('(').expect("some").1.split_once(')').expect("some").0.split(", ").collect();
                    
                    let mut kernel_builder = haj::ocl::Kernel::builder();
                    let program = HAJ_OCL_PROGRAM.lock().unwrap();
                    kernel_builder.program(&program)
                        .name(kernel_name)
                        .queue((*HAJ_OCL_QUEUE.lock().unwrap()).clone())
                        .global_work_size(1);

                    for arg in args {
                        //println!("Setting arg {}", arg);

                        if arg.contains('*') { // is a buffer
                            let arg_name = arg.rsplit_once(' ').expect("arg name should be seperated by spaces").1;
                            let buft = (*buffers).get(&format!("{}_{}", kernel_name, arg_name)).expect("buffert");
                            match buft {
                                BufferT::U8(b) => {kernel_builder.arg(b);},
                                BufferT::U16(b) => {kernel_builder.arg(b);},
                                BufferT::U32(b) => {kernel_builder.arg(b);},
                                BufferT::U64(b) => {kernel_builder.arg(b);},
                                BufferT::I8(b) => {kernel_builder.arg(b);},
                                BufferT::I16(b) => {kernel_builder.arg(b);},
                                BufferT::I32(b) => {kernel_builder.arg(b);},
                                BufferT::I64(b) => {kernel_builder.arg(b);},
                                BufferT::F32(b) => {kernel_builder.arg(b);},
                                BufferT::F64(b) => {kernel_builder.arg(b);},
                            }
                        } else {
                            if arg.contains("float") {
                                kernel_builder.arg(0.0f32);
                            } else if arg.contains("double") {
                                kernel_builder.arg(0.0f64);
                            } else if arg.contains("char") {
                                kernel_builder.arg(0u8);
                            } else if arg.contains("long") {
                                kernel_builder.arg(0u64);
                            } else {
                                kernel_builder.arg(0u32);
                            }
                        }
                        
                    }
                    let kernel = kernel_builder.build().unwrap();
                    kernels.insert(kernel_name.to_string(), kernel);
                }
        
                kernels
            });
        }
        
    };

    TokenStream::from(expanded)
}

#[proc_macro]
/// Include the following text within a string literal as raw OpenCL C source code. Useful for
/// C style functions and #define statements etc.
pub fn cl_raw(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::LitStr);
    let input_str = input.value();
    unsafe {
        CL_SOURCE.push(Box::leak(input_str.into_boxed_str()));
    }

    TokenStream::new()
}
