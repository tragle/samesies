use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::path::Path;
use std::io::Result;

type DirProcessor<K,V> = Fn(&DirEntry, &mut HashMap<K,V>) -> Result<()>;
type FileBytes = HashMap<Vec<u8>, Vec<PathBuf>>;
type FileSizes = HashMap<u64, Vec<PathBuf>>;

fn get_dir() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return args[1].clone(); 
    }
    String::from(".")
}

fn find_sizes(dir_entry: &DirEntry, hashes: &mut FileSizes) -> Result<()> {
    let filename = dir_entry.path();
    print!("\r{:?} files sized", &hashes.len());
    match dir_entry.metadata() {
        Ok(metadata) => {
            let length = metadata.len();
            if length > 0 {
                let path_vec: Vec<PathBuf> = Vec::new();
                let filenames = hashes.entry(length).or_insert(path_vec);
                filenames.push(filename);
            }
        }
        _ => ()
    }
    Ok(())
}

fn find_bytes(filename: &Path, hashes: &mut FileBytes) -> Result<()> {
    print!("\r{:?} files hashed", &hashes.len());
    match fs::read(&filename) {
        Ok(bytes) => {
            if bytes.len() > 0 {
                let path_vec: Vec<PathBuf> = Vec::new();
                let filenames = hashes.entry(bytes).or_insert(path_vec);
                filenames.push(filename.to_path_buf());
            }
        }
        _ => ()
    }
    Ok(())
}

fn visit_write<K,V>(dir: &Path, cb: &'static DirProcessor<K,V>, mut hash: &mut HashMap<K,V>) -> Result<()> {
    if dir.is_dir() {
        match fs::read_dir(dir) {
            Ok(entries) => for entry in entries {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    visit_write(&path, cb, &mut hash)?;
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
    let mut bytes_hashes = HashMap::new(); 
    let mut size_hashes = HashMap::new(); 
    let dir = get_dir();
    let path = Path::new(&dir);

    println!("Sizing files...");

    visit_write(&path, &find_sizes, &mut size_hashes)?;

    println!();
    println!("\nHashing files...");

    for (_s, f) in &size_hashes {
        if f.len() > 1 {
            for file in f {
                find_bytes(file, &mut bytes_hashes)?;
            }
        }
    }
 
    println!();
    println!("\nFound duplicates:");

    for (_b, f) in &bytes_hashes {
        if f.len() > 1 {
            println!("\n{} duplicate files:", f.len());
            let mut num = 0;
            for file in f {
                num += 1;
                println!("{}) {:?}", num, file);
            }
        }
    }
    Ok(())
}
