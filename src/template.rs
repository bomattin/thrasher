use crc::crc32;

use rand::os::OsRng;
use rand::Rng;
use png;
use png::HasParameters;

use std::path::Path;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::fmt;

use data_encoding::HEXLOWER_PERMISSIVE;


struct HexSlice<'a>(&'a [u8]);

impl<'a> HexSlice<'a> {
    fn new<T>(data: &'a T) -> HexSlice<'a>
        where T: ?Sized + AsRef<[u8]> + 'a
    {
        HexSlice(data.as_ref())
    }
}

// You can even choose to implement multiple traits, like Lower and UpperHex
impl<'a> fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for byte in self.0 {
            // Decide if you want to pad out the value here
            write!(f, "{:02x} ", byte)?;
        }
        Ok(())
    }
}

// We're going to start by using the PNG file format as our template, to determine the features
// we'll need.

pub fn run_template() {

    /* What I want:
    let template_png = "89 50 4e 47 0d 0a 1a 0a
                        00 00 00 0D 49 48 44 52 00 00 00 80 00 00 00 80 08 02 00 00 00 {crc_ihdr:x:4}
                        {len_data:x:4} 49 44 41 54 {data:x:len_data} {crc_data:x:4}
                        00 00 00 00 49 45 4E 44 AE 42 60 82";
    */

    let template_png = "89 50 4e 47 0d 0a 1a 0a 00 00 00 0D 49 48 44 52 00 00 00 80 00 00 00 80 08 02 00 00 00 {crc_ihdr:x:4} {data:x} 00 00 00 00 49 45 4E 44 AE 42 60 82";

    let ihdr = [0x49, 0x48, 0x44, 0x52, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80, 0x08, 0x02, 0x00, 0x00, 0x00];
    let crc_ihdr = crc32::checksum_ieee(&ihdr);
    let mut data_buf : Vec<u8> = vec![0x00; 49152];
    let mut idat_buf : Vec<u8> = Vec::new();

    let mut rng = OsRng::new().unwrap();
    rng.fill_bytes(&mut data_buf);

    let ref mut w = BufWriter::new(idat_buf);

    let mut encoder = png::Encoder::new(w, 128, 128); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data_buf).unwrap(); // Save


    // MOTHERFUCKER
    // I GODDAMN KNEW IT
    // I CAN'T GET THIS MOTHERFUCKER OUT OF THERE. THAT'S PROBABLY JUST GREEEEEEAT
    // EXCEPT I NEED IT. I DON'T CARE ABOUT THE GUARANTEES, MOTHERFUCKER.
    // GIVE ME. THE. CLONE.
    // GIVE. ME. THE. GODDAMN. CLONE.
    let idat_buf_clone = &idat_buf.clone();

    let inner_data_hex = HexSlice::new(&idat_buf_clone);

    print!("{}", inner_data_hex);

    let mut res = format!("89 50 4e 47 0d 0a 1a 0a 00 00 00 0D 49 48 44 52 00 00 00 80 00 00 00 80 08 02 00 00 00 {crc_ihdr:x} {data} 00 00 00 00 49 45 4E 44 AE 42 60 82", crc_ihdr=crc_ihdr, data=inner_data_hex);
    res = res.replace(" ", "");
    //print!("{}",res);

    let out_data = HEXLOWER_PERMISSIVE.decode(res.as_bytes()).unwrap();


    let mut file = File::create("foo.png").unwrap();
    file.write_all(&out_data);


    // Clean the template (remove whitespace)
    // Slice input array into linked list. [hex_data] -> [template_directive] -> [hex_data]
    // Load into memory
    // Begin loop
    // Evaluate template_directives,
    // push out until done
    // End loop



    // Can use the CPython Crate
}
