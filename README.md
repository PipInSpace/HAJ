<h1> HAJ Accelerated Jobs <img align="right" src="https://github.com/user-attachments/assets/ff9a905f-ffe7-4c44-90e3-bdab747b1889" width="176px" float="left"></h1>

HAJ (HAJ Accelerated Jobs) allows for single source OpenCL programming in Rust. Simply annotate Rust
functions as `#[cl_fn]` or `#[cl_kernel]` and they are automatically converted into C and made runnable
over OpenCL.

HAJ orients itself on the programming model of "that proprietary computing platform by this green company"
to make writing highly parallel software as convenient as possible. Rust to C translation is provided
by the crate `rs2c`. Currently only simple Rust language features are supported and all types need to
be annotated. These limitations may be lifted over time.

To use this crate, clone this repository and the `rs2c` repository. Add the `haj` crate as a dependency
for your project by specifiying the path to the crate. A crates.io release will follow in time

# Usage

This is a simple example program using HAJ:

```rust
use haj::*;

cl_raw!("
#define TYPE_BO 0x00
#define TYPE_SU 0x00
#define TYPE_S 0x00
#define TYPE_G 0x00
#define def_velocity_set 19u
#define def_c 1.3f
#define def_N 32u
");

#[cl_fn] // Include this as a basic OpenCL function for use in different OpenCL code
fn is_halo(n: u32) -> bool { return false; }
#[cl_fn]
fn neighbors(n: u32, j: &mut [u32; def_velocity_set]) { }
#[cl_fn]
fn load_f(n: u32, fhn: &mut [f32; def_velocity_set], fi: &Vec<f32>, j: &[u32; def_velocity_set], t: u64) { }
#[cl_fn]
fn calculate_rho_u(fhn: &[f32; def_velocity_set], rhon: &mut f32, uxn: &mut f32, uyn: &mut f32, uzn: &mut f32) {}

#[cl_kernel] // Include this as an OpenCL kernel function to be called in parallel
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

// The HAJ initialisation macro builds a global OpenCL context.
// In source code it needs to be called below all the included functions.   
haj_init!(); 

fn main() {
    {
        let program = HAJ_OCL_PROGRAM.lock().unwrap();
        let program_src= (*program).to_string();
        println!("{}", program_src); // Prints information about the program, including OpenCL source code and compiled bytes
    }

    // Accessing kernels will be automated over macros
    let mut kernels = HAJ_OCL_KERNELS.lock().unwrap();
    let kernel = (*kernels).get_mut("update_fields").expect("kernel");
    println!("Running kernel");
    unsafe {
        kernel.enq().unwrap()
    }
    println!("Success!");
}
```

The annotated Rust functions are translated into OpenCL C and compiled via OpenCL. This is the OpenCL C source code output:

```c
#define TYPE_BO 0x00
#define TYPE_SU 0x00
#define TYPE_S 0x00
#define TYPE_G 0x00
#define def_velocity_set 19u
#define def_c 1.3f
#define def_N 32u

bool is_halo(const unsigned int n) {
return false;
}
void neighbors(const unsigned int n, unsigned int* j) {
}
void load_f(const unsigned int n, float* fhn, const float* fi, const unsigned int* j, const unsigned long t) {
}
void calculate_rho_u(const float* fhn, float rhon, float uxn, float uyn, float uzn) {
}
__kernel void update_fields(const global float* fi, global float* rho, global float* u, const global unsigned char* flags, const unsigned long t, const float fx, const float fy, const float fz) {
const unsigned int n = get_global_id(0);
if (n >= (unsigned int)def_N || is_halo(n)) {
return;
}
const unsigned char flagsn = flags[(unsigned long)n];
const unsigned char flagsn_bo = flagsn & TYPE_BO;
const unsigned char flagsn_su = flagsn & TYPE_SU;
if (flagsn_bo == TYPE_S || flagsn_su == TYPE_G) {
return;
}
unsigned int j[def_velocity_set] = {0};
neighbors(n, j);
float fhn[def_velocity_set] = {0.0};
load_f(n, fhn, fi, j, t);
float rhon = 0.0;
float uxn = 0.0;
float uyn = 0.0;
float uzn = 0.0;
unsigned int testificat = 0;
calculate_rho_u(fhn, rhon, uxn, uyn, uzn);
{
uxn = clamp(uxn, -def_c, def_c);
uyn = clamp(uyn, -def_c, def_c);
uzn = clamp(uzn, -def_c, def_c);
}
rho[(unsigned long)n] = rhon;
u[(unsigned long)n] = uxn;
u[def_N + (unsigned long)n] = uyn;
u[2 * def_N + (unsigned long)n] = uzn;
}
```

## Vector Types

OpenCL has support for Vector Types of lenghts 2, 3, 4, 8 and 16 for the types char, short, int and
long (both signed and unsigned) as well as half, float and double. To use them in your Rust code,
do the following:
```rust
let vector: FloatN<3> = [0.0f32, 1.0f32, 0.0f32].into(); // Defines a new OpenCL float Vector of lenght 3
```
Do not use the Struct itself for initialisation, this will not be translated correctly!