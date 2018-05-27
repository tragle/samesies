use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::path::Path;
use std::io::Result;

type FileHash = HashMap<Vec<u8>, Vec<PathBuf>>;

fn get_dir() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return args[1].clone(); 
    }
    String::from(".")
}

fn process(dir_entry: &DirEntry, hashes: &mut FileHash) -> Result<()> {
    let filename = dir_entry.path();
    print!("\r{:?} files found", &hashes.len());
    match fs::read(&filename) {
        Ok(bytes) => {
            if bytes.len() > 0 {
                let path_vec: Vec<PathBuf> = Vec::new();
                let filenames = hashes.entry(bytes).or_insert(path_vec);
                filenames.push(filename);
            }
        }
        _ => ()
    }
    Ok(())
}

fn visit(dir: &Path, cb: &Fn(&DirEntry, &mut FileHash) -> Result<()>, mut hash: &mut FileHash) -> Result<()> {
    if dir.is_dir() {
        match fs::read_dir(dir) {
            Ok(entries) => for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit(&path, &cb, &mut hash)?;
                } else {
                    cb(&entry, &mut hash)?;
                }
            }
            _ => ()
        }
    }
    Ok(())

}

fn main() -> Result<()> {
    let mut hashes = HashMap::new(); 
    let dir = get_dir();
    let path = Path::new(&dir);

    visit(&path, &process, &mut hashes)?;

    println!("\nLooking for duplicates.");

    for (_b, f) in &hashes {
        if f.len() > 1 {
            println!("{:?}", f);
        }

    }
    Ok(())
}
