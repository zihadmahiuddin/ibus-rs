// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use crate::{Object,PropList,PropState,PropType,Serializable,Text};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_,fmt,mem::transmute};

glib::wrapper! {
    #[doc(alias = "IBusProperty")]
    pub struct Property(Object<ffi::IBusProperty, ffi::IBusPropertyClass>) @extends Serializable, Object;

    match fn {
        type_ => || ffi::ibus_property_get_type(),
    }
}

impl Property {
        pub const NONE: Option<&'static Property> = None;
    

    #[doc(alias = "ibus_property_new")]
    pub fn new(key: &str, type_: PropType, label: &impl IsA<Text>, icon: Option<&str>, tooltip: &impl IsA<Text>, sensitive: bool, visible: bool, state: PropState, prop_list: Option<&impl IsA<PropList>>) -> Property {
        skip_assert_initialized!();
        unsafe {
            from_glib_none(ffi::ibus_property_new(key.to_glib_none().0, type_.into_glib(), label.as_ref().to_glib_none().0, icon.to_glib_none().0, tooltip.as_ref().to_glib_none().0, sensitive.into_glib(), visible.into_glib(), state.into_glib(), prop_list.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    //#[doc(alias = "ibus_property_new_varargs")]
    //pub fn new_varargs(first_property_name: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Property {
    //    unsafe { TODO: call ffi:ibus_property_new_varargs() }
    //}

            // rustdoc-stripper-ignore-next
            /// Creates a new builder-pattern struct instance to construct [`Property`] objects.
            ///
            /// This method returns an instance of [`PropertyBuilder`](crate::builders::PropertyBuilder) which can be used to create [`Property`] objects.
            pub fn builder() -> PropertyBuilder {
                PropertyBuilder::new()
            }
        
}

impl Default for Property {
                     fn default() -> Self {
                         glib::object::Object::new::<Self>()
                     }
                 }

// rustdoc-stripper-ignore-next
        /// A [builder-pattern] type to construct [`Property`] objects.
        ///
        /// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PropertyBuilder {
            builder: glib::object::ObjectBuilder<'static, Property>,
        }

        impl PropertyBuilder {
        fn new() -> Self {
            Self { builder: glib::object::Object::builder() }
        }

                            pub fn icon(self, icon: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("icon", icon.into()), }
                        }

                            pub fn key(self, key: impl Into<glib::GString>) -> Self {
                            Self { builder: self.builder.property("key", key.into()), }
                        }

                            pub fn label(self, label: &impl IsA<Text>) -> Self {
                            Self { builder: self.builder.property("label", label.clone().upcast()), }
                        }

                            pub fn prop_type(self, prop_type: PropType) -> Self {
                            Self { builder: self.builder.property("prop-type", prop_type), }
                        }

                            pub fn sensitive(self, sensitive: bool) -> Self {
                            Self { builder: self.builder.property("sensitive", sensitive), }
                        }

                            pub fn state(self, state: PropState) -> Self {
                            Self { builder: self.builder.property("state", state), }
                        }

                            pub fn sub_props(self, sub_props: &impl IsA<PropList>) -> Self {
                            Self { builder: self.builder.property("sub-props", sub_props.clone().upcast()), }
                        }

                            pub fn symbol(self, symbol: &impl IsA<Text>) -> Self {
                            Self { builder: self.builder.property("symbol", symbol.clone().upcast()), }
                        }

                            pub fn tooltip(self, tooltip: &impl IsA<Text>) -> Self {
                            Self { builder: self.builder.property("tooltip", tooltip.clone().upcast()), }
                        }

                            pub fn visible(self, visible: bool) -> Self {
                            Self { builder: self.builder.property("visible", visible), }
                        }

    // rustdoc-stripper-ignore-next
    /// Build the [`Property`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> Property {
    self.builder.build() }
}

pub trait PropertyExt: 'static {
    #[doc(alias = "ibus_property_get_icon")]
    #[doc(alias = "get_icon")]
    fn icon(&self) -> Option<glib::GString>;

    #[doc(alias = "ibus_property_get_key")]
    #[doc(alias = "get_key")]
    fn key(&self) -> Option<glib::GString>;

    #[doc(alias = "ibus_property_get_label")]
    #[doc(alias = "get_label")]
    fn label(&self) -> Option<Text>;

    #[doc(alias = "ibus_property_get_prop_type")]
    #[doc(alias = "get_prop_type")]
    fn prop_type(&self) -> PropType;

    #[doc(alias = "ibus_property_get_sensitive")]
    #[doc(alias = "get_sensitive")]
    fn is_sensitive(&self) -> bool;

    #[doc(alias = "ibus_property_get_state")]
    #[doc(alias = "get_state")]
    fn state(&self) -> PropState;

    #[doc(alias = "ibus_property_get_sub_props")]
    #[doc(alias = "get_sub_props")]
    fn sub_props(&self) -> Option<PropList>;

    #[doc(alias = "ibus_property_get_symbol")]
    #[doc(alias = "get_symbol")]
    fn symbol(&self) -> Option<Text>;

    #[doc(alias = "ibus_property_get_tooltip")]
    #[doc(alias = "get_tooltip")]
    fn tooltip(&self) -> Option<Text>;

    #[doc(alias = "ibus_property_get_visible")]
    #[doc(alias = "get_visible")]
    fn is_visible(&self) -> bool;

    #[doc(alias = "ibus_property_set_icon")]
    fn set_icon(&self, icon: &str);

    #[doc(alias = "ibus_property_set_label")]
    fn set_label(&self, label: &impl IsA<Text>);

    #[doc(alias = "ibus_property_set_sensitive")]
    fn set_sensitive(&self, sensitive: bool);

    #[doc(alias = "ibus_property_set_state")]
    fn set_state(&self, state: PropState);

