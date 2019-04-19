// use ffi::x509;
use openssl::x509::X509;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::os::raw::c_long;
use std::path::Path;

extern "C" {
    pub fn X509_subject_name_hash_old(x: *mut X509) -> c_long;
}

pub fn install_cert(paths: Vec<&str>) {
    let test_paths = paths.clone();
    for path in test_paths {
        let path_obj = Path::new(path);
        let display = path_obj.display();
        match File::open(&path_obj) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("couldn't open {}: {}", display, err.description());
                std::process::exit(1);
            }
        };
    }

    for path in paths {
        let content = get_cert(path).unwrap();
        let mut cert = X509::from_der(&content).unwrap();
        let hash = get_old_hash(&mut cert);
        println!("{:?}", hash);
        // let cert = X509::from_pem(&content).unwrap();
    }
}

fn get_cert(path: &str) -> Result<Vec<u8>, io::Error> {
    let mut cert_file = File::open(path)?;

    let mut buffer = Vec::new();
    cert_file.read_to_end(&mut buffer)?;
    return Ok(buffer);
}

fn get_old_hash(cert: *mut X509) -> c_long {
    unsafe { X509_subject_name_hash_old(cert) }
}
