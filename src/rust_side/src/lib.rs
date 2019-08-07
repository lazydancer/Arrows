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
