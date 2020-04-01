use std::io::Read;
use std::path::Path;

use serialport::SerialPort;

mod arguments;
mod commands;
mod config;
mod port;

use commands::Command;

#[derive(Debug)]
pub enum UartError {
    Serial(serialport::Error),
    Utf8(std::str::Utf8Error),
    Io(std::io::Error),
    Other(String),
}

impl std::fmt::Display for UartError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UartError::Serial(error) => error.fmt(f),
            UartError::Utf8(error) => error.fmt(f),
            UartError::Io(error) => error.fmt(f),
            UartError::Other(error) => write!(f, "{}", error)
        }
    }
}
impl From<std::str::Utf8Error> for UartError {
    fn from(error: std::str::Utf8Error) -> Self {
        UartError::Utf8(error)
    }
}

impl From<serialport::Error> for UartError {
    fn from(error: serialport::Error) -> Self {
        UartError::Serial(error)
    }
}

impl From<std::io::Error> for UartError {
    fn from(error: std::io::Error) -> Self {
        UartError::Io(error)
    }
}

pub fn error(description: &str) -> UartError {
    UartError::Other(description.to_string())
}

enum Status {
    Ok = 0,
    NotAvailable = 1
}

#[cfg(not(target_os = "windows"))]
enum Hc800KeyCodes {
    Home = 1,
    Left = 2,
    Delete = 4,
    End = 5,
    Right = 6,
    BackSpace = 8,
    Tab = 9,
    Return = 10,
    Down = 14,
    Up = 16,
    F1 = 18,
    F2 = 19,
    F3 = 20,
    F4 = 21,
    F5 = 22,
    F6 = 23,
    F7 = 24,
    F8 = 25,
    Escape = 27,
    F9 = 28,
    F10 = 29,
    F11 = 30,
    F12 = 31,
}

#[cfg(not(target_os = "windows"))]
fn to_hc800(wch_result: ncurses::WchResult, debug: bool) -> Option<u8> {
    match wch_result {
        ncurses::WchResult::Char(wch) => {
            match wch {
                9 => Some(Hc800KeyCodes::Tab as u8),
                10 => Some(Hc800KeyCodes::Return as u8),
                27 => Some(Hc800KeyCodes::Escape as u8),
                127 => Some(Hc800KeyCodes::BackSpace as u8),
                wch if wch >= 32 && wch <= 255 => Some(wch as u8),
                _ => {
                    if debug { println!("DEBUG: Can't map character {}", wch) }
                    None
                }
            }
        }
        ncurses::WchResult::KeyCode(code) => {
            match code {
                ncurses::constants::KEY_HOME => Some(Hc800KeyCodes::Home as u8),
                ncurses::constants::KEY_LEFT => Some(Hc800KeyCodes::Left as u8),
                ncurses::constants::KEY_DC => Some(Hc800KeyCodes::Delete as u8),
                ncurses::constants::KEY_END => Some(Hc800KeyCodes::End as u8),
                ncurses::constants::KEY_RIGHT => Some(Hc800KeyCodes::Right as u8),
                ncurses::constants::KEY_DOWN => Some(Hc800KeyCodes::Down as u8),
                ncurses::constants::KEY_UP => Some(Hc800KeyCodes::Up as u8),
                ncurses::constants::KEY_F1 => Some(Hc800KeyCodes::F1 as u8),
                ncurses::constants::KEY_F2 => Some(Hc800KeyCodes::F2 as u8),
                ncurses::constants::KEY_F3 => Some(Hc800KeyCodes::F3 as u8),
                ncurses::constants::KEY_F4 => Some(Hc800KeyCodes::F4 as u8),
                ncurses::constants::KEY_F5 => Some(Hc800KeyCodes::F5 as u8),
                ncurses::constants::KEY_F6 => Some(Hc800KeyCodes::F6 as u8),
                ncurses::constants::KEY_F7 => Some(Hc800KeyCodes::F7 as u8),
                ncurses::constants::KEY_F8 => Some(Hc800KeyCodes::F8 as u8),
                ncurses::constants::KEY_F9 => Some(Hc800KeyCodes::F9 as u8),
                ncurses::constants::KEY_F10 => Some(Hc800KeyCodes::F10 as u8),
                ncurses::constants::KEY_F11 => Some(Hc800KeyCodes::F11 as u8),
                ncurses::constants::KEY_F12 => Some(Hc800KeyCodes::F12 as u8),
                _ => {
                    if debug { println!("DEBUG: Can't handle keycode {}", code) }
                    None
                }
            }
        }
    }
}

