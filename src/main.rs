use kmod;
use uname_rs::Uname;
use std::path::{PathBuf, Path};
use std::{io, fs};
use std::vec::Vec;

/***
 * Recursive .ko file search in kernel dir
 **/
fn read_dir<P>(dir: P, kmods: &mut Vec<String>) -> io::Result<()> where P: AsRef<Path> {

    for entry in fs::read_dir(dir.as_ref())? {
        let path: &Path = &entry?.path();

        if path.is_dir() {

            read_dir(path, kmods)?;
        } else if path.to_str().unwrap().ends_with("ko") {

            kmods.push(path.to_str().unwrap().to_string());
        }
    }
    Ok(())
}

fn find_insertable_kmods() -> Vec<String> {
    // Find kernel dir
    let uts = Uname::new().expect("Uname not working");
    println!("Kernel release: {}", uts.release);
    let kdir = PathBuf::from(format!("/lib/modules/{}", uts.release));

    // Read kernel dir recursive
    let mut kmods: Vec<String> = Vec::new();
    if kdir.is_dir() {
        match read_dir(&kdir, &mut kmods) {
            Err(err) => panic!("Recursive kmod search error: {}", err),
            _ => {}
        }
    } else {
        panic!("Kernel directory does not exist: {}", kdir.to_str().unwrap());
    }
    kmods
}

fn print_kmods_orderly(title: &str, kmods: Vec<&kmod::Module>) {

    println!("{} ({}):", title, kmods.len());
    kmods.iter().for_each(|m| println!("\t{}: {}", m.path().unwrap(), m.name()));
}

/***
 ** Filter them into three groups, loaded, not loaded, and loaded but not on disk. And
 ** yes I know, no set optimizations are needed for a 100 millisecond execution.
 **/
fn which_kmods_are_loaded(loaded_kmods: &Vec<kmod::Module>, disk_kmods: &Vec<kmod::Module>) {

    let mut loaded: Vec<&kmod::Module> = Vec::new();
    let mut not_loaded: Vec<&kmod::Module> = Vec::new();
    let mut loaded_not_on_disk: Vec<&kmod::Module> = Vec::new();

    // For loaded and not loaded
    for disk_kmod in disk_kmods {
        let mut l: bool = false;
        for loaded_kmod in loaded_kmods {
            if loaded_kmod.name().eq(disk_kmod.name()) {
                l = true;
                break;
            }
        }
        if l {
            loaded.push(&disk_kmod);
        } else {
            not_loaded.push(&disk_kmod);
        }
    }

    // For loaded but not on disk
    for loaded_kmod in loaded_kmods {
        let mut d: bool = false;
        for disk_kmod in disk_kmods {
            if loaded_kmod.name().eq(disk_kmod.name()) {
                d = true;
                break;
            }
        }
        if !d {
            loaded_not_on_disk.push(loaded_kmod);
        }
    }

    print_kmods_orderly("Loaded modules", loaded);
    print_kmods_orderly("Not loaded modules", not_loaded);
    print_kmods_orderly("Loaded but not on disk", loaded_not_on_disk);
}

fn main() {

    let ctx = kmod::Context::new().expect("Kmod context failed");

    // Find all loaded kmod file names
    let loaded_kmods: Vec<kmod::Module> = ctx.modules_loaded().unwrap().enumerate().map(|(_, m)| m).collect();
    println!("Loaded kmods: {}", loaded_kmods.len());

    // Load each available .ko file as a Module
    let kmod_files = find_insertable_kmods();
    let available_kmods: Vec<kmod::Module> = kmod_files.iter().map(|path| ctx.module_new_from_path(path).unwrap()).collect();
    println!("Available kmods: {}", available_kmods.len());

    which_kmods_are_loaded(&loaded_kmods, &available_kmods);
}

#[test]
fn test() {
    main();
}
