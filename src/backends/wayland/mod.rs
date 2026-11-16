unsafe extern "C" {
    fn wayland_init();
}
pub fn init() {
    unsafe { wayland_init(); }
}
