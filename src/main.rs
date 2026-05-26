use std::{
    env::args,
    fs,
    io::{BufRead, BufReader, Read, Write, stdout},
    ffi::CStr
};

use flate2::read::ZlibDecoder;

enum Kind {
    Blob,
    NotBlob,
}

fn main() {
    let args: Vec<String> = args().collect();
    if args[1] == "init" {
        fs::create_dir(".git").unwrap();
        fs::create_dir("./git/objects").unwrap();
        fs::create_dir(".git/refs").unwrap();
        fs::write("./git/HEAD", "refs/head/main\n").unwrap();
        println!("Initialized git directory");
    } else {
        println!("Unknow command: {}", args[1])
    }

    read_blob();
}

fn read_blob() {
    let args: Vec<String> = args().collect();
    if args.len() != 4 {
        return;
    }

    let command = &args[1];
    let flag = &args[2];
    let object_hash = &args[3];
    if flag != "-p" {
        return;
    }
    if command != "cat-file" {
        return;
    }

    let path = format!(".git/objects/{}/{}", &object_hash[..2], &object_hash[2..]);
    let file = fs::File::open(path).unwrap();

    let z = ZlibDecoder::new(file);
    let mut z = BufReader::new(z);
    let mut buf = Vec::new();

    z.read_until(0, &mut buf).unwrap();

    let header = CStr::from_bytes_with_nul(&buf).unwrap();
    let header = header.to_str().unwrap();
    
    let (kind, size) = header.split_once(" ").unwrap();
    
    let kind = match kind {
        "blob" => Kind::Blob,
        _ => Kind::NotBlob,
    };

    let size = size.parse::<usize>().unwrap();

    buf.clear();
    buf.resize(size, 0);

    z.read_exact(&mut buf[..]).unwrap();

    let stdout = stdout();
    let mut stdout = stdout.lock();

    match kind {
        Kind::Blob => stdout.write_all(&buf).unwrap(),
        _ => println!("not blob"),
    }
}
