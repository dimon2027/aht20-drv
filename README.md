# aht20-drv
Linux driver for AHT20 temperature and humidity sensor, can be used on Raspberry Pi 3 (tested on x64 bit Bullseye)

## Usage

fn main() {
    let mut drv = Aht20 { fd: -1 };
    let pathname = "/dev/i2c-1";

    let res = drv.init(pathname, 0); // Default address (0x38) will be used

    if !res {
        println!("Failed to open the device!");
        return;
    }

    let (t, h, res) = drv.get_temp_and_hum();
    if !res {
        println!("Failed to read temperature and humidity!");
        return;
    }

    println!("The temperatrue and humidity are: {}, {}", t, h);

    drv.close();
}