// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use crate::{Attribute,Object,Serializable};
use glib::{prelude::*,translate::*};
use std::{fmt};

glib::wrapper! {
    #[doc(alias = "IBusAttrList")]
    pub struct AttrList(Object<ffi::IBusAttrList, ffi::IBusAttrListClass>) @extends Serializable, Object;

    match fn {
        type_ => || ffi::ibus_attr_list_get_type(),
    }
}

impl AttrList {
        pub const NONE: Option<&'static AttrList> = None;
    

    #[doc(alias = "ibus_attr_list_new")]
    pub fn new() -> AttrList {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_attr_list_new())
        }
    }
}

impl Default for AttrList {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait AttrListExt: 'static {
    #[doc(alias = "ibus_attr_list_append")]
    fn append(&self, attr: &impl IsA<Attribute>);

    #[doc(alias = "ibus_attr_list_get")]
    fn get(&self, index: u32) -> Option<Attribute>;
}

impl<O: IsA<AttrList>> AttrListExt for O {
    fn append(&self, attr: &impl IsA<Attribute>) {
        unsafe {
            ffi::ibus_attr_list_append(self.as_ref().to_glib_none().0, attr.as_ref().to_glib_none().0);
        }
    }

    fn get(&self, index: u32) -> Option<Attribute> {
        unsafe {
            from_glib_none(ffi::ibus_attr_list_get(self.as_ref().to_glib_none().0, index))
        }
    }
}

impl fmt::Display for AttrList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AttrList")
    }
}
