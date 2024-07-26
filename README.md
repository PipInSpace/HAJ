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

# Usage

This is a simple example program using HAJ:

```rust
use haj::*;

haj_cl_raw_c!("
#define TYPE_BO 0x00
#define TYPE_SU 0x00
#define TYPE_S 0x00
#define TYPE_G 0x00
#define def_velocity_set 19
#define def_c 1.3f
#define def_N 32
");

#[cl_fn]
fn is_halo(n: u32) -> bool { return false; }
#[cl_fn]
fn neighbors(n: u32, j: &mut [u32; def_velocity_set]) { }
#[cl_fn]
fn load_f(n: u32, fhn: &mut [f32; def_velocity_set], fi: &Vec<f32>, j: &[u32; def_velocity_set], t: u64) { }
#[cl_fn]
fn calculate_rho_u(fhn: &[f32; def_velocity_set], rhon: &mut f32, uxn: &mut f32, uyn: &mut f32, uzn: &mut f32) {}

#[cl_kernel]
fn update_fields(fi: &Vec<f32>, rho: &mut Vec<f32>, u: &mut Vec<f32>, flags: &Vec<u8>, t: u64, fx: f32, fy: f32, fz: f32) {
    let n: u32 = get_global_id(0); // n = x+(y+z*Ny)*Nx
    if n>=def_N as u32 || is_halo(n) { return; } // don't execute update_fields() on halo
    let flagsn: u8 = flags[n as usize];
    let flagsn_bo: u8=flagsn&TYPE_BO;
    let flagsn_su: u8=flagsn&TYPE_SU; // extract boundary and surface flags
    if flagsn_bo==TYPE_S || flagsn_su==TYPE_G { return; } // don't update fields for boundary or gas lattice points

    let mut j: [u32; def_velocity_set] = [0; def_velocity_set]; // neighbor indices
    neighbors(n, &mut j); // calculate neighbor indices
    let mut fhn: [f32; def_velocity_set] = [0.0; def_velocity_set]; // local DDFs
    load_f(n, &mut fhn, &fi, &j, t); // perform streaming (part 2)

    // calculate local density and velocity for collision
    let mut rhon: f32 = 0.0;
    let mut uxn: f32 = 0.0;
    let mut uyn: f32 = 0.0;
    let mut uzn: f32 = 0.0;
    let mut testificat: u32 = 0;
    calculate_rho_u(&fhn, &mut rhon, &mut uxn, &mut uyn, &mut uzn); // calculate density and velocity fields from fi
    {
        uxn = uxn.clamp(-def_c, def_c); // limit velocity (for stability purposes)
        uyn = uyn.clamp(-def_c, def_c); // force term: F*dt/(2*rho)
        uzn = uzn.clamp(-def_c, def_c);
    }

    rho[      n as usize] = rhon; // update density field
    u[        n as usize] = uxn; // update velocity field
    u[  def_N+n as usize] = uyn;
    u[2*def_N+n as usize] = uzn;
}

haj_init!();

fn main() {
    {
        let program = HAJ_OCL_PROGRAM.lock().unwrap();
        let program_src= (*program).to_string();
        println!("{}", program_src); // Prints the compiled program bytes
    }
    
    println!("{}", CL_SOURCE); // Prints the OpenCL C source code generated from Rust functions

    let mut kernels = HAJ_OCL_KERNELS.lock().unwrap(); // This will fail. Kernel arguments are not set correctly at the moment.
    let kernel = (*kernels).get_mut("update_fields").expect("kernel");
    unsafe {
        kernel.enq().unwrap()
    }
}
```

The annotated Rust functions are translated into OpenCL C and compiled via OpenCL.