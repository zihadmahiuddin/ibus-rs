use crate::InputContext;
use glib::object::IsA;
use glib::translate::*;

pub trait InputContextExtManual: 'static {
    fn process_hand_writing_event(&self, coordinates: &[f64]);
}

impl<O: IsA<InputContext>> InputContextExtManual for O {
    fn process_hand_writing_event(&self, coordinates: &[f64]) {
        unsafe {
            ffi::ibus_input_context_process_hand_writing_event(
                self.as_ref().to_glib_none().0,
                coordinates.as_ptr(),
                coordinates
                    .len()
                    .try_into()
                    .expect("InputContext::process_hand_writing_event coordinates too large."),
            )
        }
    }
}
