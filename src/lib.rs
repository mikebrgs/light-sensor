// Local modules
pub mod i2c;

// Public imports
use embedded_hal::i2c::I2c;

// Local imports
pub use i2c::Address;


#[derive(Debug)]
pub enum LightSensorError {
    ConversionError,
    IOError
}


pub struct LightSensor<I2C> {
    dev: i2c::LightSensorI2c<I2C>
}

impl<I2C: I2c> LightSensor<I2C> {
    pub fn new(dev: I2C, address: Address) -> Self {
        let i2c_wrapper = i2c::LightSensorI2c::new(dev, address.into());
        LightSensor{dev: i2c_wrapper}
    }

    pub fn build(dev: I2C, address: Address) -> LightSensor<I2C> {
        let mut sensor = Self::new(dev, address.into());

        sensor.dev.set_shutdown(i2c::Shutdown::PowerOn).unwrap();
        sensor.dev.set_gain(i2c::Gain::X1_4).unwrap();
        sensor.dev.set_integration_time(i2c::IntegrationTime::Ms50).unwrap();

        sensor
    }

    pub fn convert_to_lux(&mut self, raw: u16) -> Result<f32, LightSensorError> {
        const LX_BIT: f32 = 0.0288;

        let gain = self.dev.get_gain().unwrap();
        let integration_time = self.dev.get_integration_time().unwrap();

        let float_raw: f32 = raw.into();
        let mut lux = match integration_time {
            i2c::IntegrationTime::Ms25 => float_raw * LX_BIT * 4.0,
            i2c::IntegrationTime::Ms50 => float_raw * LX_BIT * 2.0,
            i2c::IntegrationTime::Ms100 => float_raw * LX_BIT,
            i2c::IntegrationTime::Ms200 => float_raw * LX_BIT / 2.0,
            i2c::IntegrationTime::Ms400 => float_raw * LX_BIT / 4.0,
            i2c::IntegrationTime::Ms800 => float_raw * LX_BIT / 8.0
        };

        lux = match gain {
            i2c::Gain::X1 => lux * 2.0,
            i2c::Gain::X2 => lux,
            i2c::Gain::X1_4 => lux * 8.0,
            i2c::Gain::X1_8 => lux * 16.0
        };

        Ok(lux)
    }

    pub fn convert_from_lux(&mut self, lux: f32) -> Result<u16, LightSensorError> {
        const LX_BIT: f32 = 0.0288;
        let gain = self.dev.get_gain().unwrap();
        let integration_time = self.dev.get_integration_time().unwrap();

        let mut lux = match gain {
            i2c::Gain::X1 => lux / 2.0,
            i2c::Gain::X2 => lux,
            i2c::Gain::X1_4 => lux / 8.0,
            i2c::Gain::X1_8 => lux / 16.0
        };

        lux = match integration_time {
            i2c::IntegrationTime::Ms25 => lux / (LX_BIT * 4.0),
            i2c::IntegrationTime::Ms50 => lux / (LX_BIT * 2.0),
            i2c::IntegrationTime::Ms100 => lux / (LX_BIT),
            i2c::IntegrationTime::Ms200 => lux / (LX_BIT * 2.0),
            i2c::IntegrationTime::Ms400 => lux / (LX_BIT * 4.0),
            i2c::IntegrationTime::Ms800 => lux / (LX_BIT * 8.0)
        };

        Ok(lux as u16)

    }

    pub fn compensate_lux(&mut self, lux: f32) -> Result<f32, LightSensorError> {
        if lux > 1000. {
            return Ok(0.00000000000060135 * lux.powi(4)
                - 0.0000000093924 * lux.powi(3)
                + 0.000081488 * lux.powi(2)
                + 1.0023 * lux)
        }
        Ok(lux)
    }

    pub fn get_ambient_light_lux(&mut self) -> Result<f32, LightSensorError> {
        let raw_lux = self.dev.get_ambient_light_output().unwrap();
        let lux = self.convert_to_lux(raw_lux).unwrap();
        let lux = self.compensate_lux(lux).unwrap();

        Ok(lux)
    }

    pub fn get_white_light_lux(&mut self) -> Result<f32, LightSensorError> {
        let raw_lux = self.dev.get_white_light_output().unwrap();
        let lux = self.convert_to_lux(raw_lux).unwrap();
        let lux = self.compensate_lux(lux).unwrap();

        Ok(lux)
    }

}


#[cfg(test)]
mod tests {
    // Local imports
    use super::*;
    use i2c::constants;

    // Public imports
    use embedded_hal_mock::eh1::i2c::{Mock as I2cMock, Transaction as I2cTransaction};


    #[test]
    fn start_light_sensor() {
        let address: u8 = constants::addresses::DEFAULT;
        let expectations = [
            I2cTransaction::write_read(address, vec![constants::registers::SETTING_REG], vec![0x00, 0x00]),
            I2cTransaction::write(address, vec![constants::registers::SETTING_REG, 0x00, 0x00]),
            I2cTransaction::write_read(address, vec![constants::registers::SETTING_REG], vec![0x00, 0x00]),
            I2cTransaction::write(address, vec![constants::registers::SETTING_REG, 0x18, 0x00]),
            I2cTransaction::write_read(address, vec![constants::registers::SETTING_REG], vec![0x18, 0x00]),
            I2cTransaction::write(address, vec![constants::registers::SETTING_REG, 0x1A, 0x00]),
        ];
        let i2c = I2cMock::new(&expectations);
        let mut i2c_clone = i2c.clone();

        let _ = LightSensor::build(i2c, Address::Default);

        i2c_clone.done();
    }

}