use clap::Parser;

/// Pack an ECO archive
#[derive(Parser, Debug)]
struct Args {
    /// Path to dir containing files to archive
    input_dir: String,

    /// Path to archive .dat to create
    archive_dat_path: String,

    /// Path to archive .hed to create
    archive_hed_path: String,
}

fn main() {
    let args = Args::parse();

    let dat = &args.archive_dat_path;
    let hed = &args.archive_hed_path;
    let dir = &args.input_dir;

    // The archive is not expected to already exist
    if std::path::Path::new(&dat).exists() {
        eprintln!("{dat} already exists");
        return;
    }

    if std::path::Path::new(&hed).exists() {
        eprintln!("{hed} already exists");
        return;
    }

    // Create a new archive with the specified files
    let mut archive = match aeco_archive::Archive::open_pair(&dat, &hed) {
        Ok(x) => x,
        Err(why) => {
            eprintln!("Failed to open archive {dat} + {hed}: {why:?}");
            return;
        }
    };

    // Read files in the input dir
    let iter_dir = match std::fs::read_dir(&dir) {
        Ok(x) => x,
        Err(why) => {
            eprintln!("Unable to read directory {dir}: {}", why);
            return;
        }
    };

    for entry in iter_dir {
        let entry = match entry {
            Ok(x) => x,
            Err(why) => {
                eprintln!("Failed while iterating directory {dir}: {}", why);
                continue;
            }
        };

        let file_path = entry.path();
        let file_name = entry.file_name();

        // The file name is expected to be ASCII
        let ascii_file_name = if let Some(filename) = file_name.to_str() {
            if filename.is_ascii() {
                filename
            } else {
                eprintln!("File {filename} contains non-ASCII characters");
                continue;
            }
        } else {
            eprintln!("Could not read a file name");
            continue;
        };

        // Read data from file to be added to archive
        let data = match std::fs::read(file_path) {
            Ok(x) => x,
            Err(why) => {
                eprintln!("Failed to read {ascii_file_name}: {}", why);
                continue;
            }
        };

        // Add the file's data to the archive
        if let Err(why) = archive.add_file(ascii_file_name, &data) {
            eprintln!("Failed while adding file {ascii_file_name}: {why:?}");
            return;
        }
    }

    // Finish saving archive
    if let Err(why) = archive.finalize() {
        eprintln!("Failed to finalize archive: {why:?}");
    }
}
