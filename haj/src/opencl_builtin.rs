//! OpenCL C defines a multitude of built-in functions. These are provided here without functionality
//! to allow their usage in annotated Rust code without warnings. 
//! ## WARNING: These are NOT functional
#![allow(dead_code, unused_variables)]

use half::prelude::*;

use num_traits::float::Float;

// Work-Item Functions

/// Returns the number of dimensions in use. This is the value given to the work_dim argument specified in clEnqueueNDRangeKernel.
fn get_work_dim() -> u32 {0}
/// Returns the number of global work-items specified for dimension identified by dimindx. This value is given by the global_work_size argument to clEnqueueNDRangeKernel.
/// 
/// Valid values of dimindx are 0 to get_work_dim() - 1. For other values of dimindx, get_global_size() returns 1.
fn get_global_size(dimindx: u32) -> u32 {0}
/// Returns the unique global work-item ID value for dimension identified by dimindx. The global work-item ID specifies the work-item ID based on the number of global work-items specified to execute the kernel.
/// 
/// Valid values of dimindx are 0 to get_work_dim() - 1. For other values of dimindx, get_global_id() returns 0.
fn get_global_id(dimindx: u32) -> u32 {0}
/// Returns the number of local work-items specified in dimension identified by dimindx. This value is at most the value given by the local_work_size argument to clEnqueueNDRangeKernel if local_work_size is not NULL; otherwise the OpenCL implementation chooses an appropriate local_work_size value which is returned by this function. If the kernel is executed with a non-uniform work-group size [38], calls to this built-in from some work-groups may return different values than calls to this built-in from other work-groups.
/// 
/// Valid values of dimindx are 0 to get_work_dim() - 1. For other values of dimindx, get_local_size() returns 1.
fn get_local_size(dimindx: u32) -> u32 {0}
/// Returns the same value as that returned by get_local_size(dimindx) if the kernel is executed with a uniform work-group size.
/// 
/// If the kernel is executed with a non-uniform work-group size, returns the number of local work-items in each of the work-groups that make up the uniform region of the global range in the dimension identified by dimindx. If the local_work_size argument to clEnqueueNDRangeKernel is not NULL, this value will match the value specified in local_work_size\[dimindx\]. If local_work_size is NULL, this value will match the local size that the implementation determined would be most efficient at implementing the uniform region of the global range.
/// 
/// Valid values of dimindx are 0 to get_work_dim() - 1. For other values of dimindx, get_enqueued_local_size() returns 1.
/// 
/// Requires support for OpenCL 2.0 or newer.
fn get_enqueued_local_size(dimindx: u32) -> u32 {0}
/// Returns the unique local work-item ID, i.e. a work-item within a specific work-group for dimension identified by dimindx.
/// 
/// Valid values of dimindx are 0 to get_work_dim() - 1. For other values of dimindx, get_local_id() returns 0.
fn get_local_id(dimindx: u32) -> u32 {0}
/// Returns the number of work-groups that will execute a kernel for dimension identified by dimindx.
/// 
/// Valid values of dimindx are 0 to get_work_dim() - 1. For other values of dimindx, get_num_groups() returns 1.
fn get_num_groups(dimindx: u32) -> u32 {0}
/// get_group_id returns the work-group ID which is a number from 0 .. get_num_groups(dimindx) - 1.
/// 
/// Valid values of dimindx are 0 to get_work_dim() - 1. For other values, get_group_id() returns 0.
fn get_group_id(dimindx: u32) -> u32 {0}
/// get_global_offset returns the offset values specified in global_work_offset argument to clEnqueueNDRangeKernel.
/// 
/// Valid values of dimindx are 0 to get_work_dim() - 1. For other values, get_global_offset() returns 0.
/// 
/// Requires support for OpenCL C 1.1 or newer.
fn get_global_offset(dimindx: u32) -> u32 {0}
/// Returns the work-items 1-dimensional global ID.
/// 
/// For 1D work-groups, it is computed as get_global_id(0) - get_global_offset(0).
/// 
/// For 2D work-groups, it is computed as (get_global_id(1) - get_global_offset(1)) * get_global_size(0) + (get_global_id(0) - get_global_offset(0)).
/// 
/// For 3D work-groups, it is computed as ((get_global_id(2) - get_global_offset(2)) * get_global_size(1) * get_global_size(0)) + ((get_global_id(1) - get_global_offset(1)) * get_global_size(0)) + (get_global_id(0) - get_global_offset(0)).
/// 
/// Requires support for OpenCL 2.0 or newer.
fn get_global_linear_id(dimindx: u32) -> u32 {0}
/// Returns the work-items 1-dimensional local ID.
/// 
/// For 1D work-groups, it is the same value as
/// 
/// get_local_id(0).
/// 
/// For 2D work-groups, it is computed as
/// 
/// get_local_id(1) * get_local_size(0) + get_local_id(0).
/// 
/// For 3D work-groups, it is computed as
/// 
/// (get_local_id(2) * get_local_size(1) * get_local_size(0)) + (get_local_id(1) * get_local_size(0)) + get_local_id(0).
/// 
/// Requires support for OpenCL 2.0 or newer.
fn get_local_linear_id(dimindx: u32) -> u32 {0}
// The functionality described in the following functions requires support for the cl_khr_subgroups extension macro; or for OpenCL C 3.0 or newer and the __opencl_c_subgroups feature.

