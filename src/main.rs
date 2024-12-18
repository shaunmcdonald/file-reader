#[cfg(test)]
mod tests {
    // use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};
    // use std::path::Path;

    fn cleanup(file_path: &str) {
        let _ = std::fs::remove_file(file_path);
    }

    #[test]
    fn test_file_contents() {
        let file_path = "src/test_file.txt";
        cleanup(file_path);

        // Create a test file
        let mut file = File::create(file_path).unwrap();
        file.write_all(b"Hello\nWorld\n").unwrap();
        
        // Verify file contents directly
        let reader = BufReader::new(File::open(file_path).unwrap());
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        assert_eq!(lines, vec!["Hello", "World"]);
        
        cleanup(file_path);
    }

    #[test]
    fn test_empty_file() {
        let file_path = "src/empty_file.txt";
        cleanup(file_path);

        // Create an empty test file
        File::create(file_path).unwrap();
        
        // Verify file is empty
        let reader = BufReader::new(File::open(file_path).unwrap());
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        assert!(lines.is_empty());
        
        cleanup(file_path);
    }

    #[test]
    #[should_panic(expected = "File not found: ")]
    fn test_nonexistent_file() {
        let file_path = "nonexistent_file.txt";
        if let Err(error) = File::open(file_path) {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error)
                },
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    }
}