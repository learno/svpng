use std::fs::File;
use std::io::Result;

extern crate byteorder;

mod svpng;
use svpng::svpng;

fn test_rgb() -> Result<()> {
    let mut rgb = [0u8; 256 * 256 * 3];
    let mut index = 0;
    for y in 0..256usize {
        for x in 0..256usize {
            rgb[index] = x as u8;       /* R */
            rgb[index + 1] = y as u8;   /* G */
            rgb[index + 2] = 128;       /* B */
            index += 3;
        }
    }
    let file = File::create("rgb.png")?;
    svpng(file, 256, 256, &rgb, false);
    Ok(())
}

fn test_rgba() -> Result<()>  {
    let mut rgba = [0u8; 256 * 256 * 4];
    let mut index = 0usize;
    for y in 0..256usize {
        for x in 0..256usize {
            rgba[index] = x as u8;                  /* R */
            rgba[index + 1] = y as u8;              /* G */
            rgba[index + 2] = 128;                  /* B */
            rgba[index + 3] = ((x + y) / 2) as u8;  /* A */
            index += 4;
        }
    }
    let file = File::create("rgba.png")?;
    svpng(file, 256, 256, &rgba, true);
    Ok(())
}


fn main() {
    test_rgb().unwrap();
    test_rgba().unwrap();
}