/// Returns the number of work-items in the sub-group. This value is no more than the maximum sub-group size and is implementation-defined based on a combination of the compiled kernel and the dispatch dimensions. This will be a constant value for the lifetime of the sub-group.
fn get_sub_group_size() -> u32 {0}
/// Returns the maximum size of a sub-group within the dispatch. This value will be invariant for a given set of dispatch dimensions and a kernel object compiled for a given device.
fn get_max_sub_group_size() -> u32 {0}
/// Returns the number of sub-groups that the current work-group is divided into.
/// 
/// This number will be constant for the duration of a work-group’s execution. If the kernel is executed with a non-uniform work-group size (i.e. the global_work_size values specified to clEnqueueNDRangeKernel are not evenly divisible by the local_work_size values for any dimension, calls to this built-in from some work-groups may return different values than calls to this built-in from other work-groups.
fn get_num_sub_groups() -> u32 {0}
/// Returns the same value as that returned by get_num_sub_groups if the kernel is executed with a uniform work-group size.
/// 
/// If the kernel is executed with a non-uniform work-group size, returns the number of sub-groups in each of the work-groups that make up the uniform region of the global range.
fn get_enqueued_num_sub_groups() -> u32 {0}
/// get_sub_group_id returns the sub-group ID which is a number from 0 .. get_num_sub_groups() - 1.
/// 
/// For clEnqueueTask, this returns 0.
fn get_sub_group_id() -> u32 {0}
/// Returns the unique work-item ID within the current sub-group. The mapping from get_local_id(dimindx) to get_sub_group_local_id will be invariant for the lifetime of the work-group.
fn get_sub_group_local_id() -> u32 {0}

// Math Functions

