use std::{ffi, thread, time};

const O_RDWR: i32 = 2;
const I2C_SLAVE: u64 = 0x0703;
const AHT20_ADDR: i32 = 0x38;

struct Aht20 {
    fd: i32,
}

// TODO: Error handling
// TODO: make some methods private
// TODO: turn this into a library
// TODO: optional: calculate CRC
impl Aht20 {
    fn init(&mut self, pathname: &str, addr: i32) -> bool {
        let res = ffi::CString::new(pathname);
        if !res.is_ok() {
            return false;
        }
        let pathname = res.unwrap();

        let fd = unsafe { open(pathname.as_ptr(), O_RDWR) };

        if fd != -1 {
            self.fd = fd;
        } else {
            return false;
        }

        let addr = if addr > 0 { addr } else { AHT20_ADDR };

        let res = unsafe { ioctl(self.fd, I2C_SLAVE, addr) };
        if res < 0 {
            println!("error failed to send ioctl");
            return false;
        };

        true
    }

    fn close(&mut self) -> bool {
        if self.fd == -1 {
            return true;
        };

        let res = unsafe { close(self.fd) };

        res == 0
    }

    fn get_temp_and_hum(&self) -> (f64, f64, bool) {
        let (status, res) = self.read_status();
        if !res {
            return (0.0, 0.0, false);
        }

        if status & 0b1000 == 0 {
            let res = self.initialize();
            if !res {
                return (0.0, 0.0, false);
            }
        }

        let writebuf: [u8; 3] = [0xAC, 0x33, 0x00];

        let res = unsafe { write(self.fd, writebuf.as_ptr() as *const ffi::c_void, 3) };
        if res != 3 {
            println!("error, failed to write to sensor, res = {}", res);
            return (0.0, 0.0, false);
        }

        thread::sleep(time::Duration::from_millis(80));

        let readbuf: [u8; 7] = [0; 7];
        let res = unsafe { read(self.fd, readbuf.as_ptr() as *const ffi::c_void, 7) };
        if res != 7 as isize {
            println!("error, failed to read from sensor, res = {}", res);
            return (0.0, 0.0, false);
        }

        let status = readbuf[0];
        if status & 0b10000000 != 0 {
            println!("error: bit[7] of status word is not 0!");
            return (0.0, 0.0, false);
        }

        let traw: u32 = (readbuf[3] & 0xF) as u32;
        let traw = (traw << 8) + readbuf[4] as u32;
        let traw = (traw << 8) + readbuf[5] as u32;

        let t: f64 = (traw as f64 / 1048576.0) * 200.0 - 50.0;

        let hraw: u32 = readbuf[1] as u32;
        let hraw = (hraw << 8) + readbuf[2] as u32;
        let hraw = (hraw << 8) + readbuf[3] as u32;
        let hraw = hraw >> 4;

        let h: f64 = hraw as f64 / 1048576.0;

        (t, h, true)
    }

    fn read_status(&self) -> (u8, bool) {
        // Read status word
        let writebuf: [u8; 1] = [0x71];

        let res = unsafe { write(self.fd, writebuf.as_ptr() as *const ffi::c_void, 1) };
        if res != 1 {
            println!("faled to write, res = {}", res);
            return (0, false);
        }

        thread::sleep(time::Duration::from_millis(40));

        let readbuf: [u8; 1] = [0; 1];
        let res = unsafe { read(self.fd, readbuf.as_ptr() as *const ffi::c_void, 1) };
        if res != 1 as isize {
            println!("failed to read, res = {}", res);
            return (0, false);
        }

        println!("The state word is: {}", readbuf[0]);

        (readbuf[0], true)
    }

    fn initialize(&self) -> bool {
        let writebuf: [u8; 3] = [0xBE, 0x08, 0x00];

        let res = unsafe { write(self.fd, writebuf.as_ptr() as *const ffi::c_void, 3) };
        if res != 3 {
            println!("failed to write, res = {}", res);
            return false;
        }

        thread::sleep(time::Duration::from_millis(10));

        true
    }
}

fn main() {
    // /dev/i2c-1
    let mut drv = Aht20 { fd: -1 };
    let pathname = "/dev/i2c-1";

    let res = drv.init(pathname, 0x38);

    if res {
        println!("The device has been opened, fd = {}", drv.fd);
    } else {
        println!("Failed to open the device!");
    }

    let (t, h, res) = drv.get_temp_and_hum();
    println!(
        "The temperatrue, humidity and res are: {}, {}, {}",
        t, h, res
    );

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
    fn read(fd: ffi::c_int, buf: *const ffi::c_void, count: usize) -> isize;
    fn ioctl(fd: ffi::c_int, request: u64, ...) -> ffi::c_int;
}
