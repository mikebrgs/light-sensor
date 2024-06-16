// Local modules
pub mod constants;

// Public imports
use embedded_hal::{i2c::I2c, delay::DelayNs};

use byteorder::{ByteOrder, LittleEndian};

// Local imports


#[derive(Debug)]
pub enum LightSensorI2cError {
    ConversionError,
    IOError
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Gain {
    X1 = 0b00,  // 1x gain
    X2 = 0b01,  // 2x gain
    X1_8 = 0b10,  // 1/8x gain
    X1_4 = 0b11  // 1/4x gain
}

impl From<u16> for Gain {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::X1,
            1 => Self::X2,
            2 => Self::X1_8,
            3 => Self::X1_4,
            _ => panic!("Not expected")
        }
    }
}

impl From<Gain> for u16 {
    fn from(item: Gain) -> u16 {
        match item {
            Gain::X1 => 0,
            Gain::X2 => 1,
            Gain::X1_8 => 2,
            Gain::X1_4 => 3,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum IntegrationTime {
    Ms25 = 0b1100,
    Ms50 = 0b1000,
    Ms100 = 0b0000,
    Ms200 = 0b0001,
    Ms400 = 0b0010,
    Ms800 = 0b0011
}

impl From<u16> for IntegrationTime {
    fn from(item: u16) -> Self {
        match item {
            0b1100 => Self::Ms25,
            0b1000 => Self::Ms50,
            0b0000 => Self::Ms100,
            0b0001 => Self::Ms200,
            0b0010 => Self::Ms400,
            0b0011 => Self::Ms800,
            _ => panic!("Not expected")
        }
    }
}

impl From<IntegrationTime> for u16 {
    fn from(item: IntegrationTime) -> u16 {
        match item {
            IntegrationTime::Ms25 => 0b1100,
            IntegrationTime::Ms50 => 0b1000,
            IntegrationTime::Ms100 => 0b0000,
            IntegrationTime::Ms200 => 0b0001,
            IntegrationTime::Ms400 => 0b0010,
            IntegrationTime::Ms800 => 0b0011,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PersistenceProtectNumber {
    N1 = 0b00,
    N2 = 0b01,
    N4 = 0b10,
    N8 = 0b11
}

impl From<u16> for PersistenceProtectNumber {
    fn from(item: u16) -> Self {
        match item {
            0b00 => Self::N1,
            0b01 => Self::N2,
            0b10 => Self::N4,
            0b11 => Self::N8,
            _ => panic!("Not expected")
        }
    }
}

impl From<PersistenceProtectNumber> for u16 {
    fn from(item: PersistenceProtectNumber) -> u16 {
        match item {
            PersistenceProtectNumber::N1 => 0b00,
            PersistenceProtectNumber::N2 => 0b01,
            PersistenceProtectNumber::N4 => 0b10,
            PersistenceProtectNumber::N8 => 0b11,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PowerSavingMode {
    M1 = 0b00,  // Fastest, most current
    M2 = 0b01,
    M3 = 0b10,
    M4 = 0b11  // Slowest, least current
}

impl From<u16> for PowerSavingMode {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::M1,
            1 => Self::M2,
            2 => Self::M3,
            3 => Self::M4,
            _ => panic!("Not expected")
        }
    }
}

impl From<PowerSavingMode> for u16 {
    fn from(item: PowerSavingMode) -> u16 {
        match item {
            PowerSavingMode::M1 => 0,
            PowerSavingMode::M2 => 1,
            PowerSavingMode::M3 => 2,
            PowerSavingMode::M4 => 3,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PowerSavingModeEnable {
    Disable = 0b0,
    Enable = 0b1
}

impl From<u16> for PowerSavingModeEnable {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::Disable,
            1 => Self::Enable,
            _ => panic!("Not expected")
        }
    }
}

impl From<PowerSavingModeEnable> for u16 {
    fn from(item: PowerSavingModeEnable) -> u16 {
        match item {
            PowerSavingModeEnable::Disable => 0,
            PowerSavingModeEnable::Enable => 1,
        }
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum InterruptEnable {
    Disable = 0b0,
    Enable = 0b1,
}

impl From<u16> for InterruptEnable {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::Disable,
            1 => Self::Enable,
            _ => panic!("Not expected")
        }
    }
}

impl From<InterruptEnable> for u16 {
    fn from(item: InterruptEnable) -> u16 {
        match item {
            InterruptEnable::Disable => 0,
            InterruptEnable::Enable => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Shutdown {
    PowerOn,
    PowerOff
}

impl From<u16> for Shutdown {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::PowerOn,
            1 => Self::PowerOff,
            _ => panic!("Not expected")
        }
    }
}

impl From<Shutdown> for u16 {
    fn from(item: Shutdown) -> u16 {
        match item {
            Shutdown::PowerOn => 0,
            Shutdown::PowerOff => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Threshold {
    NotExceeded = 0,
    Exceeded = 1,
}

impl From<u16> for Threshold {
    fn from(item: u16) -> Self {
        match item {
            0 => Self::NotExceeded,
            1 => Self::Exceeded,
            _ => panic!("Not expected")
        }
    }
}

impl From<Threshold> for u16 {
    fn from(item: Threshold) -> u16 {
        match item {
            Threshold::NotExceeded => 0,
            Threshold::Exceeded => 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Address {
    Default,
    Alternative,
}

impl From<Address> for u8 {
    fn from(item: Address) -> u8 {
        match item {
            Address::Default => constants::addresses::DEFAULT,
            Address::Alternative => constants::addresses::ALTERNATIVE,
        }
    }
}


/// I2C wrapper for LightSensor
pub struct LightSensorI2c<I2C, Delay>{
    i2c: I2C,
    address: u8,
    delay: Delay
}


impl<I2C: I2c, Delay: DelayNs> LightSensorI2c<I2C, Delay>{
    /// Create new LightSensorI2c.
    pub fn new(i2c: I2C, address: u8, delay: Delay) -> LightSensorI2c<I2C, Delay> {
        LightSensorI2c { i2c, address, delay}
    }

    pub fn get_gain(&mut self) -> Result<Gain, LightSensorI2cError> {
        let state = read_and_convert_to_u16(self, constants::registers::SETTING_REG).unwrap();
        let gain = clip_u16(state, 11, 2);

        Ok(gain.into())
    }

    pub fn set_gain(&mut self, gain: Gain) -> Result<(), LightSensorI2cError> {
        let old_state = read_and_convert_to_u16(self, constants::registers::SETTING_REG).unwrap();
        let new_state = insert_u16(old_state, 11, 2, gain.into());
        convert_and_write_u16(self, constants::registers::SETTING_REG, new_state).unwrap();

        Ok(())
    }

    pub fn get_integration_time(&mut self) -> Result<IntegrationTime, LightSensorI2cError> {
        let state = read_and_convert_to_u16(self, constants::registers::SETTING_REG).unwrap();
        let integration_time = clip_u16(state, 6, 4);

        Ok(integration_time.into())
    }

    pub fn set_integration_time(&mut self, integration_time: IntegrationTime) -> Result<(), LightSensorI2cError> {
        let old_state = read_and_convert_to_u16(self, constants::registers::SETTING_REG).unwrap();
        let new_state = insert_u16(old_state, 6, 4, integration_time.into());
        convert_and_write_u16(self, constants::registers::SETTING_REG, new_state).unwrap();

        Ok(())
    }

    pub fn get_persist_protect_number(&mut self) -> Result<PersistenceProtectNumber, LightSensorI2cError> {
        let state = read_and_convert_to_u16(self, constants::registers::SETTING_REG).unwrap();
        let ppn = clip_u16(state, 4, 2);
        
        Ok(ppn.into())
    }

    pub fn set_persist_protect_number(&mut self, persistence_protect_number: PersistenceProtectNumber) -> Result<(), LightSensorI2cError> {
        let old_state = read_and_convert_to_u16(self, constants::registers::SETTING_REG).unwrap();
        let new_state = insert_u16(old_state, 4, 2, persistence_protect_number.into());
        convert_and_write_u16(self, constants::registers::SETTING_REG, new_state).unwrap();

        Ok(())
    }

    pub fn get_interrupt_enabled(&mut self) -> Result<InterruptEnable, LightSensorI2cError> {
        let state = read_and_convert_to_u16(self, constants::registers::SETTING_REG).unwrap();
        let interrupt = clip_u16(state, 1, 1);

        Ok(interrupt.into())
    }

    pub fn set_interrupt_enabled(&mut self, interrupt_enable: InterruptEnable) -> Result<(), LightSensorI2cError> {
        let old_state = read_and_convert_to_u16(self, constants::registers::SETTING_REG).unwrap();
        let new_sate = insert_u16(old_state, 1, 1, interrupt_enable.into());
        convert_and_write_u16(self, constants::registers::SETTING_REG, new_sate).unwrap();
        
        Ok(())
    }

    pub fn get_shutdown(&mut self) -> Result<Shutdown, LightSensorI2cError> {
        let state = read_and_convert_to_u16(self, constants::registers::SETTING_REG).unwrap();
        let shut_down = clip_u16(state, 0, 1);

        Ok(shut_down.into())
    }

    pub fn set_shutdown(&mut self, shutdown: Shutdown) -> Result<(), LightSensorI2cError> {
        let old_state = read_and_convert_to_u16(self, constants::registers::SETTING_REG).unwrap();
        let new_sate = insert_u16(old_state, 0, 1, shutdown.clone().into());
        convert_and_write_u16(self, constants::registers::SETTING_REG, new_sate).unwrap();
        
        match shutdown {
            Shutdown::PowerOn => self.delay.delay_ms(4),
            _ => (),
        }

        Ok(())
    }

    pub fn get_high_threshold_window(&mut self) -> Result<u16, LightSensorI2cError> {
        let high_threhold_window = read_and_convert_to_u16(self, constants::registers::H_THRESH_REG).unwrap();

        Ok(high_threhold_window)
    }

    pub fn set_high_threshold_window(&mut self, threshold: u16) -> Result<(), LightSensorI2cError> {
        convert_and_write_u16(self, constants::registers::H_THRESH_REG, threshold).unwrap();

        Ok(())
    }

    pub fn get_low_threshold_window(&mut self) -> Result<u16, LightSensorI2cError> {
        let low_threshold_window = read_and_convert_to_u16(self, constants::registers::L_THRESH_REG).unwrap();

        Ok(low_threshold_window)
    }

    pub fn set_low_threshold_window(&mut self, threshold: u16) -> Result<(), LightSensorI2cError> {
        convert_and_write_u16(self, constants::registers::L_THRESH_REG, threshold).unwrap();

        Ok(())
    }

    pub fn get_power_saving_mode(&mut self) -> Result<PowerSavingMode, LightSensorI2cError> {
        let state = read_and_convert_to_u16(self, constants::registers::POWER_SAVE_REG).unwrap();
        let mode = clip_u16(state, 1, 2);
        
        Ok(mode.into())
    }

    pub fn set_power_saving_mode(&mut self, mode: PowerSavingMode) -> Result<(), LightSensorI2cError> {
        let old_state = read_and_convert_to_u16(self, constants::registers::POWER_SAVE_REG).unwrap();
        let new_state = insert_u16(old_state, 1, 2, mode.into());
        convert_and_write_u16(self, constants::registers::POWER_SAVE_REG, new_state).unwrap();

        Ok(())
    }

    pub fn get_power_saving_mode_enabled(&mut self) -> Result<PowerSavingModeEnable, LightSensorI2cError> {
        let state = read_and_convert_to_u16(self, constants::registers::POWER_SAVE_REG).unwrap();
        let enabled = clip_u16(state, 0, 1);

        Ok(enabled.into())
    }

    pub fn set_power_saving_mode_enabled(&mut self, enable: PowerSavingModeEnable) -> Result<(), LightSensorI2cError> {
        let old_state = read_and_convert_to_u16(self, constants::registers::POWER_SAVE_REG).unwrap();
        let new_state = insert_u16(old_state, 0, 1, enable.into());
        convert_and_write_u16(self, constants::registers::POWER_SAVE_REG, new_state).unwrap();

        Ok(())
    }

    pub fn get_ambient_light_output(&mut self) -> Result<u16, LightSensorI2cError> {
        let alo = read_and_convert_to_u16(self, constants::registers::AMBIENT_LIGHT_DATA_REG).unwrap();

        Ok(alo)
    } 

    pub fn get_white_light_output(&mut self) -> Result<u16, LightSensorI2cError> {
        let wlo = read_and_convert_to_u16(self, constants::registers::WHITE_LIGHT_DATA_REG).unwrap();
        
        Ok(wlo)
    }

    pub fn get_low_threshold_exceeded(&mut self) -> Result<Threshold, LightSensorI2cError> {
        let state = read_and_convert_to_u16(self, constants::registers::INTERRUPT_REG).unwrap();
        let threshold_exceeded = clip_u16(state, 15, 1);
        
        Ok(threshold_exceeded.into())
    }

    pub fn get_high_threshold_exceeded(&mut self) -> Result<Threshold, LightSensorI2cError> {
        let state = read_and_convert_to_u16(self, constants::registers::INTERRUPT_REG).unwrap();
        let threshold_exceeded = clip_u16(state, 14, 1);
        
        Ok(threshold_exceeded.into())
    }

}


/// Get value from a specific register in sensor.
pub fn read_from_register<I2C: I2c, Delay: DelayNs>(dev: &mut LightSensorI2c<I2C, Delay> , register: u8, buffer: &mut [u8]) -> Result<(), LightSensorI2cError> {
    match dev.i2c.write_read(dev.address, &[register], buffer) {
        Ok(_) => Ok(()),
        Err(_) => Err(LightSensorI2cError::IOError)
    }
}

/// Set value from a specific register in sensor.
pub fn write_to_register<I2C: I2c, Delay: DelayNs>(dev: &mut LightSensorI2c<I2C, Delay>, register: u8, bytes: &[u8]) -> Result<(), LightSensorI2cError> {
    let mut buffer = Vec::<u8>::with_capacity(1+bytes.len());
    buffer.push(register);
    for value in bytes {
        buffer.push(*value);
    }
    // TODO check if it matches write_bytes
    match dev.i2c.write(dev.address, &buffer) {
        Ok(_) => Ok(()),
        Err(_) => Err(LightSensorI2cError::IOError)
    }
}


fn read_and_convert_to_u16<I2C: I2c, Delay: DelayNs>(dev: &mut LightSensorI2c<I2C, Delay>, register: u8) -> Result<u16, LightSensorI2cError> {
    let mut buffer = [0u8; 2];
    read_from_register(dev, register, &mut buffer).unwrap();
    let state = convert_buffer_to_u16(&buffer).unwrap();
    Ok(state)
}

fn convert_and_write_u16<I2C: I2c, Delay: DelayNs>(dev: &mut LightSensorI2c<I2C, Delay>, register: u8, state: u16) -> Result<(), LightSensorI2cError> {
    let mut buffer = [0u8; 2];
    convert_u16_to_buffer(&mut buffer, state).unwrap();
    write_to_register(dev, register, &buffer).unwrap();
    Ok(())
}

fn convert_buffer_to_u16(buffer: &[u8]) -> Result<u16, ()> {
let num = LittleEndian::read_u16(buffer);
Ok(num)
}

fn convert_u16_to_buffer(buffer: &mut [u8], num: u16) -> Result<(), ()> {
LittleEndian::write_u16(buffer, num);
Ok(())
}

fn clip_u16(state: u16, trailing_zeros: u16, length: u16) -> u16 {
let mask = create_mask(trailing_zeros, length);
(state & mask) >> trailing_zeros
}

fn insert_u16(state: u16, trailing_zeros: u16, length: u16, value: u16) -> u16 {
let mask = create_mask(trailing_zeros, length);
(state & !mask) | ((value << trailing_zeros) & mask)
}

fn create_mask(trailing_zeros: u16, length: u16) -> u16 {
let mut mask = 0u16;
for _ in 0..length {
    mask = (mask << 1) + 1u16
}
mask << trailing_zeros

}
