#[doc(alias = "ibus_free_strv")]
pub fn free_strv(strv: &mut str) {
    assert_initialized_main_thread!();
    unsafe { ffi::ibus_free_strv(strv.as_mut_ptr().cast()) }
}
