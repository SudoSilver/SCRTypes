use std::fs::File;
use std::io::{ BufRead, BufReader};
use crate::errors::FileErrors;

pub fn read_lines(path: &str) -> Result<Vec<String>, FileErrors> {
    let mut lines: Vec<String> = Vec::new();
    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(FileErrors::FileNotExists),
    };

    let reader = BufReader::new(file);

    for line_results in reader.lines() {
        let line = match line_results {
            Ok(content) => content,
            Err(_) => return Err(FileErrors::UnableToReadFromFile),
        };
        lines.push(line);
    }
    return Ok(lines);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_read_lines_success() {
        let path = "test_io_temp.txt";
        fs::write(path, "Rust\nIs\nAwesome").unwrap();

        let result = read_lines(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["Rust", "Is", "Awesome"]);

        fs::remove_file(path).unwrap(); // Keep the sandbox filesystem clean!
    }

    #[test]
    fn test_read_lines_missing() {
        let result = read_lines("ghost_file_xyz.txt");
        assert_eq!(result, Err(FileErrors::FileNotExists));
    }
}
