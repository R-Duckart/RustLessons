use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn write_in_file(path: &str, line: &str) -> std::io::Result<()> {
    let mut file = File::options().append(true).create(true).open(path)?;
    writeln!(file, "{}", line)?;

    Ok(())
}

fn main() {
    let path = "file.txt";
    let line = "nouvelle ligne";

    if let Err(e) = write_in_file(path, line) {
        match e.kind() {
            std::io::ErrorKind::NotFound => eprintln!("Fichier introuvable !"),
            std::io::ErrorKind::PermissionDenied => eprintln!("Permission refusée !"),
            _ => eprintln!("Erreur fatale : {}", e),
        }
    }

    let file = File::open(path);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(error) => eprintln!("Error reading line: {}", error),
                }
            }
        }
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!("File not found: {}", error)
            }
            std::io::ErrorKind::PermissionDenied => {
                eprintln!("Open File Permission denied: {}", error)
            }
            _ => {
                eprintln!("Error opening file: {}", error)
            }
        },
    }
}
