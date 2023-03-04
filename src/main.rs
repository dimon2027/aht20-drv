use std::ffi;

const O_RDWR: i32 = 2;

struct Aht20 {
    fd: i32,
}

impl Aht20 {
    fn open(&mut self, pathname: &str) -> bool {
        let pathname = ffi::CString::new(pathname).expect("Failed to create a CString");

        let fd = unsafe { open(pathname.as_ptr(), O_RDWR) };

        if fd != -1 {
            self.fd = fd;
        } else {
            return false;
        }

        true
    }
}

fn main() {
    // /dev/i2c-1
    let mut drv = Aht20 { fd: -1 };
    let pathname = "/dev/i2c-1";

    let res = drv.open(pathname);

    if res {
        println!("The device has been opened, fd = {}", drv.fd);
    } else {
        println!("Failed to open the device!");
    }
}

extern "C" {
    fn open(pathname: *const ffi::c_char, flags: i32) -> i32;
}
