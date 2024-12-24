#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod reducewc {
            #[allow(dead_code, clippy::all)]
            pub mod reduce {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_reduce_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let base4 = arg2;
                    let len4 = arg3;
                    let mut result4 = _rt::Vec::with_capacity(len4);
                    for i in 0..len4 {
                        let base = base4.add(i * 8);
                        let e4 = {
                            let l1 = *base.add(0).cast::<*mut u8>();
                            let l2 = *base.add(4).cast::<usize>();
                            let len3 = l2;
                            let bytes3 = _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                            _rt::string_lift(bytes3)
                        };
                        result4.push(e4);
                    }
                    _rt::cabi_dealloc(base4, len4 * 8, 4);
                    let result5 = T::reduce(_rt::string_lift(bytes0), result4);
                    let ptr6 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let (t7_0, t7_1) = result5;
                    let vec8 = (t7_0.into_bytes()).into_boxed_slice();
                    let ptr8 = vec8.as_ptr().cast::<u8>();
                    let len8 = vec8.len();
                    ::core::mem::forget(vec8);
                    *ptr6.add(4).cast::<usize>() = len8;
                    *ptr6.add(0).cast::<*mut u8>() = ptr8.cast_mut();
                    let vec9 = (t7_1.into_bytes()).into_boxed_slice();
                    let ptr9 = vec9.as_ptr().cast::<u8>();
                    let len9 = vec9.len();
                    ::core::mem::forget(vec9);
                    *ptr6.add(12).cast::<usize>() = len9;
                    *ptr6.add(8).cast::<*mut u8>() = ptr9.cast_mut();
                    ptr6
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_reduce<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                    let l2 = *arg0.add(8).cast::<*mut u8>();
                    let l3 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                }
                pub trait Guest {
                    fn reduce(
                        key: _rt::String,
                        values: _rt::Vec<_rt::String>,
                    ) -> (_rt::String, _rt::String);
                }
                #[doc(hidden)]
                macro_rules! __export_component_reducewc_reduce_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "component:reducewc/reduce#reduce"] unsafe extern "C" fn
                        export_reduce(arg0 : * mut u8, arg1 : usize, arg2 : * mut u8,
                        arg3 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_reduce_cabi::<$ty > (arg0, arg1, arg2, arg3) }
                        #[export_name = "cabi_post_component:reducewc/reduce#reduce"]
                        unsafe extern "C" fn _post_return_reduce(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_reduce::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_reducewc_reduce_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 16]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 16],
                );
            }
        }
    }
}
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_reducer_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::reducewc::reduce::__export_component_reducewc_reduce_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::component::reducewc::reduce);
    };
}
#[doc(inline)]
pub(crate) use __export_reducer_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:component:reducewc:reducer:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 232] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07k\x01A\x02\x01A\x02\x01\
B\x04\x01ps\x01o\x02ss\x01@\x02\x03keys\x06values\0\0\x01\x04\0\x06reduce\x01\x02\
\x04\0\x19component:reducewc/reduce\x05\0\x04\0\x1acomponent:reducewc/reducer\x04\
\0\x0b\x0d\x01\0\x07reducer\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dw\
it-component\x070.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
