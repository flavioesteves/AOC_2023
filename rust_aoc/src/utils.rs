use std::fs;
use std::path::Path;

/// Read a external file and convert into a string
pub fn read_file(filename: String) -> String {
    let path_root = "../inputs";
    let path_to_file = Path::new(&path_root).join(filename.as_str());
    let contents = fs::read_to_string(Path::new(&path_to_file))
        .expect("File not found or not able to read the file");
    contents
}

/// Convert test lines into a Vector
pub fn split_text_lines(file_data: String) -> Vec<String> {
    let mut lines = Vec::new();

    for line in file_data.lines() {
        lines.push(line.to_string())
    }
    lines
}
