use crate::ppm_writer::write_image;

mod ppm_writer;

fn main() {
    println!("Hello, world!");

    let data = vec![
        (255, 0, 0),
        (0, 255, 0),
        (0, 0, 255),
        (255, 255, 0),
        (255, 255, 255),
        (0, 0, 0),
    ];

    write_image("test.ppm", 2, 3, data).expect("Failed to write image");
}
