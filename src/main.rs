use std::io;
use std::fs;
use std::time::Instant;
fn main() {
    std::process::exit(real_main());
}
fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect(); // Get command line arguments
    if args.len() != 2 { // Check if the number of arguments is correct
        eprintln!("Usage: {} <filename>", args[0]);
        return 1;
    }
    let fname = std::path::Path::new(&args[1]); // Get the filename
    let file = fs::File::open(&fname).unwrap(); // Open the file
    let mut archive = zip::ZipArchive::new(file).unwrap();
    let start = Instant::now();
    for i in 0..archive.len() { // Iterate over files in the archive
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        {
            let comment = file.comment(); // Read the file comment
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }
        if (*file.name()).ends_with('/') { // Directory
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!("File {} extracted to \"{}\" ({} bytes)", i, outpath.display(), file.size());
            if let Some(p) = outpath.parent() { // Create parent directories if they don't exist
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap(); // Create the file
            io::copy(&mut file, &mut outfile).unwrap(); // Write the file contents
        }
        #[cfg(windows)] // Set file permissions on Windows
        {
            use std::os::windows::fs::MetadataExt;
            use std::fs::OpenOptions;
            use std::os::windows::fs::OpenOptionsExt;
            let mut perms = file.unix_mode().unwrap();
            let mut options = OpenOptions::new();
            options.write(true);
            options.read(true);
            options.custom_flags(libc::O_CREAT as u32);
            options.custom_flags(perms as u32);
            let _ = options.open(&outpath);
        }
    }
    println!("File extracted in {:?}", start.elapsed());
    0
}