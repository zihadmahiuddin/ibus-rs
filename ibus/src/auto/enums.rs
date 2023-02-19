// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use crate::{Attribute};
use glib::{error::ErrorDomain,translate::*,value::FromValue,value::ToValue,Quark,StaticType,Type};
use std::{fmt};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "IBusAttrType")]
pub enum AttrType {
    #[doc(alias = "IBUS_ATTR_TYPE_UNDERLINE")]
    Underline,
    #[doc(alias = "IBUS_ATTR_TYPE_FOREGROUND")]
    Foreground,
    #[doc(alias = "IBUS_ATTR_TYPE_BACKGROUND")]
    Background,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for AttrType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AttrType::{}", match *self {
            Self::Underline => "Underline",
            Self::Foreground => "Foreground",
            Self::Background => "Background",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for AttrType {
    type GlibType = ffi::IBusAttrType;

    #[inline]
fn into_glib(self) -> ffi::IBusAttrType {
match self {
            Self::Underline => ffi::IBUS_ATTR_TYPE_UNDERLINE,
            Self::Foreground => ffi::IBUS_ATTR_TYPE_FOREGROUND,
            Self::Background => ffi::IBUS_ATTR_TYPE_BACKGROUND,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::IBusAttrType> for AttrType {
    #[inline]
unsafe fn from_glib(value: ffi::IBusAttrType) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::IBUS_ATTR_TYPE_UNDERLINE => Self::Underline,
            ffi::IBUS_ATTR_TYPE_FOREGROUND => Self::Foreground,
            ffi::IBUS_ATTR_TYPE_BACKGROUND => Self::Background,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for AttrType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ibus_attr_type_get_type()) }
    }
}

impl glib::HasParamSpec for AttrType {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
                }
}

impl glib::value::ValueType for AttrType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AttrType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AttrType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<AttrType> for glib::Value {
    #[inline]
    fn from(v: AttrType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "IBusAttrUnderline")]
pub enum AttrUnderline {
    #[doc(alias = "IBUS_ATTR_UNDERLINE_NONE")]
    None,
    #[doc(alias = "IBUS_ATTR_UNDERLINE_SINGLE")]
    Single,
    #[doc(alias = "IBUS_ATTR_UNDERLINE_DOUBLE")]
    Double,
    #[doc(alias = "IBUS_ATTR_UNDERLINE_LOW")]
    Low,
    #[doc(alias = "IBUS_ATTR_UNDERLINE_ERROR")]
    Error,
#[doc(hidden)]
    __Unknown(i32),
}

impl AttrUnderline {
    #[doc(alias = "ibus_attr_underline_new")]
    pub fn new(underline_type: u32, start_index: u32, end_index: u32) -> Option<Attribute> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::ibus_attr_underline_new(underline_type, start_index, end_index))
        }
    }
}

impl fmt::Display for AttrUnderline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AttrUnderline::{}", match *self {
            Self::None => "None",
            Self::Single => "Single",
            Self::Double => "Double",
            Self::Low => "Low",
            Self::Error => "Error",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for AttrUnderline {
    type GlibType = ffi::IBusAttrUnderline;

    #[inline]
fn into_glib(self) -> ffi::IBusAttrUnderline {
match self {
            Self::None => ffi::IBUS_ATTR_UNDERLINE_NONE,
            Self::Single => ffi::IBUS_ATTR_UNDERLINE_SINGLE,
            Self::Double => ffi::IBUS_ATTR_UNDERLINE_DOUBLE,
            Self::Low => ffi::IBUS_ATTR_UNDERLINE_LOW,
            Self::Error => ffi::IBUS_ATTR_UNDERLINE_ERROR,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::IBusAttrUnderline> for AttrUnderline {
    #[inline]
unsafe fn from_glib(value: ffi::IBusAttrUnderline) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::IBUS_ATTR_UNDERLINE_NONE => Self::None,
            ffi::IBUS_ATTR_UNDERLINE_SINGLE => Self::Single,
            ffi::IBUS_ATTR_UNDERLINE_DOUBLE => Self::Double,
            ffi::IBUS_ATTR_UNDERLINE_LOW => Self::Low,
            ffi::IBUS_ATTR_UNDERLINE_ERROR => Self::Error,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for AttrUnderline {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ibus_attr_underline_get_type()) }
    }
}

impl glib::HasParamSpec for AttrUnderline {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
                }
}

