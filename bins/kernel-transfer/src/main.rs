use std::io::prelude::Write;
use std::{env::args, fs::File, io::Read};

const HELP_MESSAGE: &str = "Usage: kernel-transfer <tty> <kernel image>";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if args().len() != 3 {
        return Err(HELP_MESSAGE.into());
    }
    let tty_path = args().nth(1).unwrap();
    let kernel_image_path = args().nth(2).unwrap();

    // read kernel image
    let mut kernel_image_file = File::open(kernel_image_path)?;
    let mut buf: Vec<u8> = Vec::new();
    let size = kernel_image_file.read_to_end(&mut buf)?;
    println!("Kernel size: {} bytes", size);

    // write to tty
    let mut tty = File::create(tty_path)?;
    println!("Sending protocal header...");
    tty.write_all(&size.to_le_bytes())?;
    println!("Sending kernel image...");
    tty.write_all(buf.as_slice())?;
    println!("Send complete.");
    Ok(())
}
