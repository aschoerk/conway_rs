use rand;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;
use std::path::Path;

pub type Seed = fn(i16, i16) -> bool;

pub fn named(seed: &str) -> Option<Seed> {
    match seed {
        "random" => Some(random),
        "gosper_glider" => Some(gosper_glider),
        "binary_adder" => Some(binary_adder),
        _ => Some(read_file(seed).unwrap()),
    }
}

pub fn read_file(name: &str) -> io::Result<Seed> {
    let path = Path::new(name);
    if path.exists() {        
        let mut file = try!(File::open(path));
        let mut reader = BufReader::new(file);
        for line in reader.lines() {
            let line = try!(line);
            println!("{}",line);
        }
    }
    
    Ok::<Seed,io::Error>(random)
}

pub fn random(_: i16, _: i16) -> bool {
    rand::random()
}

pub fn binary_adder(x: i16, y: i16) -> bool {
    match (x, y) {
        (111,111) => true,
        (112,111) => true,
        (113,112) => true,
        (114,113) => true,
        _ => false
    }
}

pub fn gosper_glider(x: i16, y: i16) -> bool {
    match (x, y) {
        (1, 6) => true,
        (1, 5) => true,
        (2, 6) => true,
        (2, 5) => true,

        (11, 7) => true,
        (11, 6) => true,
        (11, 5) => true,

        (12, 8) => true,
        (12, 4) => true,

        (13, 9) => true,
        (13, 3) => true,
        (14, 9) => true,
        (14, 3) => true,

        (15, 6) => true,

        (16, 8) => true,
        (16, 4) => true,

        (17, 7) => true,
        (17, 6) => true,
        (17, 5) => true,

        (18, 6) => true,

        (21, 5) => true,
        (21, 4) => true,
        (21, 3) => true,

        (22, 5) => true,
        (22, 4) => true,
        (22, 3) => true,

        (23, 6) => true,
        (23, 2) => true,

        (25, 7) => true,
        (25, 6) => true,
        (25, 2) => true,
        (25, 1) => true,

        (35, 4) => true,
        (35, 3) => true,
        (36, 4) => true,
        (36, 3) => true,

        _ => false
    }
}
