use crate::XML;
use glib::ffi::GString;
use glib::translate::*;

pub trait XMLExtManual: 'static {
    fn output(&self, output: String);
}

impl XMLExtManual for XML {
    fn output(&self, mut output: String) {
        unsafe {
            let mut output = GString {
                str: output.as_mut_ptr().cast(),
                allocated_len: output.capacity(),
                len: output.len(),
            };
            ffi::ibus_xml_output(self.to_glib_none().0, std::ptr::addr_of_mut!(output));
        }
    }
}