fn acos<T: Float>(x: T) -> T {x}
fn acosh<T: Float>(x: T) -> T {x}
fn acospi<T: Float>(x: T) -> T {x}
fn asin<T: Float>(x: T) -> T {x}
fn asinh<T: Float>(x: T) -> T {x}
fn asinpi<T: Float>(x: T) -> T {x}
fn atan<T: Float>(y_over_x: T) -> T {y_over_x}
fn atan2<T: Float>(x: T, y: T) -> T {x}
fn atanh<T: Float>(x: T) -> T {x}
fn atanpi<T: Float>(x: T) -> T {x}
fn atan2pi<T: Float>(x: T, y: T) -> T {x}
fn cbrt<T: Float>(x: T) -> T {x}
fn ceil<T: Float>(x: T) -> T {x}
fn copy_sign<T: Float>(x: T, y: T) -> T {x}
fn cos<T: Float>(x: T) -> T {x}
fn cosh<T: Float>(x: T) -> T {x}
fn cospi<T: Float>(x: T) -> T {x}
fn erfc<T: Float>(x: T) -> T {x}
fn erf<T: Float>(x: T) -> T {x}
fn exp<T: Float>(x: T) -> T {x}
fn exp2<T: Float>(x: T) -> T {x}
fn exp10<T: Float>(x: T) -> T {x}
fn expm1<T: Float>(x: T) -> T {x}
fn fabs<T: Float>(x: T) -> T {x}
fn fdim<T: Float>(x: T) -> T {x}
fn floor<T: Float>(x: T) -> T {x}
fn fma<T: Float>(a: T, b: T, c: T) -> T {a}
fn fmax<T: Float>(x: T, y: T) -> T {x}
fn fmin<T: Float>(x: T, y: T) -> T {x}
fn fmod<T: Float>(x: T, y: T) -> T {x}
fn fract<T: Float>(x: T, y: T) -> T {x}
trait Frexp<S> {fn frexp(x: Self, exp: S) -> Self;}
impl<const N: usize> Frexp<IntN<N>> for HalfN<N> {fn frexp(x: Self, y: IntN<N>) -> Self {x}}
impl Frexp<i32> for f16 {fn frexp(x: Self, y: i32) -> Self {x}}
impl<const N: usize> Frexp<IntN<N>> for FloatN<N> {fn frexp(x: Self, y: IntN<N>) -> Self {x}}
impl Frexp<i32> for f32 {fn frexp(x: Self, y: i32) -> Self {x}}
impl<const N: usize> Frexp<IntN<N>> for DoubleN<N> {fn frexp(x: Self, y: IntN<N>) -> Self {x}}
impl Frexp<i32> for f64 {fn frexp(x: Self, y: i32) -> Self {x}}
fn hypot<T: Float>(x: T, y: T) -> T {x}
trait Ilogb<S> {fn ilogb(x: S) -> Self;}
impl<const N: usize> Ilogb<HalfN<N>> for IntN<N> {fn ilogb(x: HalfN<N>) -> Self {IntN { coordinates: [0; N] }}}
impl Ilogb<f16> for i32 {fn ilogb(x: f16) -> Self {0}}
impl<const N: usize> Ilogb<FloatN<N>> for IntN<N> {fn ilogb(x: FloatN<N>) -> Self {IntN { coordinates: [0; N] }}}
impl Ilogb<f32> for i32 {fn ilogb(x: f32) -> Self {0}}
impl<const N: usize> Ilogb<DoubleN<N>> for IntN<N> {fn ilogb(x: DoubleN<N>) -> Self {IntN { coordinates: [0; N] }}}
impl Ilogb<f64> for i32 {fn ilogb(x: f64) -> Self {0}}
trait Ldexp<S> {fn ldexp(x: Self, exp: S) -> Self;}
impl<const N: usize> Ldexp<IntN<N>> for HalfN<N> {fn ldexp(x: Self, y: IntN<N>) -> Self {x}}
impl<const N: usize> Ldexp<i32> for HalfN<N> {fn ldexp(x: Self, y: i32) -> Self {x}}
impl Ldexp<i32> for f16 {fn ldexp(x: Self, y: i32) -> Self {x}}
impl<const N: usize> Ldexp<IntN<N>> for FloatN<N> {fn ldexp(x: Self, y: IntN<N>) -> Self {x}}
impl<const N: usize> Ldexp<i32> for FloatN<N> {fn ldexp(x: Self, y: i32) -> Self {x}}
impl Ldexp<i32> for f32 {fn ldexp(x: Self, y: i32) -> Self {x}}
impl<const N: usize> Ldexp<IntN<N>> for DoubleN<N> {fn ldexp(x: Self, y: IntN<N>) -> Self {x}}
impl<const N: usize> Ldexp<i32> for DoubleN<N> {fn ldexp(x: Self, y: i32) -> Self {x}}
impl Ldexp<i32> for f64 {fn ldexp(x: Self, y: i32) -> Self {x}}
fn lgamma<T: Float>(x: T) -> T {x}
trait LgammaR<S> {fn lgama_r(x: Self, exp: S) -> Self;}
impl<const N: usize> LgammaR<IntN<N>> for HalfN<N> {fn lgama_r(x: Self, y: IntN<N>) -> Self {x}}
impl LgammaR<i32> for f16 {fn lgama_r(x: Self, y: i32) -> Self {x}}
impl<const N: usize> LgammaR<IntN<N>> for FloatN<N> {fn lgama_r(x: Self, y: IntN<N>) -> Self {x}}
impl LgammaR<i32> for f32 {fn lgama_r(x: Self, y: i32) -> Self {x}}
impl<const N: usize> LgammaR<IntN<N>> for DoubleN<N> {fn lgama_r(x: Self, y: IntN<N>) -> Self {x}}
impl LgammaR<i32> for f64 {fn lgama_r(x: Self, y: i32) -> Self {x}}
fn log<T: Float>(x: T) -> T {x}
fn log2<T: Float>(x: T) -> T {x}
fn log10<T: Float>(x: T) -> T {x}
fn log1p<T: Float>(x: T) -> T {x}
fn mad<T: Float>(a: T, b: T, c: T) -> T {a}
fn maxmag<T: Float>(x: T, y: T) -> T {x}
fn minmag<T: Float>(x: T, y: T) -> T {x}
fn modf<T: Float>(x: T, iptr: T) -> T {x}


