use crate::EmojiData;
use glib::object::IsA;
use glib::translate::ToGlibPtr;
use glib::{GStringPtr, SList};

pub trait EmojiDataExtManual: 'static {
    fn save(path: &str, list: Vec<EmojiData>);
    fn set_annotations(&self, annotations: Vec<String>);
}

impl<O: IsA<EmojiData>> EmojiDataExtManual for O {
    fn save(path: &str, list: Vec<EmojiData>) {
        let mut slist: SList<EmojiData> = list.into_iter().collect();
        unsafe { ffi::ibus_emoji_data_save(path.as_ptr().cast(), slist.as_mut_ptr()) }
    }
    fn set_annotations(&self, annotations: Vec<String>) {
        let mut slist = annotations
            .into_iter()
            .map(|mut s| unsafe {
                let yes: GStringPtr = std::mem::transmute(
                    std::ptr::NonNull::new(s.as_mut_ptr()).expect("Null string pointer"),
                );
                yes
            })
            .collect::<SList<_>>();

        unsafe {
            ffi::ibus_emoji_data_set_annotations(
                self.as_ref().to_glib_none().0,
                slist.as_mut_ptr(),
            );
        }
    }
}
