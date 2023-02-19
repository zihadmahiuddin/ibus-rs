use std::ptr::null_mut;
use std::time::Duration;

use crate::Bus;
use glib::object::IsA;
use glib::translate::ToGlibPtr;

pub trait BusExtManual: 'static {
    fn set_global_engine_async(&self, global_engine: &str, timeout: Option<Duration>);
}

impl<O: IsA<Bus>> BusExtManual for O {
    fn set_global_engine_async(&self, global_engine: &str, timeout: Option<Duration>) {
        unsafe {
            ffi::ibus_bus_set_global_engine_async(
                self.as_ref().to_glib_none().0,
                global_engine.to_glib_none().0,
                if let Some(timeout) = timeout {
                    timeout.as_millis().try_into().expect("Timeout too big.")
                } else {
                    -1
                },
                null_mut(),
                None,
                null_mut(),
            );
        }
    }
}
