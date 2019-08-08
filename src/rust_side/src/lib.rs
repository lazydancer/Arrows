use view;

#[no_mangle]
pub extern "C" fn doubl(x: i32) -> i32 {
    x * 2
}

#[no_mangle]
pub extern "C" fn start_sim() {
    println!("{:?}", "starting sim");
    view::start();
}

#[repr(C)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[no_mangle]
pub extern "C" fn length(ptr: *const Pos) -> i32 {
    let pos = unsafe {
        assert!(!ptr.is_null());
        &*ptr
    };
    4
}