    #[doc(alias = "ibus_property_set_sub_props")]
    fn set_sub_props(&self, prop_list: &impl IsA<PropList>);

    #[doc(alias = "ibus_property_set_symbol")]
    fn set_symbol(&self, symbol: &impl IsA<Text>);

    #[doc(alias = "ibus_property_set_tooltip")]
    fn set_tooltip(&self, tooltip: &impl IsA<Text>);

    #[doc(alias = "ibus_property_set_visible")]
    fn set_visible(&self, visible: bool);

    #[doc(alias = "ibus_property_update")]
    fn update(&self, prop_update: &impl IsA<Property>) -> bool;

    #[doc(alias = "icon")]
    fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "label")]
    fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "sensitive")]
    fn connect_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "state")]
    fn connect_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "sub-props")]
    fn connect_sub_props_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "symbol")]
    fn connect_symbol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "tooltip")]
    fn connect_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "visible")]
    fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Property>> PropertyExt for O {
    fn icon(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_property_get_icon(self.as_ref().to_glib_none().0))
        }
    }

    fn key(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::ibus_property_get_key(self.as_ref().to_glib_none().0))
        }
    }

    fn label(&self) -> Option<Text> {
        unsafe {
            from_glib_none(ffi::ibus_property_get_label(self.as_ref().to_glib_none().0))
        }
    }

    fn prop_type(&self) -> PropType {
        unsafe {
            from_glib(ffi::ibus_property_get_prop_type(self.as_ref().to_glib_none().0))
        }
    }

    fn is_sensitive(&self) -> bool {
        unsafe {
            from_glib(ffi::ibus_property_get_sensitive(self.as_ref().to_glib_none().0))
        }
    }

    fn state(&self) -> PropState {
        unsafe {
            from_glib(ffi::ibus_property_get_state(self.as_ref().to_glib_none().0))
        }
    }

    fn sub_props(&self) -> Option<PropList> {
        unsafe {
            from_glib_none(ffi::ibus_property_get_sub_props(self.as_ref().to_glib_none().0))
        }
    }

    fn symbol(&self) -> Option<Text> {
        unsafe {
            from_glib_none(ffi::ibus_property_get_symbol(self.as_ref().to_glib_none().0))
        }
    }

    fn tooltip(&self) -> Option<Text> {
        unsafe {
            from_glib_none(ffi::ibus_property_get_tooltip(self.as_ref().to_glib_none().0))
        }
    }

    fn is_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::ibus_property_get_visible(self.as_ref().to_glib_none().0))
        }
    }

    fn set_icon(&self, icon: &str) {
        unsafe {
            ffi::ibus_property_set_icon(self.as_ref().to_glib_none().0, icon.to_glib_none().0);
        }
    }

    fn set_label(&self, label: &impl IsA<Text>) {
        unsafe {
            ffi::ibus_property_set_label(self.as_ref().to_glib_none().0, label.as_ref().to_glib_none().0);
        }
    }

    fn set_sensitive(&self, sensitive: bool) {
        unsafe {
            ffi::ibus_property_set_sensitive(self.as_ref().to_glib_none().0, sensitive.into_glib());
        }
    }

    fn set_state(&self, state: PropState) {
        unsafe {
            ffi::ibus_property_set_state(self.as_ref().to_glib_none().0, state.into_glib());
        }
    }

    fn set_sub_props(&self, prop_list: &impl IsA<PropList>) {
        unsafe {
            ffi::ibus_property_set_sub_props(self.as_ref().to_glib_none().0, prop_list.as_ref().to_glib_none().0);
        }
    }

    fn set_symbol(&self, symbol: &impl IsA<Text>) {
        unsafe {
            ffi::ibus_property_set_symbol(self.as_ref().to_glib_none().0, symbol.as_ref().to_glib_none().0);
        }
    }

    fn set_tooltip(&self, tooltip: &impl IsA<Text>) {
        unsafe {
            ffi::ibus_property_set_tooltip(self.as_ref().to_glib_none().0, tooltip.as_ref().to_glib_none().0);
        }
    }

    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::ibus_property_set_visible(self.as_ref().to_glib_none().0, visible.into_glib());
        }
    }

    fn update(&self, prop_update: &impl IsA<Property>) -> bool {
        unsafe {
            from_glib(ffi::ibus_property_update(self.as_ref().to_glib_none().0, prop_update.as_ref().to_glib_none().0))
        }
    }

    fn connect_icon_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_trampoline<P: IsA<Property>, F: Fn(&P) + 'static>(this: *mut ffi::IBusProperty, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Property::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_icon_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<P: IsA<Property>, F: Fn(&P) + 'static>(this: *mut ffi::IBusProperty, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Property::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_label_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_sensitive_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sensitive_trampoline<P: IsA<Property>, F: Fn(&P) + 'static>(this: *mut ffi::IBusProperty, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Property::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::sensitive\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_sensitive_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<P: IsA<Property>, F: Fn(&P) + 'static>(this: *mut ffi::IBusProperty, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Property::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_state_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_sub_props_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sub_props_trampoline<P: IsA<Property>, F: Fn(&P) + 'static>(this: *mut ffi::IBusProperty, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Property::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::sub-props\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_sub_props_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_symbol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_symbol_trampoline<P: IsA<Property>, F: Fn(&P) + 'static>(this: *mut ffi::IBusProperty, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Property::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::symbol\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_symbol_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_tooltip_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_tooltip_trampoline<P: IsA<Property>, F: Fn(&P) + 'static>(this: *mut ffi::IBusProperty, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Property::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::tooltip\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_tooltip_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<P: IsA<Property>, F: Fn(&P) + 'static>(this: *mut ffi::IBusProperty, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(Property::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visible\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_visible_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Property")
    }
}
