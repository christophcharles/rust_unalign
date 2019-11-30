#![no_std]

pub enum Result {
    Ok(u32),
    Err(ErrorTest)
}

pub enum ErrorTest {
    InvalidPin,
    Err2
}

pub fn test(reg: u32, pin: u32) -> Result {
    if pin < 32 {
        return Result::Ok(reg);
    }
    Result::Err(ErrorTest::InvalidPin)
}

