use clap::{App, Arg};
use std::collections::HashSet;
use std::fs;
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use uname_rs::Uname;

fn read_disk_kmods_recursive(
    ctx: &kmod::Context,
    loaded_kmods: &HashSet<String>,
    disk_kmods: &mut HashSet<String>,
    path: &Path,
) {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            read_disk_kmods_recursive(ctx, loaded_kmods, disk_kmods, &entry.path());
        }
    } else {
        let path = path.to_str().unwrap();
        if path.contains(".ko") {
            let modpath = path_to_modpath(Some(path));
            disk_kmods.insert(modpath);
        }
    }
}

fn read_disk_kmods(
    ctx: &kmod::Context,
    loaded_kmods: &HashSet<String>,
    disk_kmods: &mut HashSet<String>,
    release: &str,
) {
    let kmod_dir = PathBuf::from(format!("/lib/modules/{}/kernel", release));
    read_disk_kmods_recursive(ctx, loaded_kmods, disk_kmods, &kmod_dir);
}

fn path_to_modpath(path_str: Option<&str>) -> String {
    let mut path_str = path_str.unwrap();
    for _ in 0..4 {
        path_str = path_str.split_at(path_str.find('/').unwrap() + 1).1;
    }
    path_str.to_string()
}

fn read_builtins(loaded_kmods: &mut HashSet<String>, release: &str) {
    std::fs::read_to_string(format!("/lib/modules/{}/modules.builtin", release))
        .unwrap()
        .lines()
        .for_each(|l| {
            loaded_kmods.insert(l.to_string());
        });
}

fn main() {
    let release = Uname::new().unwrap().release;
    let release = release.as_str();
    let args = App::new("KBuild kmods util")
        .arg(
            Arg::with_name("release")
                .short("r")
                .long("release")
                .value_name("$(uname -r)")
                .takes_value(true)
                .required(true)
                .default_value(release),
        )
        .get_matches();
    let disk_release = args.value_of("release").unwrap();

    let ctx = kmod::Context::new().expect("Kmod context failed");

    // Loaded mods
    let mut loaded_kmods: HashSet<String> = ctx
        .modules_loaded()
        .unwrap()
        .enumerate()
        .map(|(_, m)| path_to_modpath(m.path()))
        .collect();
    // Also add builtins to loaded, but from the running kernel
    read_builtins(&mut loaded_kmods, release);

    // Disk mods
    let mut disk_kmods: HashSet<String> = HashSet::new();
    // For now, the disk kmods are the same as the running kernel
    read_disk_kmods(&ctx, &loaded_kmods, &mut disk_kmods, disk_release);
    read_builtins(&mut disk_kmods, disk_release);

    // Find differences
    let missing_disk_kmods: HashSet<String> = disk_kmods
        .iter()
        .filter(|i| !loaded_kmods.contains(*i))
        .cloned()
        .collect();

    let missing_loaded_kmods: HashSet<String> = loaded_kmods
        .iter()
        .filter(|i| !disk_kmods.contains(*i))
        .cloned()
        .collect();

    // Print differences
    for loaded in &loaded_kmods {
        println!("Loaded: {}", loaded);
    }

    for disk in &disk_kmods {
        println!("On disk: {}", disk);
    }

    for missing in &missing_loaded_kmods {
        println!("Missing in loaded kmods: {}", missing);
    }

    for missing in &missing_disk_kmods {
        println!("Missing on disk: {}", missing);
    }
}

#[test]
fn test() {
    main();
}
