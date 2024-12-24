#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod mapwc {
            #[allow(dead_code, clippy::all)]
            pub mod map {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_map_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg3;
                    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
                    let result2 = T::map(
                        _rt::string_lift(bytes0),
                        _rt::string_lift(bytes1),
                    );
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec7 = result2;
                    let len7 = vec7.len();
                    let layout7 = _rt::alloc::Layout::from_size_align_unchecked(
                        vec7.len() * 16,
                        4,
                    );
                    let result7 = if layout7.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout7).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout7);
                        }
                        ptr
                    } else {
                        ::core::ptr::null_mut()
                    };
                    for (i, e) in vec7.into_iter().enumerate() {
                        let base = result7.add(i * 16);
                        {
                            let (t4_0, t4_1) = e;
                            let vec5 = (t4_0.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *base.add(4).cast::<usize>() = len5;
                            *base.add(0).cast::<*mut u8>() = ptr5.cast_mut();
                            let vec6 = (t4_1.into_bytes()).into_boxed_slice();
                            let ptr6 = vec6.as_ptr().cast::<u8>();
                            let len6 = vec6.len();
                            ::core::mem::forget(vec6);
                            *base.add(12).cast::<usize>() = len6;
                            *base.add(8).cast::<*mut u8>() = ptr6.cast_mut();
                        }
                    }
                    *ptr3.add(4).cast::<usize>() = len7;
                    *ptr3.add(0).cast::<*mut u8>() = result7;
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_map<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    let base6 = l0;
                    let len6 = l1;
                    for i in 0..len6 {
                        let base = base6.add(i * 16);
                        {
                            let l2 = *base.add(0).cast::<*mut u8>();
                            let l3 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l2, l3, 1);
                            let l4 = *base.add(8).cast::<*mut u8>();
                            let l5 = *base.add(12).cast::<usize>();
                            _rt::cabi_dealloc(l4, l5, 1);
                        }
                    }
                    _rt::cabi_dealloc(base6, len6 * 16, 4);
                }
                pub trait Guest {
                    fn map(
                        key: _rt::String,
                        value: _rt::String,
                    ) -> _rt::Vec<(_rt::String, _rt::String)>;
                }
                #[doc(hidden)]
                macro_rules! __export_component_mapwc_map_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "component:mapwc/map#map"]
                        unsafe extern "C" fn export_map(arg0 : * mut u8, arg1 : usize,
                        arg2 : * mut u8, arg3 : usize,) -> * mut u8 {
                        $($path_to_types)*:: _export_map_cabi::<$ty > (arg0, arg1, arg2,
                        arg3) } #[export_name = "cabi_post_component:mapwc/map#map"]
                        unsafe extern "C" fn _post_return_map(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_map::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_mapwc_map_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
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
    pub use alloc_crate::alloc;
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
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
macro_rules! __export_mapper_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::component::mapwc::map::__export_component_mapwc_map_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::component::mapwc::map);
    };
}
#[doc(inline)]
pub(crate) use __export_mapper_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.35.0:component:mapwc:mapper:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 217] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07]\x01A\x02\x01A\x02\x01\
B\x04\x01o\x02ss\x01p\0\x01@\x02\x03keys\x05values\0\x01\x04\0\x03map\x01\x02\x04\
\0\x13component:mapwc/map\x05\0\x04\0\x16component:mapwc/mapper\x04\0\x0b\x0c\x01\
\0\x06mapper\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x07\
0.220.0\x10wit-bindgen-rust\x060.35.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