impl glib::value::ValueType for AttrUnderline {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for AttrUnderline {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for AttrUnderline {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<AttrUnderline> for glib::Value {
    #[inline]
    fn from(v: AttrUnderline) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "IBusBusRequestNameReply")]
pub enum BusRequestNameReply {
    #[doc(alias = "IBUS_BUS_REQUEST_NAME_REPLY_PRIMARY_OWNER")]
    PrimaryOwner,
    #[doc(alias = "IBUS_BUS_REQUEST_NAME_REPLY_IN_QUEUE")]
    InQueue,
    #[doc(alias = "IBUS_BUS_REQUEST_NAME_REPLY_EXISTS")]
    Exists,
    #[doc(alias = "IBUS_BUS_REQUEST_NAME_REPLY_ALREADY_OWNER")]
    AlreadyOwner,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BusRequestNameReply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BusRequestNameReply::{}", match *self {
            Self::PrimaryOwner => "PrimaryOwner",
            Self::InQueue => "InQueue",
            Self::Exists => "Exists",
            Self::AlreadyOwner => "AlreadyOwner",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for BusRequestNameReply {
    type GlibType = ffi::IBusBusRequestNameReply;

    #[inline]
fn into_glib(self) -> ffi::IBusBusRequestNameReply {
match self {
            Self::PrimaryOwner => ffi::IBUS_BUS_REQUEST_NAME_REPLY_PRIMARY_OWNER,
            Self::InQueue => ffi::IBUS_BUS_REQUEST_NAME_REPLY_IN_QUEUE,
            Self::Exists => ffi::IBUS_BUS_REQUEST_NAME_REPLY_EXISTS,
            Self::AlreadyOwner => ffi::IBUS_BUS_REQUEST_NAME_REPLY_ALREADY_OWNER,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::IBusBusRequestNameReply> for BusRequestNameReply {
    #[inline]
unsafe fn from_glib(value: ffi::IBusBusRequestNameReply) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::IBUS_BUS_REQUEST_NAME_REPLY_PRIMARY_OWNER => Self::PrimaryOwner,
            ffi::IBUS_BUS_REQUEST_NAME_REPLY_IN_QUEUE => Self::InQueue,
            ffi::IBUS_BUS_REQUEST_NAME_REPLY_EXISTS => Self::Exists,
            ffi::IBUS_BUS_REQUEST_NAME_REPLY_ALREADY_OWNER => Self::AlreadyOwner,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for BusRequestNameReply {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ibus_bus_request_name_reply_get_type()) }
    }
}

impl glib::HasParamSpec for BusRequestNameReply {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
                }
}

impl glib::value::ValueType for BusRequestNameReply {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for BusRequestNameReply {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for BusRequestNameReply {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<BusRequestNameReply> for glib::Value {
    #[inline]
    fn from(v: BusRequestNameReply) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "IBusBusStartServiceByNameReply")]
pub enum BusStartServiceByNameReply {
    #[doc(alias = "IBUS_BUS_START_REPLY_SUCCESS")]
    Success,
    #[doc(alias = "IBUS_BUS_START_REPLY_ALREADY_RUNNING")]
    AlreadyRunning,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for BusStartServiceByNameReply {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BusStartServiceByNameReply::{}", match *self {
            Self::Success => "Success",
            Self::AlreadyRunning => "AlreadyRunning",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for BusStartServiceByNameReply {
    type GlibType = ffi::IBusBusStartServiceByNameReply;

    #[inline]
fn into_glib(self) -> ffi::IBusBusStartServiceByNameReply {
match self {
            Self::Success => ffi::IBUS_BUS_START_REPLY_SUCCESS,
            Self::AlreadyRunning => ffi::IBUS_BUS_START_REPLY_ALREADY_RUNNING,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::IBusBusStartServiceByNameReply> for BusStartServiceByNameReply {
    #[inline]
unsafe fn from_glib(value: ffi::IBusBusStartServiceByNameReply) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::IBUS_BUS_START_REPLY_SUCCESS => Self::Success,
            ffi::IBUS_BUS_START_REPLY_ALREADY_RUNNING => Self::AlreadyRunning,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for BusStartServiceByNameReply {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ibus_bus_start_service_by_name_reply_get_type()) }
    }
}

impl glib::HasParamSpec for BusStartServiceByNameReply {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
                }
}

impl glib::value::ValueType for BusStartServiceByNameReply {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for BusStartServiceByNameReply {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for BusStartServiceByNameReply {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<BusStartServiceByNameReply> for glib::Value {
    #[inline]
    fn from(v: BusStartServiceByNameReply) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "IBusError")]
pub enum Error {
    #[doc(alias = "IBUS_ERROR_NO_ENGINE")]
    NoEngine,
    #[doc(alias = "IBUS_ERROR_NO_CONFIG")]
    NoConfig,
    #[doc(alias = "IBUS_ERROR_FAILED")]
    Failed,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error::{}", match *self {
            Self::NoEngine => "NoEngine",
            Self::NoConfig => "NoConfig",
            Self::Failed => "Failed",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for Error {
    type GlibType = ffi::IBusError;

    #[inline]
fn into_glib(self) -> ffi::IBusError {
match self {
            Self::NoEngine => ffi::IBUS_ERROR_NO_ENGINE,
            Self::NoConfig => ffi::IBUS_ERROR_NO_CONFIG,
            Self::Failed => ffi::IBUS_ERROR_FAILED,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::IBusError> for Error {
    #[inline]
unsafe fn from_glib(value: ffi::IBusError) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::IBUS_ERROR_NO_ENGINE => Self::NoEngine,
            ffi::IBUS_ERROR_NO_CONFIG => Self::NoConfig,
            ffi::IBUS_ERROR_FAILED => Self::Failed,
            value => Self::__Unknown(value),
}
}
}

impl ErrorDomain for Error {
    #[inline]
    fn domain() -> Quark {
        skip_assert_initialized!();
        
        unsafe { from_glib(ffi::ibus_error_quark()) }
    }

    #[inline]
    fn code(self) -> i32 {
        self.into_glib()
    }

    #[inline]
    #[allow(clippy::match_single_binding)]
    fn from(code: i32) -> Option<Self> {
        skip_assert_initialized!();
        match unsafe { from_glib(code) } {
            Self::__Unknown(_) => Some(Self::Failed),
            value => Some(value),
}
    }
}

impl StaticType for Error {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ibus_error_get_type()) }
    }
}

impl glib::HasParamSpec for Error {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
                }
}

impl glib::value::ValueType for Error {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for Error {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Error {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Error> for glib::Value {
    #[inline]
    fn from(v: Error) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[cfg(any(feature = "v1_5_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_5_4")))]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "IBusInputPurpose")]
pub enum InputPurpose {
    #[doc(alias = "IBUS_INPUT_PURPOSE_FREE_FORM")]
    FreeForm,
    #[doc(alias = "IBUS_INPUT_PURPOSE_ALPHA")]
    Alpha,
    #[doc(alias = "IBUS_INPUT_PURPOSE_DIGITS")]
    Digits,
    #[doc(alias = "IBUS_INPUT_PURPOSE_NUMBER")]
    Number,
    #[doc(alias = "IBUS_INPUT_PURPOSE_PHONE")]
    Phone,
    #[doc(alias = "IBUS_INPUT_PURPOSE_URL")]
    Url,
    #[doc(alias = "IBUS_INPUT_PURPOSE_EMAIL")]
    Email,
    #[doc(alias = "IBUS_INPUT_PURPOSE_NAME")]
    Name,
    #[doc(alias = "IBUS_INPUT_PURPOSE_PASSWORD")]
    Password,
    #[doc(alias = "IBUS_INPUT_PURPOSE_PIN")]
    Pin,
    #[doc(alias = "IBUS_INPUT_PURPOSE_TERMINAL")]
    Terminal,
#[doc(hidden)]
    __Unknown(i32),
}

#[cfg(any(feature = "v1_5_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_5_4")))]
impl fmt::Display for InputPurpose {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "InputPurpose::{}", match *self {
            Self::FreeForm => "FreeForm",
            Self::Alpha => "Alpha",
            Self::Digits => "Digits",
            Self::Number => "Number",
            Self::Phone => "Phone",
            Self::Url => "Url",
            Self::Email => "Email",
            Self::Name => "Name",
            Self::Password => "Password",
            Self::Pin => "Pin",
            Self::Terminal => "Terminal",
            _ => "Unknown",
        })
    }
}

#[cfg(any(feature = "v1_5_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_5_4")))]
#[doc(hidden)]
impl IntoGlib for InputPurpose {
    type GlibType = ffi::IBusInputPurpose;

