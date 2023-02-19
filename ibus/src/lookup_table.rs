use crate::{LookupTable, Orientation};
use glib::object::IsA;
use glib::translate::{IntoGlib, ToGlibPtr};

pub trait LookupTableExtManual: 'static {
    fn set_orientation(&self, orientation: Orientation);
}

impl<O: IsA<LookupTable>> LookupTableExtManual for O {
    fn set_orientation(&self, orientation: Orientation) {
        unsafe {
            ffi::ibus_lookup_table_set_orientation(
                self.as_ref().to_glib_none().0,
                orientation.into_glib(),
            );
        }
    }
}
