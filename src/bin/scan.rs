extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;

use std::fs::File;

use hfsplus::*;

fn main() -> std::io::Result<()> {
    let filename = std::env::args().nth(1).unwrap();
    let path = std::env::args().nth(2);
    let volume = HFSVolume::load_file(filename.as_ref()).expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    {
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
                CatalogBody::Folder(ref x) => Some(&x.permissions),
                CatalogBody::File(ref x) => Some(&x.permissions),
                _ => None,
            };
            let f = c.key.node_name.to_string().nfc().collect::<String>();
            println!("File: {}", f);
            if let Some(x) = perms {
                println!("Mode: {:?}", x.fileMode);
                println!("User: {:?}", x.ownerID);
                println!("Group: {:?}", x.groupID);
            }
            println!("");
        }
    }

    Ok(())
}
