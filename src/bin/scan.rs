extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;

use std::fs::File;

use hfsplus::*;

fn main() -> std::io::Result<()> {
    let filename = std::env::args().nth(1).unwrap();
    let path = std::env::args().nth(2);
    let volume = HFSVolume::load_file(filename.as_ref()).expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    if path.is_none() {
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
    println!("{} -> {}", btree.header.header.firstLeafNode, btree.header.header.lastLeafNode);
    let mut node_num = btree.header.header.firstLeafNode;
    while node_num != 0 {
        println!("Dump node {}:", node_num);
        let node = btree.get_node(node_num as usize)?;
        match node {
            Node::LeafNode(LeafNode { descriptor: d, .. }) => {
                println!("Next: {}", d.fLink);
                node_num = d.fLink;
            },
            _ => {
            },
        }
    }
    }
    if let Some(p) = path {
        println!("Open path: {}", p);
        let children = vol2.get_path(&p)?;
        for c in &children {
            let perms = match c.body {
                CatalogBody::Folder(ref x) => Some((&x.permissions, 4096)),
                CatalogBody::File(ref x) => Some((&x.permissions, x.dataFork.logicalSize)),
                _ => None,
            };
            let f = c.key.node_name.to_string().nfc().collect::<String>();
            //println!("File: {}", f);
            if let Some((x, size)) = perms {
                // TODO Check for special mode bits
                let ftype = match x.fileMode & S_IFMT {
                    S_IFIFO => "p",
                    S_IFCHR => "c",
                    S_IFDIR => "d",
                    S_IFBLK => "b",
                    S_IFREG => "-",
                    S_IFLNK => "l",
                    S_IFSOCK => "s",
                    _ => "?",
                };
                let mode = format!("{}{}{}{}{}{}{}{}{}{}",
                        ftype,
                        if x.fileMode & S_IRUSR != 0 { "r" } else { "-" },
                        if x.fileMode & S_IWUSR != 0 { "w" } else { "-" },
                        if x.fileMode & S_IXUSR != 0 { "x" } else { "-" },
                        if x.fileMode & S_IRGRP != 0 { "r" } else { "-" },
                        if x.fileMode & S_IWGRP != 0 { "w" } else { "-" },
                        if x.fileMode & S_IXGRP != 0 { "x" } else { "-" },
                        if x.fileMode & S_IROTH != 0 { "r" } else { "-" },
                        if x.fileMode & S_IWOTH != 0 { "w" } else { "-" },
                        if x.fileMode & S_IXOTH != 0 { "x" } else { "-" });
                println!("{} 1 {:5} {:5} {:7} Jan  1 1970 {}", mode, x.ownerID, x.groupID, size, f);
                //println!("Mode: {:?}", mode);
                //println!("User: {:?}", x.ownerID);
                //println!("Group: {:?}", x.groupID);
            }
            //println!("");
        }
    }

    Ok(())
}
