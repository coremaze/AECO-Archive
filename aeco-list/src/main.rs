use clap::Parser;

/// List the contents of an ECO archive
#[derive(Parser, Debug)]
struct Args {
    /// Path to archive .dat to list
    archive_dat_path: String,

    /// Path to archive .hed to list
    archive_hed_path: String,
}

fn main() {
    let args = Args::parse();

    let dat = &args.archive_dat_path;
    let hed = &args.archive_hed_path;

    let archive = match aeco_archive::Archive::open_pair(&dat, &hed) {
        Ok(x) => x,
        Err(why) => {
            eprintln!("Unable to open archive: {why:?}");
            return;
        }
    };

    println!("{:>#10} NAME", "SIZE");
    println!("---------- ----------");

    for file in archive.file_names() {
        let data = match archive.get_file(file) {
            Ok(x) => x,
            Err(why) => {
                eprintln!("Failed to list {file}: {why:?}");
                continue;
            }
        };

        println!("{:>#10} {file}", data.len());
    }

    println!("---------- ----------");

    if let Ok((used_space, total_space)) = archive.utilized_space() {
        let unused_space = total_space - used_space;
        let waste_percent = (unused_space as f32 / total_space as f32) * 100.;

        println!("{total_space} .dat size");
        println!("{used_space} bytes used");
        println!("{unused_space} bytes unused");
        println!("{waste_percent:.2}% wasted");
    }
}