    #[inline]
fn into_glib(self) -> ffi::IBusInputPurpose {
match self {
            Self::FreeForm => ffi::IBUS_INPUT_PURPOSE_FREE_FORM,
            Self::Alpha => ffi::IBUS_INPUT_PURPOSE_ALPHA,
            Self::Digits => ffi::IBUS_INPUT_PURPOSE_DIGITS,
            Self::Number => ffi::IBUS_INPUT_PURPOSE_NUMBER,
            Self::Phone => ffi::IBUS_INPUT_PURPOSE_PHONE,
            Self::Url => ffi::IBUS_INPUT_PURPOSE_URL,
            Self::Email => ffi::IBUS_INPUT_PURPOSE_EMAIL,
            Self::Name => ffi::IBUS_INPUT_PURPOSE_NAME,
            Self::Password => ffi::IBUS_INPUT_PURPOSE_PASSWORD,
            Self::Pin => ffi::IBUS_INPUT_PURPOSE_PIN,
            Self::Terminal => ffi::IBUS_INPUT_PURPOSE_TERMINAL,
            Self::__Unknown(value) => value,
}
}
}

#[cfg(any(feature = "v1_5_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_5_4")))]
#[doc(hidden)]
impl FromGlib<ffi::IBusInputPurpose> for InputPurpose {
    #[inline]
unsafe fn from_glib(value: ffi::IBusInputPurpose) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::IBUS_INPUT_PURPOSE_FREE_FORM => Self::FreeForm,
            ffi::IBUS_INPUT_PURPOSE_ALPHA => Self::Alpha,
            ffi::IBUS_INPUT_PURPOSE_DIGITS => Self::Digits,
            ffi::IBUS_INPUT_PURPOSE_NUMBER => Self::Number,
            ffi::IBUS_INPUT_PURPOSE_PHONE => Self::Phone,
            ffi::IBUS_INPUT_PURPOSE_URL => Self::Url,
            ffi::IBUS_INPUT_PURPOSE_EMAIL => Self::Email,
            ffi::IBUS_INPUT_PURPOSE_NAME => Self::Name,
            ffi::IBUS_INPUT_PURPOSE_PASSWORD => Self::Password,
            ffi::IBUS_INPUT_PURPOSE_PIN => Self::Pin,
            ffi::IBUS_INPUT_PURPOSE_TERMINAL => Self::Terminal,
            value => Self::__Unknown(value),
}
}
}

#[cfg(any(feature = "v1_5_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_5_4")))]
impl StaticType for InputPurpose {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ibus_input_purpose_get_type()) }
    }
}

#[cfg(any(feature = "v1_5_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_5_4")))]
impl glib::HasParamSpec for InputPurpose {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
                }
}

