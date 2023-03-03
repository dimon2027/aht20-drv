use std::ffi;

fn main() {
    // /dev/i2c-1
    let path_name = ffi::CString::new("/dev/i2c-1").expect("CString::new failed");

    let fd = unsafe {
        let fd = open(path_name.as_ptr(), 2);
        fd
    };

    println!("The devices has been opened: {fd}");
}

extern "C" {
    fn open(pathname: *const ffi::c_char, flags: i32) -> i32;
}


