use std::fs::{self, File};
use std::path::PathBuf;

fn main() {
    // Read the contents of the output.txt file
    let contents =
        fs::read_to_string("output_problem.txt").expect("Failed to read the contents of the file.");

    // Split the contents into lines
    let lines: Vec<&str> = contents.lines().collect();

    // Create folders and generate Rust files
    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            continue; // Skip empty lines
        }

        let problem_number = line.replace("Problem ", "");
        let folder_name    = format!("Problem {}", problem_number);
        let file_name      = format!("problem_{}.rs", problem_number);
        let folder_path   = PathBuf::from(&folder_name);
        let file_path     = folder_path.join(&file_name);

        // Create the folder
        if let Err(err) = fs::create_dir(&folder_path) {
            eprintln!("Failed to create folder '{}': {}", folder_name, err);
            continue;
        }

        // Create the Rust file inside the folder
        if let Err(err) = File::create(&file_path) {
            eprintln!("Failed to create file '{}': {}", file_path.display(), err);
            continue;
        }

        // Close the file after writing (an empty file in this case)
        drop(File::open(&file_path));
    }
}
