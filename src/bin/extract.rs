//#![feature(fs_time)]
extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;

use std::io::prelude::*;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::fs::File;
use std::path::Path;
use std::fs::{self, OpenOptions};
use std::os::unix::fs::symlink;
use std::os::unix::fs::PermissionsExt;
use std::fs::set_permissions;
//use std::fs::set_file_times;

use hfsplus::*;

fn extract<P: AsRef<Path>>(vol2: &Ref<HFSVolume<File>>, volume: Rc<RefCell<HFSVolume<File>>>, record: &CatalogRecord, local_parent: P) -> std::io::Result<()> {
    let filename = record.key.node_name.to_string().nfc().collect::<String>();
    let local_file_path = local_parent.as_ref().join(&filename);
    if record.key.node_name.to_string().chars().nth(0) == Some('\u{0000}') {
        return Ok(())
    }
    match record.body {
        CatalogBody::Folder(body) => {
            fs::create_dir(&local_file_path)?;
            let children = vol2.get_children_id(body.folderID)?;
            for child in children {
                println!("Extract...");
                extract(&vol2, Rc::clone(&volume), child.as_ref(), &local_file_path)?;
            }
        },
        CatalogBody::File(body) => {
            println!("Found a file: {}", filename);
            let mut data_fork = Fork::load(Rc::clone(&vol2.file), body.fileID, 0, Rc::clone(&volume), &body.dataFork)?;
            let data = data_fork.read_all().unwrap();
            match body.permissions.fileMode & S_IFMT {
                S_IFIFO => {
                    println!("Found a FIFO!");
                },
                S_IFCHR => {
                    println!("Found a Character device!");
                },
                S_IFDIR => {
                    panic!("Found a directory inside a File object");
                },
                S_IFBLK => {
                    println!("Found a Block device!");
                },
                S_IFREG => {
                    println!("Saving contents to {:?}", local_file_path);
                    let mut local_file = OpenOptions::new().write(true)
                                                           .create_new(true)
                                                           .open(&local_file_path)?;
                    println!("Saving...");
                    local_file.write(&data[..])?;
                    println!("Done.");
                },
                S_IFLNK => {
                    println!("Found a symlink!");
                    symlink(std::str::from_utf8(&data).unwrap(), &local_file_path)?;
                },
                S_IFSOCK => {
                    println!("Found a Socket!");
                },
                _ => {
                    println!("Unsupported file type");
                },
            };
            let metadata = local_file_path.metadata()?;
            let mut permissions = metadata.permissions();
            permissions.set_mode(body.permissions.fileMode as u32 & 0o7777);
            set_permissions(&local_file_path, permissions)?;
            println!("User: {:?}", body.permissions.ownerID);
            println!("Group: {:?}", body.permissions.groupID);
        },
        _ => {
            panic!("Invalid Return Value");
        },
    };

    Ok(())
}

fn main() -> std::io::Result<()> {
    let filename_arg = std::env::args().nth(1).unwrap();
    let path_arg     = std::env::args().nth(2).unwrap();
    let local_arg    = std::env::args().nth(3).unwrap();
    let local = Path::new(&local_arg);
    let volume = HFSVolume::load_file(&filename_arg).expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    let record = vol2.get_path_record(&path_arg)?;
    if !local.is_dir() {
        writeln!(std::io::stderr(), "Destination for extraction must be a directory");
        std::process::exit(1);
    }
    extract(&vol2, Rc::clone(&volume), &record, &local)?;
    //            println!("{} 1 {:5} {:5} {:7} Jan  1 1970 {}", mode, x.ownerID, x.groupID, size, f);
    //            //println!("Mode: {:?}", mode);
    //            //println!("User: {:?}", x.ownerID);
    //            //println!("Group: {:?}", x.groupID);

    Ok(())
}
