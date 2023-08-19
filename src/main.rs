use std::fs::File;
use std::io::{Result,Read,Write, Error,ErrorKind};
use std::env;

const DEVICE:&str="/sys/class/backlight/amdgpu_bl1/brightness";

fn get_value(file:&mut File) -> Result<u8>
{
    let mut value = String::new();
    file.read_to_string(&mut value)?;
    let value : String = value.chars().filter(|c|{c.is_digit(10)}).collect();
    u8::from_str_radix(&value,10).map_err(|_| Error::new(ErrorKind::InvalidData,"can't read current value"))
}

fn set_value(file:&mut File, value:u8) -> Result<()>
{
    let value = format!("{}\n", value);
    file.write_all(value.as_bytes())
}

fn main() -> Result <()> {
    let diff : String = env::args().nth(1).unwrap_or("0".to_owned());
    let diff = i32::from_str_radix(&diff,10).unwrap_or(0);
    let mut file = File::options().write(true).read(true).open(DEVICE)?;
    let value = get_value(&mut file)?;
    let value = value as i32 + diff;
    let value = std::cmp::min(value, 255);
    let value = std::cmp::max(value, 0);
    set_value(&mut file, value as u8)?;

    Ok(())
}