#[cfg(target_os = "windows")]
fn handle_request_char(port: &mut dyn SerialPort, _debug: bool) -> Result<(), UartError> {
    port::write_byte(port, b'!')?;
    port::write_byte(port, Status::NotAvailable as u8)?;
    port.flush()?;
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn handle_request_char(port: &mut dyn SerialPort, debug: bool) -> Result<(), UartError> {
    match ncurses::get_wch().and_then(|result| to_hc800(result, debug)) {
        Some(w1252) => {
            port::write_byte(port, b'!')?;
            port::write_byte(port, Status::Ok as u8)?;
            port::write_byte(port, w1252)?;
        }
        _ => {
            port::write_byte(port, b'!')?;
            port::write_byte(port, Status::NotAvailable as u8)?;
        }
    }
    port.flush()?;
    Ok(())
}

fn handle_identity(nonce: u16, port: &mut dyn SerialPort, debug: bool) -> Result<(), UartError> {
    let return_value = !nonce;
    if debug { println!("DEBUG: Identity returning {}", return_value); }
    port::write_byte(port, b'!')?;
    port::write_byte(port, Status::Ok as u8)?;
    port::write_u16(port, return_value)?;
    port.flush()?;
    Ok(())
}

fn handle_send_file(options: commands::SendFileOptions, root: &Path, port: &mut dyn SerialPort, debug: bool) -> Result<(), UartError> {
    let full_path = root.join(options.path);
    if debug { println!("DEBUG: Send file {}", full_path.display()) }
    if let Ok(mut file) = std::fs::File::open(full_path) {
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;

        if debug { println!("DEBUG: Sending file with {} bytes", data.len()) }

        port::write_byte(port, b'!')?;
        port::write_byte(port, Status::Ok as u8)?;
        port::write_vec(port, &data)?;
    } else {
        if debug { println!("DEBUG: File not found") }

        port::write_byte(port, b'!')?;
        port::write_byte(port, Status::NotAvailable as u8)?;
    }
    port.flush()?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn init_terminal() {
    ncurses::initscr();
    ncurses::timeout(0);
    ncurses::cbreak();
    ncurses::noecho();
    ncurses::keypad(ncurses::constants::stdscr(), true);
    ncurses::set_escdelay(100);
}

#[cfg(target_os = "windows")]
fn init_terminal() {
}

fn serve(port: &mut dyn SerialPort, root: &Path, debug: bool) -> Result<(), UartError> {
    init_terminal();
    loop {
        let command = commands::read_command(port, debug)?;
        match command {
            Command::Identify { nonce } => { handle_identity(nonce, port, debug)?; }
            Command::SendFile { options } => { handle_send_file(options, root, port, debug)?; }
            Command::RequestChar => { handle_request_char(port, debug)?; }
        }
    }
}

fn inner_main() -> Result<(), UartError> {
    let cfg = config::Config::read();
    let arguments = arguments::Arguments::new(cfg);
    if let Some(port_name) = arguments.port {
        let mut port = port::open(&port_name)?;
        serve(&mut *port, Path::new(&arguments.path), arguments.debug)?;
        return Ok(());
    } else {
        return Err(error("Port not specified. Either use --port argument or define in configuration."));
    }
}

fn main() {
    let exit_code =
        match inner_main() {
            Ok(_) => { 0 }
            Err(msg) => {
                println!("{}", msg);
                1
            }
        };

    std::process::exit(exit_code);
}