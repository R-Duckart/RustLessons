use std::io;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn path_file_prompt() -> std::io::Result<String> {
    println!("Please enter a path:");
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read input");

    Ok(path.trim().to_string())
}

fn line_prompt() -> std::io::Result<String> {
    println!("Please enter a line:");
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read input");

    Ok(line.trim().to_string())
}

fn read_file(path: &str) -> std::io::Result<File> {
    let file = File::open(path)?;
    Ok(file)
}

fn write_file(path: &str, line: &str) -> std::io::Result<()> {
    let mut file = File::options().append(true).create(true).open(path)?;
    writeln!(file, "{}", line)?;

    Ok(())
}

fn main() {

    let path: String = match path_file_prompt() {
        Ok(path_from_prompt) => path_from_prompt,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => panic!("File not found !"),
            std::io::ErrorKind::PermissionDenied => panic!("Permission denied !"),
            _ => panic!("Erreur : {}", error),
        }
    };

    let line: String = match line_prompt(){
        Ok(path) => path,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => panic!("File not found !"),
            std::io::ErrorKind::PermissionDenied => panic!("Permission denied !"),
            _ => panic!("Erreur : {}", error),
        }
    };

    if let Err(e) = write_file(&path, &line) {
        match e.kind() {
            std::io::ErrorKind::NotFound => panic!("Fichier introuvable !"),
            std::io::ErrorKind::PermissionDenied => panic!("Permission refusée !"),
            _ => panic!("Erreur fatale : {}", e),
        }
    }

    match read_file(&path) {
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
            std::io::ErrorKind::NotFound => eprintln!("File not found !"),
            std::io::ErrorKind::PermissionDenied => eprintln!("Permission denied !"),
            _ => eprintln!("Erreur fatale : {}", error),
        },
    }
}
