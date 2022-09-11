use clap::Parser;

/// Unpack an ECO archive
#[derive(Parser, Debug)]
struct Args {
    /// Path to archive .dat to unpack
    archive_dat_path: String,

    /// Path to archive .hed to unpack
    archive_hed_path: String,

    /// Location to unpack archive contents
    output_dir: String,
}

fn main() {
    let args = Args::parse();

    let dat = &args.archive_dat_path;
    let hed = &args.archive_hed_path;
    let out = &args.output_dir;

    if let Err(why) = std::fs::create_dir(out) {
        eprintln!("Unable to create output dir: {}", why);
        return;
    }

    let archive = match aeco_archive::Archive::open_pair(&dat, &hed) {
        Ok(x) => x,
        Err(why) => {
            eprintln!("Unable to open archive: {why:?}");
            return;
        }
    };

    for file in archive.file_names() {
        let mut path = std::path::PathBuf::new();
        path.push(&out);
        path.push(&file);

        let data = match archive.get_file(file) {
            Ok(x) => x,
            Err(why) => {
                eprintln!("Failed to extract {file}: {why:?}");
                continue;
            }
        };

        match std::fs::write(&path, data) {
            Ok(_) => println!("Extracted and wrote {file}"),
            Err(why) => eprintln!("Failed to write {file}: {}", why),
        }
    }
}
