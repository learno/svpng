use std::ffi::CString;

macro_rules! c_str {
    ($s: expr) => {
        CString::new($s).unwrap().as_ptr()
    };
}

extern crate libc;
use libc::{FILE, c_int, c_uint};

extern {
    fn svpng(fp: *mut FILE, w: c_uint, h: c_uint, img: *const u8, alpha: c_int);
}

fn test_rgb()  {
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
    unsafe {
        let fp = libc::fopen(c_str!("rgb.png"), c_str!("wb"));
        svpng(fp, 256, 256, rgb.as_ptr(), 0);
        libc::fclose(fp);
    }
}

fn test_rgba()  {
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
    unsafe {
        let fp = libc::fopen(c_str!("rgba.png"), c_str!("wb"));
        svpng(fp, 256, 256, rgba.as_ptr(), 1);
        libc::fclose(fp);
    }
}


fn main() {
    test_rgb();
    test_rgba();
}
