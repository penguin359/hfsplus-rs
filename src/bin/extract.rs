extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;

use std::io::prelude::*;
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::fs::File;
use std::path::Path;
use std::fs::{self, OpenOptions};

use hfsplus::*;

fn extract<P: AsRef<Path>>(vol2: &Ref<HFSVolume<File>>, volume: Rc<RefCell<HFSVolume<File>>>, record: &CatalogRecord, local_parent: P) -> std::io::Result<()> {
    let filename = record.key.node_name.to_string().nfc().collect::<String>();
    let local_file_path = local_parent.as_ref().join(&filename);
    match record.body {
        CatalogBody::Folder(body) => {
            fs::create_dir(&local_file_path)?;
            let children = vol2.get_children_id(body.folderID)?;
            for child in children {
                extract(&vol2, Rc::clone(&volume), child.as_ref(), &local_file_path)?;
                let perms = match child.body {
                    CatalogBody::Folder(ref x) => Some((&x.permissions, 4096)),
                    CatalogBody::File(ref x) => Some((&x.permissions, x.dataFork.logicalSize)),
                    _ => None,
                };
            }
        },
        CatalogBody::File(body) => {
            println!("Found a file: {}", filename);
            let mut data_fork = Fork::load(Rc::clone(&vol2.file), Rc::clone(&volume), &body.dataFork)?;
            let data = data_fork.read_all().unwrap();
            let mut local_file = OpenOptions::new().write(true)
                                                   .create_new(true)
                                                   .open(local_file_path)?;
            local_file.write(&data[..]);
            //let contents = std::str::from_utf8(data.as_ref()).unwrap();
            //println!("Contents: {}", contents);
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
    extract(&vol2, Rc::clone(&volume), &record, &local);
    //if let Some(p) = path {
    //    println!("Open path: {}", p);
    //    let record = vol2.get_path_record(&p)?;
    //    match record {
    //        CatalogBody::Folder(body) => {
    //            let children = vol2.get_children_id(body.folderID)?;
    //    for c in &children {
    //        let perms = match c.body {
    //            CatalogBody::Folder(ref x) => Some((&x.permissions, 4096)),
    //            CatalogBody::File(ref x) => Some((&x.permissions, x.dataFork.logicalSize)),
    //            _ => None,
    //        };
    //        let f = c.key.node_name.to_string().nfc().collect::<String>();
    //        //println!("File: {}", f);
    //        if let Some((x, size)) = perms {
    //            // TODO Check for special mode bits
    //            let ftype = match x.fileMode & S_IFMT {
    //                S_IFIFO => "p",
    //                S_IFCHR => "c",
    //                S_IFDIR => "d",
    //                S_IFBLK => "b",
    //                S_IFREG => "-",
    //                S_IFLNK => "l",
    //                S_IFSOCK => "s",
    //                _ => "?",
    //            };
    //            let mode = format!("{}{}{}{}{}{}{}{}{}{}",
    //                    ftype,
    //                    if x.fileMode & S_IRUSR != 0 { "r" } else { "-" },
    //                    if x.fileMode & S_IWUSR != 0 { "w" } else { "-" },
    //                    if x.fileMode & S_IXUSR != 0 { "x" } else { "-" },
    //                    if x.fileMode & S_IRGRP != 0 { "r" } else { "-" },
    //                    if x.fileMode & S_IWGRP != 0 { "w" } else { "-" },
    //                    if x.fileMode & S_IXGRP != 0 { "x" } else { "-" },
    //                    if x.fileMode & S_IROTH != 0 { "r" } else { "-" },
    //                    if x.fileMode & S_IWOTH != 0 { "w" } else { "-" },
    //                    if x.fileMode & S_IXOTH != 0 { "x" } else { "-" });
    //            println!("{} 1 {:5} {:5} {:7} Jan  1 1970 {}", mode, x.ownerID, x.groupID, size, f);
    //            //println!("Mode: {:?}", mode);
    //            //println!("User: {:?}", x.ownerID);
    //            //println!("Group: {:?}", x.groupID);
    //        }
    //        //println!("");
    //    }
    //        },
    //        CatalogBody::File(body) => {
    //            println!("Found a file!");
    //            let mut data_fork = Fork::load(Rc::clone(&vol2.file), Rc::clone(&volume), &body.dataFork)?;
    //            let data = data_fork.read_all().unwrap();
    //            let contents = std::str::from_utf8(data.as_ref()).unwrap();
    //            println!("Contents: {}", contents);
    //        },
    //        _ => {
    //            panic!("Invalid Return Value");
    //        },
    //    };
    //};

    Ok(())
}
