use chrono::{DateTime, Local};
use core::time;
use std::fs;
//use clap::Command;
use std::path::{Path, PathBuf};
//use walkdir::WalkDir;
use anyhow::{anyhow, Result};
//use fs_extra::dir::{self, MoveOptions};
use glob::Pattern;

const JANITOR_DIR_NAME: &str = "Archive";
const IGNORE_GLOBS: [&str; 1] = ["**/.*"];

fn process_folder(source_path: &PathBuf) -> Result<()> {
    //let mut destination_path = janitor_path.clone();
    //destination_path.push("janitor");

    // Check if source exists
    if !source_path.exists() || !source_path.is_dir() {
        eprintln!("Source path does not exist or is not a directory.");
        std::process::exit(1);
    }

    // Janitor dir is in the same folder as the source data
    let mut destination_path = source_path
        .parent()
        .ok_or_else(|| anyhow!("No parent?"))?
        .to_path_buf();
    let dirname = source_path
        .file_name()
        .ok_or_else(|| anyhow!("No top-level source path?"))?;
    destination_path.push(dirname);
    destination_path.push(JANITOR_DIR_NAME);

    // Append current date
    let current_date = Local::now();
    let date_string = current_date.format("%Y-%m-%d").to_string();
    destination_path.push(date_string);

    println!("Target Dir {}", destination_path.to_str().unwrap());

    // Create destination directory if it does not exist
    if !destination_path.exists() {
        //fs::create_dir_all(&destination_path).expect("Failed to create destination directory.");
    }

    let ioptions = glob::MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: true,
    };

    //let mut options = MoveOptions::new();
    // options.overwrite = true; // Enable if you want to overwrite existing files in the destination
    //options.copy_inside = true; // For moving the contents into the destination directory

    let paths = fs::read_dir(source_path)?;
    for p in paths {
        let dent: std::fs::DirEntry = p?;
        let mdata = dent.metadata()?;
        let created: DateTime<Local> = mdata.created()?.into();
        //let accessed: DateTime<Local> = mdata.accessed()?.into();
        let pathbuf = dent.path();
        let fname = pathbuf.file_name().unwrap();

        let mut ignore = false;
        let mut ignore_reason = "";

        // Ignore by glob
        for iglob in IGNORE_GLOBS {
            let p = glob::Pattern::new(iglob).unwrap();
            if p.matches_path_with(pathbuf.as_path(), ioptions) {
                ignore = true;
                ignore_reason = "glob";
            }
        }

        // Ignore symlinks
        if !ignore && pathbuf.is_symlink() {
            ignore = true;
            ignore_reason = "symlink";
        }

        // Ignore special files
        if !ignore && fname == JANITOR_DIR_NAME {
            ignore = true;
            ignore_reason = "special";
        }

        // Ingore new files
        let time_diff = current_date - created;
        if !ignore && time_diff < chrono::Duration::days(1) {
            ignore = true;
            ignore_reason = "new file";
        }

        println!(
            "{}: C:{} {} {}",
            if pathbuf.is_dir() { "Dir " } else { "File" },
            created.format("%Y-%m-%d").to_string(),
            pathbuf.display(),
            if ignore {
                format!("[Ignore: {}]", ignore_reason)
            } else {
                String::new()
            }
        );

        if ignore {
            continue;
        }

        if pathbuf.is_dir() {
            //fs::create_dir_all(&dest_path).expect("Failed to create directory in destination.");
        } else {
            //fs::rename(path, &dest_path).expect("Failed to move file.");
        }
    }
    Ok(())
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

    let source_path = dirs::desktop_dir().unwrap();
    println!("Desktop dir {}", source_path.display());

    process_folder(&source_path).unwrap();
}
