use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if there are any arguments
    if args.len() < 3 {
        println!("Please provide at least two folder locations.");
        return;
    }

    // Create a new folder to store the unique files
    let output_folder = Path::new("unique_ebooks");
    if !output_folder.exists() {
        fs::create_dir(output_folder).unwrap();
    }

    // Create a set to store the unique file names
    let mut unique_files = std::collections::HashSet::new();

    // Iterate over the provided folders
    for folder in &args[1..] {
        // Read the directory
        let dir = fs::read_dir(folder).unwrap();

        // Iterate over the files in the directory
        for file in dir {
            let file = file.unwrap();
            let file_name = file.file_name().to_str().unwrap().to_string();

            // Check if the file is a pdf
            if file_name.ends_with(".pdf") {
                // Check if the file is already in the set
                if !unique_files.contains(&file_name) {
                    // Add the file to the set
                    unique_files.insert(file_name.clone());

                    // Copy the file to the output folder
                    let src = file.path();
                    let dst = output_folder.join(file_name);
                    fs::copy(src, dst).unwrap();
                }
            }
        }
    }

    println!("Unique ebooks have been copied to the 'unique_ebooks' folder.");
}
