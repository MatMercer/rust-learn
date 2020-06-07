use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

pub fn load_file(location: &str) -> File {
    let f = File::open(location);

    match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create(location) {
                Ok(created) => created,
                Err(e) => {
                    panic!(
                        "Tried to create file but couldn't :( {:?}",
                        e
                    )
                }
            }
        },
        Err(e) => {
            panic!(
                "Couldn't open the file :( {:?}",
                e
            )
        }
    }
}

pub fn long_read_str_from_file(location: &str) -> Result<String, io::Error> {
    let f = File::open(location);
    
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn small_read_str_from_file(location: &str) -> Result<String, io::Error> {
    let mut f = File::open(location)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

pub fn read_str_from_file(location: &str) -> Result<String, io::Error> {
    let mut s = String::new();

    File::open(location)?.read_to_string(&mut s)?;

    Ok(s)
}
