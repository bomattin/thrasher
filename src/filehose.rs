use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::Write;
use rand::os::OsRng;
use rand::Rng;


pub fn spam(path: &PathBuf, n: i32, ext: &str) {
    let mut rng = OsRng::new().unwrap();
    let mut buf : Vec<u8> = vec![0x00; 4096];


    for i in 0..n {
        rng.fill_bytes(&mut buf);
        let mut file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(format!("{}.txt", i))
                        .unwrap();
        file.write_all(&buf).unwrap();
    }
}
