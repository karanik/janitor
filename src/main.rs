use chrono::Local;
use clap::Command;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
//use fs_extra::dir::{self, MoveOptions};

fn process_folder(source_path: &PathBuf, janitor_path: &PathBuf) {
    //let mut destination_path = dirs::home_dir().unwrap();
    let mut destination_path = janitor_path.clone();
    destination_path.push("janitor");

    // Check if source exists
    if !source_path.exists() || !source_path.is_dir() {
        eprintln!("Source path does not exist or is not a directory.");
        std::process::exit(1);
    }

    // Generate destination directory name based on current date
    let date_string = Local::now().format("%Y-%m-%d").to_string();
    destination_path.push(date_string);

    println!("Target Dir {}", destination_path.to_str().unwrap());

    // Create destination directory if it does not exist
    if !destination_path.exists() {
        //fs::create_dir_all(&destination_path).expect("Failed to create destination directory.");
    }

    //let mut options = MoveOptions::new();
    // options.overwrite = true; // Enable if you want to overwrite existing files in the destination
    //options.copy_inside = true; // For moving the contents into the destination directory

    let paths = fs::read_dir(source_path).unwrap();
    for p in paths {
        let path = p.unwrap();
        let is_dir = path.path().is_dir();
        println!(
            "{}: {}",
            if is_dir { "Dir" } else { "File" },
            path.path().display()
        );

        if is_dir {
            //fs::create_dir_all(&dest_path).expect("Failed to create directory in destination.");
        } else {
            //fs::rename(path, &dest_path).expect("Failed to move file.");
        }
    }

    /*

        for entry in WalkDir::new(source_path) {
            let entry = entry.unwrap();
            let path = entry.path();
            let relative_path = path.strip_prefix(source_path).unwrap();
            let dest_path = destination_path.join(relative_path);

            if path.is_dir() {
                fs::create_dir_all(&dest_path).expect("Failed to create directory in destination.");
            } else {
                fs::rename(path, &dest_path).expect("Failed to move file.");
            }
        }

        // Optionally, remove the source directory after moving its contents
        // fs::remove_dir_all(source_path).expect("Failed to remove source directory.");
    */
}

fn main() {
    /*
        let matches = Command::new("File Mover")
            .version("1.0")
            .author("Your Name")
            .about("Moves files and directories from source to a date-named destination directory")
            .arg(clap::Arg::new("source")
                .help("The source directory to move files from")
                .required(true)
                .index(1))
            .get_matches();

        let source_path = Path::new(matches.value_of("source").unwrap());
    */

    let mut janitor_path = dirs::home_dir().unwrap();
    janitor_path.push("janitor");
    let source_path = dirs::desktop_dir().unwrap();
    println!("Desktop dir {}", source_path.display());

    process_folder(&source_path, &janitor_path);
}
