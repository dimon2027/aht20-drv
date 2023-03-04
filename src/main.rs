use std::{ffi, thread, time};

const O_RDWR: i32 = 2;
const I2C_SLAVE: u64 = 0x0703;
const AHT20_ADDR: i32 = 0x38;

const INIT_COMMAND: i32 = 0xBE;
const MEASURE_COMMAND: i32 = 0xAC;

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

    fn close(&mut self) -> bool {
        if self.fd == -1 {
            return true;
        };

        let res = unsafe { close(self.fd) };

        res == 0
    }

    fn temperature(&self) -> bool {
        let res = unsafe { ioctl(self.fd, I2C_SLAVE, AHT20_ADDR) };
        if res < 0 {
            println!("error temperature 1");
            return false;
        };

        let mut buf: [u8; 3] = [0; 3];
        buf[0] = 0xAC;
        buf[1] = 0x33;

        let res = unsafe { write(self.fd, buf.as_ptr() as *const ffi::c_void, 2) };
        if res != 2 {
            println!("error temperature 2, failed to write, res = {}", res);
            return false;
        }

        thread::sleep(time::Duration::from_millis(100));

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

    let res = drv.temperature();
    println!("The result of ioctl: {}", res);

    let res = drv.close();

    if res {
        println!("The devices has been successfully closed!");
    } else {
        println!("Failed to close the device!");
    }
}

extern "C" {
    fn open(pathname: *const ffi::c_char, flags: i32) -> i32;
    fn close(fd: i32) -> i32;
    fn write(fd: ffi::c_int, buf: *const ffi::c_void, count: usize) -> isize;

    fn ioctl(fd: ffi::c_int, request: u64, ...) -> ffi::c_int;
}
