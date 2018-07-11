use std::fs::File;
use std::io::Write;

use byteorder::WriteBytesExt;
use byteorder::{ByteOrder, LittleEndian, BigEndian};

struct PngWriter {
    file: File,

    a: u32,
    b: u32,
    c: u32,
}


impl PngWriter {
    const T: [u32; 16] = [ 0, 0x1db71064, 0x3b6e20c8, 0x26d930ac, 0x76dc4190, 0x6b6b51f4, 0x4db26158, 0x5005713c, 0xedb88320, 0xf00f9344, 0xd6d6a3e8, 0xcb61b38c, 0x9b64c2b0, 0x86d3d2d4, 0xa00ae278, 0xbdbdf21c ]; //CRC32 Table

    fn new(file: File) -> PngWriter {
        PngWriter {
            file: file,

            a: 1,
            b: 0,
            c: !0,
        }
    }

    fn write_u8c(&mut self, u: u8) {
        self.file.write_u8(u).unwrap();
        self.c ^= u as u32;
        let crc = |c| (c >> 4) ^ PngWriter::T[(c & 0xf) as usize];
        self.c = crc(self.c);
        self.c = crc(self.c);
    }

    fn write_u8ac(&mut self, ua: &[u8]) {
        for u in ua {
            self.write_u8c(*u);
        }
    }

    fn write_u16lc(&mut self, u: u16) {
        let mut buf = [0; 2];
        LittleEndian::write_u16(&mut buf, u);
        self.write_u8ac(&buf);
    }


    fn write_u32c(&mut self, u: u32) {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, u);
        self.write_u8ac(&buf);
    }

    fn write_u8adler(&mut self, u: u8) {
        self.write_u8c(u); 
        self.a = (self.a + u as u32) % 0xfff1;
        self.b = (self.b + self.a) % 0xfff1;
    }

    fn write_begin(&mut self, s: &[u8], l: u32) {
        self.file.write_u32::<BigEndian>(l).unwrap();
        self.c = !0;
        self.write_u8ac(s) ;
    }

    fn write_end(&mut self, ) {
        self.file.write_u32::<BigEndian>(!self.c).unwrap();
    }

    fn write_header(&mut self, ) {
        self.file.write_all(b"\x89PNG\r\n\x1a\n").unwrap(); //Magic
    }

    fn write_ihdr(&mut self, w: u32, h: u32, alpha: bool) {
        self.write_begin(b"IHDR", 13);  //IHDR chunk
        self.write_u32c(w);
        self.write_u32c(h);             //Width & Height (8 bytes)

        self.write_u8c(8);              //Depth=8
        self.write_u8c(if alpha {6} else {2});   //Color=True color with/without alpha (2 bytes)
        self.write_u8ac(b"\0\0\0");     //Compression=Deflate, Filter=No, Interlace=No (3 bytes) 
        self.write_end();
    }

    fn write_idat(&mut self, w: u32, h: u32, alpha: bool, img: &[u8]) {
        let p = w * (if alpha {4} else {3}) + 1;
        let l = 2 + h * (5 + p) + 4;
        self.write_begin(b"IDAT", l); //IDAT chunk
        self.write_u8ac(b"\x78\x01");   //Deflate block begin (2 bytes)

        //ADLER-a, ADLER-b, CRC, pitch
        let mut index = 0;
        for y in 0..h { //Each horizontal line makes a block for simplicity
            self.write_u8c(if y == h - 1 {1} else {0}); //1 for the last block, 0 for others (1 byte)
            self.write_u16lc(p as u16);
            self.write_u16lc(!p as u16);        //Size of block in little endian and its 1's complement (4 bytes)
            self.write_u8adler(0);              //No filter prefix (1 byte)
            for _ in 0..(p - 1) {
                self.write_u8adler(img[index]); //Image pixel data
                index += 1;
            }

        }
        let adler = (self.b << 16) | self.a;
        self.write_u32c(adler);         //Deflate block end with adler (4 bytes)
        self.write_end();
    }

    fn write_iend(&mut self) {
        self.write_begin(b"IEND", 0);   //IEND chunk
        self.write_end();
    }
}


pub fn svpng(file: File, w: u32, h: u32, img: &[u8], alpha: bool) {
    let mut writer = PngWriter::new(file);
    writer.write_header();
    writer.write_ihdr(w, h, alpha);
    writer.write_idat(w, h, alpha, img);
    writer.write_iend();
}

