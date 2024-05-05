use std::fs::File;
use std::io;
use std::io::Write;

fn save_as_ppm(file_path: &str, pixels: &[u32], width: usize, height: usize) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    write!(file, "P6\n{} {} 255\n", width, height)?;
    for y in 0..height {
        for x in 0..width {
            let pixel = pixels[y * width + x];
            let color = [
                ((pixel >> 8 * 2) & 0xFF) as u8,
                ((pixel >> 8 * 1) & 0xFF) as u8,
                ((pixel >> 8 * 0) & 0xFF) as u8,
            ];
            file.write(&color)?;
        }
    }
    Ok(())
}
fn main() {
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;
    const OUTPUT_PATH: &str = "output.ppm";
    let mut pixels = [0u32; WIDTH * HEIGHT];
    pixels.fill(0x00FF00);
    // checker_pattern(&mut pixels, WIDTH, HEIGHT, 32, 0xFF00F, 0x000000);
    save_as_ppm(OUTPUT_PATH, &pixels, WIDTH, HEIGHT);
}