// Vector Data Types
struct CharN<const N: usize> {coordinates: [i8; N]}
impl From<[i8; 2]> for CharN<2>  {fn from(arr: [i8; 2]) -> Self {CharN { coordinates: arr }}}
impl From<[i8; 3]> for CharN<3>  {fn from(arr: [i8; 3]) -> Self {CharN { coordinates: arr }}}
impl From<[i8; 4]> for CharN<4>  {fn from(arr: [i8; 4]) -> Self {CharN { coordinates: arr }}}
impl From<[i8; 8]> for CharN<8>  {fn from(arr: [i8; 8]) -> Self {CharN { coordinates: arr }}}
impl From<[i8; 16]> for CharN<16>  {fn from(arr: [i8; 16]) -> Self {CharN { coordinates: arr }}}
struct UcharN<const N: usize> {coordinates: [u8; N]}
impl From<[u8; 2]> for UcharN<2>  {fn from(arr: [u8; 2]) -> Self {UcharN { coordinates: arr }}}
impl From<[u8; 3]> for UcharN<3>  {fn from(arr: [u8; 3]) -> Self {UcharN { coordinates: arr }}}
impl From<[u8; 4]> for UcharN<4>  {fn from(arr: [u8; 4]) -> Self {UcharN { coordinates: arr }}}
impl From<[u8; 8]> for UcharN<8>  {fn from(arr: [u8; 8]) -> Self {UcharN { coordinates: arr }}}
impl From<[u8; 16]> for UcharN<16>  {fn from(arr: [u8; 16]) -> Self {UcharN { coordinates: arr }}}
struct ShortN<const N: usize> {coordinates: [i16; N]}
impl From<[i16; 2]> for ShortN<2>  {fn from(arr: [i16; 2]) -> Self {ShortN { coordinates: arr }}}
impl From<[i16; 3]> for ShortN<3>  {fn from(arr: [i16; 3]) -> Self {ShortN { coordinates: arr }}}
impl From<[i16; 4]> for ShortN<4>  {fn from(arr: [i16; 4]) -> Self {ShortN { coordinates: arr }}}
impl From<[i16; 8]> for ShortN<8>  {fn from(arr: [i16; 8]) -> Self {ShortN { coordinates: arr }}}
impl From<[i16; 16]> for ShortN<16>  {fn from(arr: [i16; 16]) -> Self {ShortN { coordinates: arr }}}
struct UshortN<const N: usize> {coordinates: [u16; N]}
impl From<[u16; 2]> for UshortN<2>  {fn from(arr: [u16; 2]) -> Self {UshortN { coordinates: arr }}}
impl From<[u16; 3]> for UshortN<3>  {fn from(arr: [u16; 3]) -> Self {UshortN { coordinates: arr }}}
impl From<[u16; 4]> for UshortN<4>  {fn from(arr: [u16; 4]) -> Self {UshortN { coordinates: arr }}}
impl From<[u16; 8]> for UshortN<8>  {fn from(arr: [u16; 8]) -> Self {UshortN { coordinates: arr }}}
impl From<[u16; 16]> for UshortN<16>  {fn from(arr: [u16; 16]) -> Self {UshortN { coordinates: arr }}}
struct IntN<const N: usize> {coordinates: [i32; N]}
impl From<[i32; 2]> for IntN<2>  {fn from(arr: [i32; 2]) -> Self {IntN { coordinates: arr }}}
impl From<[i32; 3]> for IntN<3>  {fn from(arr: [i32; 3]) -> Self {IntN { coordinates: arr }}}
impl From<[i32; 4]> for IntN<4>  {fn from(arr: [i32; 4]) -> Self {IntN { coordinates: arr }}}
impl From<[i32; 8]> for IntN<8>  {fn from(arr: [i32; 8]) -> Self {IntN { coordinates: arr }}}
impl From<[i32; 16]> for IntN<16>  {fn from(arr: [i32; 16]) -> Self {IntN { coordinates: arr }}}
struct UintN<const N: usize> {coordinates: [u32; N]}
impl From<[u32; 2]> for UintN<2>  {fn from(arr: [u32; 2]) -> Self {UintN { coordinates: arr }}}
impl From<[u32; 3]> for UintN<3>  {fn from(arr: [u32; 3]) -> Self {UintN { coordinates: arr }}}
impl From<[u32; 4]> for UintN<4>  {fn from(arr: [u32; 4]) -> Self {UintN { coordinates: arr }}}
impl From<[u32; 8]> for UintN<8>  {fn from(arr: [u32; 8]) -> Self {UintN { coordinates: arr }}}
impl From<[u32; 16]> for UintN<16>  {fn from(arr: [u32; 16]) -> Self {UintN { coordinates: arr }}}
struct LongN<const N: usize> {coordinates: [i64; N]}
impl From<[i64; 2]> for LongN<2>  {fn from(arr: [i64; 2]) -> Self {LongN { coordinates: arr }}}
impl From<[i64; 3]> for LongN<3>  {fn from(arr: [i64; 3]) -> Self {LongN { coordinates: arr }}}
impl From<[i64; 4]> for LongN<4>  {fn from(arr: [i64; 4]) -> Self {LongN { coordinates: arr }}}
impl From<[i64; 8]> for LongN<8>  {fn from(arr: [i64; 8]) -> Self {LongN { coordinates: arr }}}
impl From<[i64; 16]> for LongN<16>  {fn from(arr: [i64; 16]) -> Self {LongN { coordinates: arr }}}
struct UlongN<const N: usize> {coordinates: [u64; N]}
impl From<[u64; 2]> for UlongN<2>  {fn from(arr: [u64; 2]) -> Self {UlongN { coordinates: arr }}}
impl From<[u64; 3]> for UlongN<3>  {fn from(arr: [u64; 3]) -> Self {UlongN { coordinates: arr }}}
impl From<[u64; 4]> for UlongN<4>  {fn from(arr: [u64; 4]) -> Self {UlongN { coordinates: arr }}}
impl From<[u64; 8]> for UlongN<8>  {fn from(arr: [u64; 8]) -> Self {UlongN { coordinates: arr }}}
impl From<[u64; 16]> for UlongN<16>  {fn from(arr: [u64; 16]) -> Self {UlongN { coordinates: arr }}}
struct HalfN<const N: usize> {coordinates: [f16; N]}
impl From<[f16; 2]> for HalfN<2>  {fn from(arr: [f16; 2]) -> Self {HalfN { coordinates: arr }}}
impl From<[f16; 3]> for HalfN<3>  {fn from(arr: [f16; 3]) -> Self {HalfN { coordinates: arr }}}
impl From<[f16; 4]> for HalfN<4>  {fn from(arr: [f16; 4]) -> Self {HalfN { coordinates: arr }}}
impl From<[f16; 8]> for HalfN<8>  {fn from(arr: [f16; 8]) -> Self {HalfN { coordinates: arr }}}
impl From<[f16; 16]> for HalfN<16>  {fn from(arr: [f16; 16]) -> Self {HalfN { coordinates: arr }}}
struct FloatN<const N: usize> {coordinates: [f32; N]}
impl From<[f32; 2]> for FloatN<2>  {fn from(arr: [f32; 2]) -> Self {FloatN { coordinates: arr }}}
impl From<[f32; 3]> for FloatN<3>  {fn from(arr: [f32; 3]) -> Self {FloatN { coordinates: arr }}}
impl From<[f32; 4]> for FloatN<4>  {fn from(arr: [f32; 4]) -> Self {FloatN { coordinates: arr }}}
impl From<[f32; 8]> for FloatN<8>  {fn from(arr: [f32; 8]) -> Self {FloatN { coordinates: arr }}}
impl From<[f32; 16]> for FloatN<16>  {fn from(arr: [f32; 16]) -> Self {FloatN { coordinates: arr }}}
struct DoubleN<const N: usize> {coordinates: [f64; N]}
impl From<[f64; 2]> for DoubleN<2>  {fn from(arr: [f64; 2]) -> Self {DoubleN { coordinates: arr }}}
impl From<[f64; 3]> for DoubleN<3>  {fn from(arr: [f64; 3]) -> Self {DoubleN { coordinates: arr }}}
impl From<[f64; 4]> for DoubleN<4>  {fn from(arr: [f64; 4]) -> Self {DoubleN { coordinates: arr }}}
impl From<[f64; 8]> for DoubleN<8>  {fn from(arr: [f64; 8]) -> Self {DoubleN { coordinates: arr }}}
impl From<[f64; 16]> for DoubleN<16>  {fn from(arr: [f64; 16]) -> Self {DoubleN { coordinates: arr }}}
