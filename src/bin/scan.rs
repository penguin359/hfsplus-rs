use std::io::{Read, Seek};
use std::fs::File;

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};

use hfsplus::*;

fn main() -> std::io::Result<()> {
    let volume = HFSVolume::load_file("hfsp-small.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
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

    Ok(())
}
