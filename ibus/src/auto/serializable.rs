// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use crate::{Object};
use glib::{prelude::*,translate::*};
use std::{fmt};

glib::wrapper! {
    #[doc(alias = "IBusSerializable")]
    pub struct Serializable(Object<ffi::IBusSerializable, ffi::IBusSerializableClass>) @extends Object;

    match fn {
        type_ => || ffi::ibus_serializable_get_type(),
    }
}

impl Serializable {
        pub const NONE: Option<&'static Serializable> = None;
    

    #[doc(alias = "ibus_serializable_new")]
    pub fn new() -> Serializable {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_serializable_new())
        }
    }

    #[doc(alias = "ibus_serializable_deserialize_object")]
    pub fn deserialize_object(variant: &glib::Variant) -> Option<Serializable> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_serializable_deserialize_object(variant.to_glib_none().0))
        }
    }
}

impl Default for Serializable {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait SerializableExt: 'static {
    #[doc(alias = "ibus_serializable_copy")]
#[must_use]
    fn copy(&self) -> Option<Serializable>;

    #[doc(alias = "ibus_serializable_get_qattachment")]
    #[doc(alias = "get_qattachment")]
    fn qattachment(&self, key: glib::Quark) -> Option<glib::Variant>;

    #[doc(alias = "ibus_serializable_remove_qattachment")]
    fn remove_qattachment(&self, key: glib::Quark);

    #[doc(alias = "ibus_serializable_serialize_object")]
    fn serialize_object(&self) -> Option<glib::Variant>;

    #[doc(alias = "ibus_serializable_set_qattachment")]
    fn set_qattachment(&self, key: glib::Quark, value: &glib::Variant);
}

impl<O: IsA<Serializable>> SerializableExt for O {
    fn copy(&self) -> Option<Serializable> {
        unsafe {
            from_glib_none(ffi::ibus_serializable_copy(self.as_ref().to_glib_none().0))
        }
    }

    fn qattachment(&self, key: glib::Quark) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::ibus_serializable_get_qattachment(self.as_ref().to_glib_none().0, key.into_glib()))
        }
    }

    fn remove_qattachment(&self, key: glib::Quark) {
        unsafe {
            ffi::ibus_serializable_remove_qattachment(self.as_ref().to_glib_none().0, key.into_glib());
        }
    }

    fn serialize_object(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::ibus_serializable_serialize_object(self.as_ref().to_glib_none().0))
        }
    }

    fn set_qattachment(&self, key: glib::Quark, value: &glib::Variant) {
        unsafe {
            ffi::ibus_serializable_set_qattachment(self.as_ref().to_glib_none().0, key.into_glib(), value.to_glib_none().0);
        }
    }
}

impl fmt::Display for Serializable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Serializable")
    }
}
