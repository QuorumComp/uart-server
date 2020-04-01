use std::convert::TryInto;

use serialport::*;

pub fn open(port_name: &str) -> Result<Box<dyn SerialPort>> {
    serialport::new(port_name, 57600).timeout(std::time::Duration::from_secs(60*60*24)).open()
}

pub fn read_byte(port: &mut dyn SerialPort) -> serialport::Result<u8> {
    let mut d: [u8; 1] = [0; 1];
    port.read_exact(&mut d)?;
    Ok(d[0])
}

pub fn write_byte(port: &mut dyn SerialPort, data: u8) -> serialport::Result<()> {
    let d: [u8; 1] = [data; 1];
    port.write_all(&d)?;
    Ok(())
}

pub fn write_vec(port: &mut dyn SerialPort, data: &Vec<u8>) -> serialport::Result<()> {
    write_u16(port, data.len().try_into().unwrap())?;
    port.write_all(data)?;
    Ok(())
}

pub fn read_u16(port: &mut dyn SerialPort) -> Result<u16> {
    let low = read_byte(port)? as u16;
    let high = read_byte(port)? as u16;
    Ok((high << 8) | low)
}

pub fn write_u16(port: &mut dyn SerialPort, value: u16) -> Result<()> {
    let low = value as u8;
    let high = (value >> 8) as u8;
    write_byte(port, low)?;
    write_byte(port, high)?;
    Ok(())
}

pub fn read_string(port: &mut dyn SerialPort) -> Result<String> {
    let mut result = String::new();
    let length = read_u16(port)?;

    for _ in 0..length {
        result.push(read_byte(port)? as char);
    }

    Ok(result)
}
