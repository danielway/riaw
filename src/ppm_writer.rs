use std::fs::File;
use std::io::{self, Write};

pub fn write_image(
    file: &str,
    height: usize,
    width: usize,
    pixels: Vec<(u8, u8, u8)>,
) -> io::Result<()> {
    let mut file = File::create(file)?;

    file.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())?;
    for (r, g, b) in pixels {
        file.write_all(format!("{} {} {}\n", r, g, b).as_bytes())?;
    }

    Ok(())
}