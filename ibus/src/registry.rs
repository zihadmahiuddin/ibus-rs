use crate::Registry;
use glib::ffi::GString;
use glib::object::IsA;
use glib::translate::ToGlibPtr;

pub trait RegistryExtManual: 'static {
    fn output(&self, output: String, indent: i32);
}

impl<O: IsA<Registry>> RegistryExtManual for O {
    fn output(&self, mut output: String, indent: i32) {
        unsafe {
            let mut output = GString {
                str: output.as_mut_ptr().cast(),
                allocated_len: output.capacity(),
                len: output.len(),
            };
            ffi::ibus_registry_output(
                self.as_ref().to_glib_none().0,
                std::ptr::addr_of_mut!(output),
                indent,
            );
        }
    }
}
