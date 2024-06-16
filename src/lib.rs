// Local modules
pub mod i2c;

// Public imports
use embedded_hal::{delay::DelayNs, i2c::I2c};

// Local imports
pub use i2c::Address;


// Constants for conversion
const C0: f64 = 1.0023;
const C1: f64 = 8.1488e-05;
const C2: f64 = -9.3924e-09;
const C3: f64 = 6.0135e-13;


#[derive(Debug)]
pub enum LightSensorError {
    ConversionError,
    IOError
}


pub struct LightSensor<I2C, Delay> {
    dev: i2c::LightSensorI2c<I2C, Delay>
}

impl<I2C: I2c, Delay: DelayNs> LightSensor<I2C, Delay> {
    pub fn new(dev: I2C, address: Address, delay: Delay) -> Self {
        let i2c_wrapper = i2c::LightSensorI2c::new(dev, address.into(), delay);
        LightSensor{dev: i2c_wrapper}
    }

    pub fn build(dev: I2C, address: Address, delay: Delay) -> LightSensor<I2C, Delay> {
        let mut sensor = Self::new(dev, address.into(), delay);

        sensor.dev.set_shutdown(i2c::Shutdown::PowerOn).unwrap();
        sensor.dev.set_gain(i2c::Gain::X1_4).unwrap();
        sensor.dev.set_integration_time(i2c::IntegrationTime::Ms50).unwrap();

        sensor
    }

    pub fn convert_raw_to_lux(&mut self, raw: u16) -> Result<f32, LightSensorError> {
        const LX_BIT: f64 = 0.0288;

        let gain = self.dev.get_gain().unwrap();
        let integration_time = self.dev.get_integration_time().unwrap();

        let it_factor: f64 = match integration_time {
            i2c::IntegrationTime::Ms25 => 4.0,
            i2c::IntegrationTime::Ms50 => 2.0,
            i2c::IntegrationTime::Ms100 => 1.0,
            i2c::IntegrationTime::Ms200 => 0.5,
            i2c::IntegrationTime::Ms400 => 0.25,
            i2c::IntegrationTime::Ms800 => 0.125
        };

        // lux = 
        let gain_factor: f64 = match gain {
            i2c::Gain::X2 => 1.0,
            i2c::Gain::X1 => 2.0,
            i2c::Gain::X1_4 => 8.0,
            i2c::Gain::X1_8 => 16.0
        };

        let mut lux = LX_BIT * it_factor * gain_factor * f64::from(raw);
        println!("raw: {}", raw);
        println!("it_factor {}", it_factor);
        println!("gain_factor {}", gain_factor);
        println!("lux {}", lux);

        match gain {
            i2c::Gain::X1_4 | i2c::Gain::X1_8 => {
                // Compensate high lux
                if lux > 1000.0 {
                    lux = C3 * lux.powi(4) - C2 * lux.powi(3) + C1 * lux.powi(2) + C0 * lux;
                }
            },
            _ => (),
        };

        println!("raw: {}", raw);
        println!("it_factor {}", it_factor);
        println!("gain_factor {}", gain_factor);
        println!("lux {}", lux);

        Ok(lux as f32)

    }

    pub fn get_ambient_light_lux(&mut self) -> Result<f32, LightSensorError> {
        let raw_lux = self.dev.get_ambient_light_output().unwrap();

        let lux = self.convert_raw_to_lux(raw_lux).unwrap();
        Ok(lux)

        // let gain = self.dev.get_gain().unwrap();
        // let it = self.dev.get_integration_time().unwrap();

        // let factor = get_lux_raw_conversion_factor(it, gain);
        // let lux = f64::from(raw_lux) * f64::from(factor);
        // if (gain == i2c::Gain::X1_4 || gain == i2c::Gain::X1_8) && lux > 1000.0 {
        //     Ok(compensate_high_lux(lux) as f32)
        // } else {
        //     Ok(lux as f32)
        // }

    }

    pub fn get_white_light(&mut self) -> Result<f32, LightSensorError> {
        let white_light = self.dev.get_white_light_output().unwrap();

        Ok(white_light as f32)
    }

}


// fn get_lux_raw_conversion_factor(it: i2c::IntegrationTime, gain: i2c::Gain) -> f32 {
//     let gain_factor = match gain {
//         i2c::Gain::X2 => 1.0,
//         i2c::Gain::X1 => 2.0,
//         i2c::Gain::X1_4 => 8.0,
//         i2c::Gain::X1_8 => 16.0,
//     };
//     let it_factor = match it {
//         i2c::IntegrationTime::Ms800 => 0.0036,
//         i2c::IntegrationTime::Ms400 => 0.0072,
//         i2c::IntegrationTime::Ms200 => 0.0144,
//         i2c::IntegrationTime::Ms100 => 0.0288,
//         i2c::IntegrationTime::Ms50 => 0.0576,
//         i2c::IntegrationTime::Ms25 => 0.1152,
//     };
//     gain_factor * it_factor
// }


// pub fn compensate_high_lux(lux: f64) -> f64 {
//     C3 * lux.powi(4)
//         - C2 * lux.powi(3)
//         + C1 * lux.powi(2)
//         + C0 * lux
// }


#[cfg(test)]
mod tests {
    // Local imports
    use super::*;
    use i2c::constants;

    // Public imports
    use embedded_hal_mock::eh1::{delay::NoopDelay, i2c::{Mock as I2cMock, Transaction as I2cTransaction}};


    #[test]
    fn start_light_sensor() {
        let address: u8 = constants::addresses::DEFAULT;
        let expectations = [
            I2cTransaction::write_read(address, vec![constants::registers::SETTING_REG], vec![0x00, 0x00]),
            I2cTransaction::write(address, vec![constants::registers::SETTING_REG, 0x00, 0x00]),
            I2cTransaction::write_read(address, vec![constants::registers::SETTING_REG], vec![0x00, 0x00]),
            I2cTransaction::write(address, vec![constants::registers::SETTING_REG, 0x00, 0x18]),
            I2cTransaction::write_read(address, vec![constants::registers::SETTING_REG], vec![0x00, 0x18]),
            I2cTransaction::write(address, vec![constants::registers::SETTING_REG, 0x00, 0x1A]),
        ];
        let i2c = I2cMock::new(&expectations);
        let mut i2c_clone = i2c.clone();

        let _ = LightSensor::build(i2c, Address::Default, NoopDelay::new()) ;

        i2c_clone.done();
    }

}