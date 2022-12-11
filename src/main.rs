use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use anyhow::{anyhow, Result};
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use lazy_static::lazy_static;

mod temperature;
use temperature::Temperature;

// Номер шины
const I2C_PATH: &str = "/dev/i2c-1";
// Адрес винтелятора
const ADDRESS: u16 = 0x1a;
// Минимальная температура. Если температура меньше или равна то скорость вентилятора 0
const MIN_TEMPERATURE: u8 = 50;
// Максимальная температура.
const MAX_TEMPERATURE: u8 = 65;
// Частота обновления показаний в секундах
const TIMEOUT_SEC: u8 = 1;

lazy_static! {
    static ref FAN: Mutex<LinuxI2CDevice> =
        Mutex::new(LinuxI2CDevice::new(I2C_PATH, ADDRESS).unwrap());
    static ref TEMPERATURE: Mutex<Temperature> = Mutex::new(Temperature::new().unwrap());
}

fn set_fan_speed(mut speed: u8) -> Result<()> {
    speed = if speed > 100 { 100 } else { speed };
    let mut fan = FAN.lock().map_err(|err| anyhow!("{err}"))?;
    fan.smbus_write_byte(speed)?;
    Ok(())
}

fn fan_speed_calculation(temp: u8) -> u8 {
    if temp < MIN_TEMPERATURE {
        return 0;
    } else if temp > MAX_TEMPERATURE {
        return 100;
    }

    ((temp - MIN_TEMPERATURE) as f32 * (100_f32 / (MAX_TEMPERATURE - MIN_TEMPERATURE) as f32)) as u8
}

fn processing_temperature() -> Result<()> {
    let temperature = TEMPERATURE.lock().map_err(|err| anyhow!("{err}"))?.get();

    let speed = fan_speed_calculation(temperature);
    set_fan_speed(speed)?;
    println!("Показатели: {temperature}° {speed}");
    Ok(())
}

fn main() {
    loop {
        if let Err(err) = processing_temperature() {
            println!("Error: {err}");
        }

        sleep(Duration::from_secs(TIMEOUT_SEC.into()));
    }
}