#[cfg(any(feature = "v1_5_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_5_4")))]
impl glib::value::ValueType for InputPurpose {
    type Type = Self;
}

#[cfg(any(feature = "v1_5_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_5_4")))]
unsafe impl<'a> FromValue<'a> for InputPurpose {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

#[cfg(any(feature = "v1_5_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_5_4")))]
impl ToValue for InputPurpose {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg(any(feature = "v1_5_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_5_4")))]
impl From<InputPurpose> for glib::Value {
    #[inline]
    fn from(v: InputPurpose) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "IBusOrientation")]
pub enum Orientation {
    #[doc(alias = "IBUS_ORIENTATION_HORIZONTAL")]
    Horizontal,
    #[doc(alias = "IBUS_ORIENTATION_VERTICAL")]
    Vertical,
    #[doc(alias = "IBUS_ORIENTATION_SYSTEM")]
    System,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Orientation::{}", match *self {
            Self::Horizontal => "Horizontal",
            Self::Vertical => "Vertical",
            Self::System => "System",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for Orientation {
    type GlibType = ffi::IBusOrientation;

    #[inline]
fn into_glib(self) -> ffi::IBusOrientation {
match self {
            Self::Horizontal => ffi::IBUS_ORIENTATION_HORIZONTAL,
            Self::Vertical => ffi::IBUS_ORIENTATION_VERTICAL,
            Self::System => ffi::IBUS_ORIENTATION_SYSTEM,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::IBusOrientation> for Orientation {
    #[inline]
unsafe fn from_glib(value: ffi::IBusOrientation) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::IBUS_ORIENTATION_HORIZONTAL => Self::Horizontal,
            ffi::IBUS_ORIENTATION_VERTICAL => Self::Vertical,
            ffi::IBUS_ORIENTATION_SYSTEM => Self::System,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for Orientation {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ibus_orientation_get_type()) }
    }
}

impl glib::HasParamSpec for Orientation {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
                }
}

impl glib::value::ValueType for Orientation {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for Orientation {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Orientation {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<Orientation> for glib::Value {
    #[inline]
    fn from(v: Orientation) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "IBusPreeditFocusMode")]
pub enum PreeditFocusMode {
    #[doc(alias = "IBUS_ENGINE_PREEDIT_CLEAR")]
    Clear,
    #[doc(alias = "IBUS_ENGINE_PREEDIT_COMMIT")]
    Commit,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PreeditFocusMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PreeditFocusMode::{}", match *self {
            Self::Clear => "Clear",
            Self::Commit => "Commit",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for PreeditFocusMode {
    type GlibType = ffi::IBusPreeditFocusMode;

    #[inline]
fn into_glib(self) -> ffi::IBusPreeditFocusMode {
match self {
            Self::Clear => ffi::IBUS_ENGINE_PREEDIT_CLEAR,
            Self::Commit => ffi::IBUS_ENGINE_PREEDIT_COMMIT,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::IBusPreeditFocusMode> for PreeditFocusMode {
    #[inline]
unsafe fn from_glib(value: ffi::IBusPreeditFocusMode) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::IBUS_ENGINE_PREEDIT_CLEAR => Self::Clear,
            ffi::IBUS_ENGINE_PREEDIT_COMMIT => Self::Commit,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for PreeditFocusMode {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ibus_preedit_focus_mode_get_type()) }
    }
}

impl glib::HasParamSpec for PreeditFocusMode {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
                }
}

impl glib::value::ValueType for PreeditFocusMode {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PreeditFocusMode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PreeditFocusMode {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<PreeditFocusMode> for glib::Value {
    #[inline]
    fn from(v: PreeditFocusMode) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "IBusPropState")]
pub enum PropState {
    #[doc(alias = "PROP_STATE_UNCHECKED")]
    Unchecked,
    #[doc(alias = "PROP_STATE_CHECKED")]
    Checked,
    #[doc(alias = "PROP_STATE_INCONSISTENT")]
    Inconsistent,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PropState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PropState::{}", match *self {
            Self::Unchecked => "Unchecked",
            Self::Checked => "Checked",
            Self::Inconsistent => "Inconsistent",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for PropState {
    type GlibType = ffi::IBusPropState;

    #[inline]
fn into_glib(self) -> ffi::IBusPropState {
match self {
            Self::Unchecked => ffi::PROP_STATE_UNCHECKED,
            Self::Checked => ffi::PROP_STATE_CHECKED,
            Self::Inconsistent => ffi::PROP_STATE_INCONSISTENT,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::IBusPropState> for PropState {
    #[inline]
unsafe fn from_glib(value: ffi::IBusPropState) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::PROP_STATE_UNCHECKED => Self::Unchecked,
            ffi::PROP_STATE_CHECKED => Self::Checked,
            ffi::PROP_STATE_INCONSISTENT => Self::Inconsistent,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for PropState {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ibus_prop_state_get_type()) }
    }
}

impl glib::HasParamSpec for PropState {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
                }
}

impl glib::value::ValueType for PropState {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PropState {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PropState {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<PropState> for glib::Value {
    #[inline]
    fn from(v: PropState) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "IBusPropType")]
pub enum PropType {
    #[doc(alias = "PROP_TYPE_NORMAL")]
    Normal,
    #[doc(alias = "PROP_TYPE_TOGGLE")]
    Toggle,
    #[doc(alias = "PROP_TYPE_RADIO")]
    Radio,
    #[doc(alias = "PROP_TYPE_MENU")]
    Menu,
    #[doc(alias = "PROP_TYPE_SEPARATOR")]
    Separator,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PropType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PropType::{}", match *self {
            Self::Normal => "Normal",
            Self::Toggle => "Toggle",
            Self::Radio => "Radio",
            Self::Menu => "Menu",
            Self::Separator => "Separator",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for PropType {
    type GlibType = ffi::IBusPropType;

    #[inline]
fn into_glib(self) -> ffi::IBusPropType {
match self {
            Self::Normal => ffi::PROP_TYPE_NORMAL,
            Self::Toggle => ffi::PROP_TYPE_TOGGLE,
            Self::Radio => ffi::PROP_TYPE_RADIO,
            Self::Menu => ffi::PROP_TYPE_MENU,
            Self::Separator => ffi::PROP_TYPE_SEPARATOR,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::IBusPropType> for PropType {
    #[inline]
unsafe fn from_glib(value: ffi::IBusPropType) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::PROP_TYPE_NORMAL => Self::Normal,
            ffi::PROP_TYPE_TOGGLE => Self::Toggle,
            ffi::PROP_TYPE_RADIO => Self::Radio,
            ffi::PROP_TYPE_MENU => Self::Menu,
            ffi::PROP_TYPE_SEPARATOR => Self::Separator,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for PropType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ibus_prop_type_get_type()) }
    }
}

impl glib::HasParamSpec for PropType {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
                }
}

impl glib::value::ValueType for PropType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PropType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PropType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<PropType> for glib::Value {
    #[inline]
    fn from(v: PropType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "IBusXEventType")]
pub enum XEventType {
    #[doc(alias = "IBUS_X_EVENT_NOTHING")]
    Nothing,
    #[doc(alias = "IBUS_X_EVENT_KEY_PRESS")]
    KeyPress,
    #[doc(alias = "IBUS_X_EVENT_KEY_RELEASE")]
    KeyRelease,
    #[doc(alias = "IBUS_X_EVENT_OTHER")]
    Other,
    #[doc(alias = "IBUS_X_EVENT_EVENT_LAST")]
    EventLast,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for XEventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "XEventType::{}", match *self {
            Self::Nothing => "Nothing",
            Self::KeyPress => "KeyPress",
            Self::KeyRelease => "KeyRelease",
            Self::Other => "Other",
            Self::EventLast => "EventLast",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl IntoGlib for XEventType {
    type GlibType = ffi::IBusXEventType;

    #[inline]
fn into_glib(self) -> ffi::IBusXEventType {
match self {
            Self::Nothing => ffi::IBUS_X_EVENT_NOTHING,
            Self::KeyPress => ffi::IBUS_X_EVENT_KEY_PRESS,
            Self::KeyRelease => ffi::IBUS_X_EVENT_KEY_RELEASE,
            Self::Other => ffi::IBUS_X_EVENT_OTHER,
            Self::EventLast => ffi::IBUS_X_EVENT_EVENT_LAST,
            Self::__Unknown(value) => value,
}
}
}

#[doc(hidden)]
impl FromGlib<ffi::IBusXEventType> for XEventType {
    #[inline]
unsafe fn from_glib(value: ffi::IBusXEventType) -> Self {
        skip_assert_initialized!();
        
match value {
            ffi::IBUS_X_EVENT_NOTHING => Self::Nothing,
            ffi::IBUS_X_EVENT_KEY_PRESS => Self::KeyPress,
            ffi::IBUS_X_EVENT_KEY_RELEASE => Self::KeyRelease,
            ffi::IBUS_X_EVENT_OTHER => Self::Other,
            ffi::IBUS_X_EVENT_EVENT_LAST => Self::EventLast,
            value => Self::__Unknown(value),
}
}
}

impl StaticType for XEventType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::ibus_xevent_type_get_type()) }
    }
}

impl glib::HasParamSpec for XEventType {
                type ParamSpec = glib::ParamSpecEnum;
                type SetValue = Self;
                type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;
    
                fn param_spec_builder() -> Self::BuilderFn {
                    |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
                }
}

impl glib::value::ValueType for XEventType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for XEventType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        skip_assert_initialized!();
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for XEventType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<XEventType> for glib::Value {
    #[inline]
    fn from(v: XEventType) -> Self {
        skip_assert_initialized!();
        ToValue::to_value(&v)
    }
}
