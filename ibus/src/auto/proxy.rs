// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_,fmt,mem::transmute};

#[cfg(any(feature = "gio_v2_26", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
glib::wrapper! {
    #[doc(alias = "IBusProxy")]
    pub struct Proxy(Object<ffi::IBusProxy, ffi::IBusProxyClass>) @extends gio::DBusProxy, @implements gio::AsyncInitable, gio::DBusInterface, gio::Initable;

    match fn {
        type_ => || ffi::ibus_proxy_get_type(),
    }
}

#[cfg(not(any(feature = "gio_v2_26", feature = "dox")))]
#[cfg(any(feature = "gio_v2_22", feature = "dox"))]
glib::wrapper! {
    #[doc(alias = "IBusProxy")]
    pub struct Proxy(Object<ffi::IBusProxy, ffi::IBusProxyClass>) @implements gio::AsyncInitable, gio::DBusInterface, gio::Initable;

    match fn {
        type_ => || ffi::ibus_proxy_get_type(),
    }
}

#[cfg(not(any(feature = "gio_v2_22", feature = "dox")))]
#[cfg(any(feature = "gio_v2_30", feature = "dox"))]
glib::wrapper! {
    #[doc(alias = "IBusProxy")]
    pub struct Proxy(Object<ffi::IBusProxy, ffi::IBusProxyClass>) @implements gio::DBusInterface, gio::Initable;

    match fn {
        type_ => || ffi::ibus_proxy_get_type(),
    }
}

#[cfg(not(any(feature = "gio_v2_30", feature = "dox")))]
#[cfg(any(feature = "gio_v2_22", feature = "dox"))]
glib::wrapper! {
    #[doc(alias = "IBusProxy")]
    pub struct Proxy(Object<ffi::IBusProxy, ffi::IBusProxyClass>) @implements gio::Initable;

    match fn {
        type_ => || ffi::ibus_proxy_get_type(),
    }
}

#[cfg(not(any(feature = "gio_v2_22", feature = "dox")))]
glib::wrapper! {
    #[doc(alias = "IBusProxy")]
    pub struct Proxy(Object<ffi::IBusProxy, ffi::IBusProxyClass>);

    match fn {
        type_ => || ffi::ibus_proxy_get_type(),
    }
}

impl Proxy {
        pub const NONE: Option<&'static Proxy> = None;
    

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Proxy`] objects.
            ///
            /// This method returns an instance of [`ProxyBuilder`](crate::builders::ProxyBuilder) which can be used to create [`Proxy`] objects.
            pub fn builder() -> ProxyBuilder {
                ProxyBuilder::new()
            }
        
}

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Proxy`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ProxyBuilder {
            builder: glib::object::ObjectBuilder<'static, Proxy>,
        }

        impl ProxyBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                        //    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    //pub fn g_bus_type(self, g_bus_type: /*Ignored*/gio::BusType) -> Self {
                        //    Self { builder: self.builder.property("g-bus-type", g_bus_type), }
                        //}

                            #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_connection(self, g_connection: &gio::DBusConnection) -> Self {
                            Self { builder: self.builder.property("g-connection", g_connection.clone()), }
                        }

                            #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_default_timeout(self, g_default_timeout: i32) -> Self {
                            Self { builder: self.builder.property("g-default-timeout", g_default_timeout), }
                        }

                        //    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    //pub fn g_flags(self, g_flags: /*Ignored*/gio::DBusProxyFlags) -> Self {
                        //    Self { builder: self.builder.property("g-flags", g_flags), }
                        //}

                        //    #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    //pub fn g_interface_info(self, g_interface_info: /*Ignored*/&gio::DBusInterfaceInfo) -> Self {
                        //    Self { builder: self.builder.property("g-interface-info", g_interface_info), }
                        //}

                            #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_interface_name(self, g_interface_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("g-interface-name", g_interface_name.into()), }
                        }

                            #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_name(self, g_name: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("g-name", g_name.into()), }
                        }

                            #[cfg(any(feature = "gio_v2_26", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "gio_v2_26")))]
    pub fn g_object_path(self, g_object_path: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("g-object-path", g_object_path.into()), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Proxy`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Proxy {
    self.builder.build() }
}

pub trait ProxyExt: 'static {
    #[doc(alias = "ibus_proxy_destroy")]
    fn destroy(&self);

    #[doc(alias = "destroy")]
    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Proxy>> ProxyExt for O {
    fn destroy(&self) {
        unsafe {
            ffi::ibus_proxy_destroy(self.as_ref().to_glib_none().0);
        }
    }

    fn connect_destroy<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn destroy_trampoline<P: IsA<Proxy>, F: Fn(&P) + 'static>(this: *mut ffi::IBusProxy, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Proxy::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"destroy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(destroy_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Proxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Proxy")
    }
}
