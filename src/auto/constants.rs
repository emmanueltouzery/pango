// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use pango_sys;
use std::ffi::CStr;

#[cfg_attr(feature = "v1_38", deprecated)]
lazy_static! {
    pub static ref ENGINE_TYPE_LANG: &'static str = unsafe {
        CStr::from_ptr(pango_sys::PANGO_ENGINE_TYPE_LANG)
            .to_str()
            .unwrap()
    };
}
#[cfg_attr(feature = "v1_38", deprecated)]
lazy_static! {
    pub static ref ENGINE_TYPE_SHAPE: &'static str = unsafe {
        CStr::from_ptr(pango_sys::PANGO_ENGINE_TYPE_SHAPE)
            .to_str()
            .unwrap()
    };
}
#[cfg_attr(feature = "v1_38", deprecated)]
lazy_static! {
    pub static ref RENDER_TYPE_NONE: &'static str = unsafe {
        CStr::from_ptr(pango_sys::PANGO_RENDER_TYPE_NONE)
            .to_str()
            .unwrap()
    };
}
