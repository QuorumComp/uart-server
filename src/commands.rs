use serialport::*;

use super::port;

use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(Copy, Clone, FromPrimitive)]
enum CommandIdentifier {
    Identify = 0,
    SendFile = 1,
    RequestChar = 2,
    PrintChar = 3
}

#[derive(Debug)]
pub struct SendFileOptions {
    pub path: String,
    pub offset: u16,
    pub length: u16
}

#[derive(Debug)]
pub enum Command {
    Identify { nonce: u16 },
    SendFile { options: SendFileOptions },
    RequestChar,
    PrintChar { character: char }
}


fn read_identify(port: &mut dyn SerialPort) -> Result<Command> {
    Ok(Command::Identify { nonce: port::read_u16(port)? })
}

fn read_request_char() -> Result<Command> {
    Ok(Command::RequestChar)
}

fn read_print_char(port: &mut dyn SerialPort) -> Result<Command> {
    Ok(Command::PrintChar { character: port::read_byte(port)? as char })
}

fn read_send_file(port: &mut dyn SerialPort) -> Result<Command> {
    let path = port::read_string(port)?;
    let offset = port::read_u16(port)?;
    let length = port::read_u16(port)?;

    let options = SendFileOptions {
        path: path,
        offset: offset,
        length: length
    };

    Ok(Command::SendFile { options: options })
}


pub fn read_command(port: &mut dyn SerialPort, debug: bool) -> Result<Command> {
    loop {
        let start_char = port::read_byte(port)?;
        if start_char == b'?' {
            let b = port::read_byte(port)?;
            //if debug { println!("DEBUG: Received identifier {}", b); }

            let opt_identifier: Option<CommandIdentifier> = FromPrimitive::from_u8(b);
            if let Some(identifier) = opt_identifier {
                let command = match identifier {
                    CommandIdentifier::Identify => { read_identify(port) }
                    CommandIdentifier::SendFile => { read_send_file(port) }
                    CommandIdentifier::RequestChar => { read_request_char() }
                    CommandIdentifier::PrintChar => { read_print_char(port) }
                };
                if debug { println!("DEBUG: Command {:?}", command) };
                return command;
            } else {
                //Err(port::error("Unknown command identifier"))
            }
        }
    }
}

