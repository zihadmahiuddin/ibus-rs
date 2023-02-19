// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use crate::{Engine,Object,Service};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_,fmt,mem::transmute};

glib::wrapper! {
    #[doc(alias = "IBusFactory")]
    pub struct Factory(Object<ffi::IBusFactory, ffi::IBusFactoryClass>) @extends Service, Object;

    match fn {
        type_ => || ffi::ibus_factory_get_type(),
    }
}

impl Factory {
        pub const NONE: Option<&'static Factory> = None;
    

    #[doc(alias = "ibus_factory_new")]
    pub fn new(connection: &gio::DBusConnection) -> Factory {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_factory_new(connection.to_glib_none().0))
        }
    }

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Factory`] objects.
            ///
            /// This method returns an instance of [`FactoryBuilder`](crate::builders::FactoryBuilder) which can be used to create [`Factory`] objects.
            pub fn builder() -> FactoryBuilder {
                FactoryBuilder::new()
            }
        
}

impl Default for Factory {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Factory`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FactoryBuilder {
            builder: glib::object::ObjectBuilder<'static, Factory>,
        }

        impl FactoryBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn connection(self, connection: &gio::DBusConnection) -> Self {
                            Self { builder: self.builder.property("connection", connection.clone()), }
                        }

                            pub fn object_path(self, object_path: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("object-path", object_path.into()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Factory`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Factory {
    self.builder.build() }
}

pub trait FactoryExt: 'static {
    #[doc(alias = "ibus_factory_add_engine")]
    fn add_engine(&self, engine_name: &str, engine_type: glib::types::Type);

    #[doc(alias = "ibus_factory_create_engine")]
    fn create_engine(&self, engine_name: &str) -> Option<Engine>;

    #[doc(alias = "create-engine")]
    fn connect_create_engine<F: Fn(&Self, &str) -> Option<Engine> + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Factory>> FactoryExt for O {
    fn add_engine(&self, engine_name: &str, engine_type: glib::types::Type) {
        unsafe {
            ffi::ibus_factory_add_engine(self.as_ref().to_glib_none().0, engine_name.to_glib_none().0, engine_type.into_glib());
        }
    }

    fn create_engine(&self, engine_name: &str) -> Option<Engine> {
        unsafe {
            from_glib_full(ffi::ibus_factory_create_engine(self.as_ref().to_glib_none().0, engine_name.to_glib_none().0))
        }
    }

    fn connect_create_engine<F: Fn(&Self, &str) -> Option<Engine> + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn create_engine_trampoline<P: IsA<Factory>, F: Fn(&P, &str) -> Option<Engine> + 'static>(this: *mut ffi::IBusFactory, engine_name: *mut libc::c_char, f: glib::ffi::gpointer) -> *mut ffi::IBusEngine {
            let f: &F = &*(f as *const F);
            f(Factory::from_glib_borrow(this).unsafe_cast_ref(), &glib::GString::from_glib_borrow(engine_name)).to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"create-engine\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(create_engine_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Factory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Factory")
    }
}
