use std::fs::read_to_string;
use std::io::Write;
use std::path::Path;

pub fn write_string_to_file(s: &str, filename: &Path) {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(filename);
    let mut f = match f {
        Ok(f) => f,
        Err(_) => {
            let dir = filename.parent().unwrap();
            std::fs::create_dir_all(dir).unwrap();
            std::fs::File::create(filename)
                .unwrap_or_else(|_| panic!("unable to open file: '{}'", filename.to_str().unwrap()))
        }
    };

    f.write_all(s.as_bytes()).unwrap();
}

#[allow(unused)]
pub fn create_or_append(s: &str, filename: &Path) {
    let f = std::fs::OpenOptions::new().append(true).open(filename);
    if let Ok(mut f) = f {
        f.write_all(s.as_bytes()).unwrap();
    } else {
        write_string_to_file(s, filename);
    }
}

pub fn overwrite_if_not_same_contents(s: &str, filename: &Path) {
    let f = std::fs::File::open(filename);
    if f.is_ok() {
        let f = read_to_string(filename).unwrap();
        if f != s {
            eprintln!("Updating file: {}", filename.display());
            write_string_to_file(s, filename);
        } else {
            eprintln!("Not modified: {}", filename.display());
        }
    } else {
        eprintln!("Creating file: {}", filename.display());
        write_string_to_file(s, filename);
    }
}
