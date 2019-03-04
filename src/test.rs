extern crate unicode_normalization;

use super::*;

use byteorder::WriteBytesExt;

fn blank_volume_header() -> HFSPlusVolumeHeader {
    HFSPlusVolumeHeader {
        signature: HFSP_SIGNATURE,
        version: 4,
        attributes: VolumeAttributes::from_bits(2147483904u32).unwrap(),
        lastMountedVersion: 825241136u32,
        journalInfoBlock: 0,
        createDate: 3633621750u32,
        modifyDate: 3633650550u32,
        backupDate: 0,
        checkedDate: 3633650550u32,
        fileCount: 0,
        folderCount: 0,
        blockSize: 4096,
        totalBlocks: 256,
        freeBlocks: 221,
        nextAllocation: 114,
        rsrcClumpSize: 65536,
        dataClumpSize: 65536,
        nextCatalogID: 16,
        writeCount: 0,
        encodingsBitmap: 1,
        finderInfo: [
             0, 0, 0, 0, 0, 0, 2358727412, 1649547363
        ],
        allocationFile: HFSPlusForkData {
            logicalSize: 4096,
            clumpSize: 4096,
            totalBlocks: 1,
            extents: [
                HFSPlusExtentDescriptor { startBlock: 1, blockCount: 1, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            ],
        },
        extentsFile: HFSPlusForkData {
            logicalSize: 32768,
            clumpSize: 32768,
            totalBlocks: 8,
            extents: [
                HFSPlusExtentDescriptor { startBlock: 2, blockCount: 8, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            ],
        },
        catalogFile: HFSPlusForkData {
            logicalSize: 32768,
            clumpSize: 32768,
            totalBlocks: 8,
            extents: [
                HFSPlusExtentDescriptor { startBlock: 26, blockCount: 8, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            ],
        },
        attributesFile: HFSPlusForkData {
            logicalSize: 65536,
            clumpSize: 65536,
            totalBlocks: 16,
            extents: [
                HFSPlusExtentDescriptor { startBlock: 10, blockCount: 16, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            ],
        },
        startupFile: HFSPlusForkData {
            logicalSize: 0,
            clumpSize: 0,
            totalBlocks: 0,
            extents: [
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            ],
        },
    }
}

/// Generate a bare-bones Volume Header suitable for basic testing
/// Some fields will need to be modified for basic testing, but this
/// should provide a decent starting point with minimal changes needed
/// to set up a test bed.
fn empty_v4_volume_header() -> HFSPlusVolumeHeader {
    HFSPlusVolumeHeader {
        signature: HFSP_SIGNATURE,
        version: 4,
        attributes: VolumeAttributes::kHFSVolumeUnmountedBit,
        lastMountedVersion: 0,
        journalInfoBlock: 0,
        createDate: 0,
        modifyDate: 0,
        backupDate: 0,
        checkedDate: 0,
        fileCount: 0,
        folderCount: 0,
        blockSize: 4096,
        totalBlocks: 0,
        freeBlocks: 0,
        nextAllocation: 0,
        rsrcClumpSize: 65536,
        dataClumpSize: 65536,
        nextCatalogID: 16,
        writeCount: 0,
        encodingsBitmap: 1,
        finderInfo: [
             0, 0, 0, 0, 0, 0, 0, 0
        ],
        allocationFile: HFSPlusForkData {
            logicalSize: 0,
            clumpSize: 4096,
            totalBlocks: 0,
            extents: [
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            ],
        },
        extentsFile: HFSPlusForkData {
            logicalSize: 0,
            clumpSize: 32768,
            totalBlocks: 0,
            extents: [
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            ],
        },
        catalogFile: HFSPlusForkData {
            logicalSize: 0,
            clumpSize: 32768,
            totalBlocks: 0,
            extents: [
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            ],
        },
        attributesFile: HFSPlusForkData {
            logicalSize: 0,
            clumpSize: 65536,
            totalBlocks: 0,
            extents: [
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            ],
        },
        startupFile: HFSPlusForkData {
            logicalSize: 0,
            clumpSize: 0,
            totalBlocks: 0,
            extents: [
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
                HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            ],
        },
    }
}

#[test]
fn load_blank_volume_header() {
    let expected = blank_volume_header();

    let mut file = File::open("hfsp-blank.img").unwrap();
    file.seek(std::io::SeekFrom::Start(1024)).expect("Failed to seek in file");
    let actual = HFSPlusVolumeHeader::import(&mut file).expect("Failed to read Volume Header");
    assert_eq!(expected, actual);
}

#[test]
fn load_blank_volume() {
    let volume = HFSVolume::load_file("hfsp-blank.img").expect("Failed to read Volume Header");
    assert_eq!(volume.borrow().header.version, 4);
}

#[test]
fn load_blank_file() {
    let volume = HFSVolume::load_file("/dev/zero");
    assert!(volume.is_err(), "Failed to throw error reading blank volume");
    //assert_eq!(volume.unwrap_err().to_string(), "");
}

#[test]
fn load_bad_version_file() {
    let volume = HFSVolume::load_file("/dev/zero");
    assert!(volume.is_err(), "Failed to throw error reading blank volume");
    //assert_eq!(volume.unwrap_err().to_string(), "");
}

#[test]
fn test_bad_fork_data() {
    let fork_data = HFSPlusForkData {
        logicalSize: 32768,
        clumpSize: 32768,
        totalBlocks: 8,
        extents: [
            HFSPlusExtentDescriptor { startBlock: 2, blockCount: 6, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        ],
    };
    let file = Rc::new(RefCell::new(File::open("/dev/null").unwrap()));
    let header = blank_volume_header();
    let volume = Rc::new(RefCell::new(HFSVolume {
        file: Rc::clone(&file),
        header,
        catalog_fork: Weak::new(),
        extents_fork: Weak::new(),
        forks: HashMap::new(),
        catalog_btree: None,
        extents_btree: None,
    }));
    let fork = Fork::load(file, volume, &fork_data).expect("Failed to load Fork");
    assert!(fork.check().is_err(), "Errors in fork data not detected in check");
}

#[test]
fn test_good_fork_data() {
    let fork_data = HFSPlusForkData {
        logicalSize: 32768,
        clumpSize: 32768,
        totalBlocks: 8,
        extents: [
            HFSPlusExtentDescriptor { startBlock: 2, blockCount: 8, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        ],
    };
    let file = Rc::new(RefCell::new(File::open("/dev/null").unwrap()));
    let header = blank_volume_header();
    let volume = Rc::new(RefCell::new(HFSVolume {
        file: Rc::clone(&file),
        header,
        catalog_fork: Weak::new(),
        extents_fork: Weak::new(),
        forks: HashMap::new(),
        catalog_btree: None,
        extents_btree: None,
    }));
    let fork = Fork::load(file, volume, &fork_data).expect("Failed to load Fork");
    assert!(fork.check().is_ok(), "Errors found in fork data");
}

#[test]
fn load_blank_volume_catalog_fork() {
    let volume = HFSVolume::load_file("hfsp-blank.img").expect("Failed to read Volume Header");
    assert!(volume.borrow().catalog_fork.upgrade().is_some(), "Invalid catalog fork pointer");
    assert!(volume.borrow().extents_fork.upgrade().is_some(), "Invalid extents fork pointer");
    let vol = volume.borrow();
    let vol2 = vol.catalog_fork.upgrade().unwrap();
    let fork = vol2.borrow();
    assert_eq!(fork.len(), 32768);
    assert_eq!(volume.borrow().extents_fork.upgrade().unwrap().borrow().len(), 32768);
    //let mut buffer = vec![0; 512];
    let mut buffer = vec![0; 512];
    fork.read(0, &mut buffer).expect("Failed to read from fork");
    //let node = BTNodeDescriptor::import(&mut Cursor::new(&mut buffer)).unwrap();
    let node = BTNodeDescriptor::import(&mut &buffer[..]).unwrap();
    assert_eq!(node.kind, kBTHeaderNode);
    assert_eq!(node.bLink, 0);
    assert_eq!(node.numRecords, 3);
    assert_eq!(node.reserved, 0);
    let node_size = (&buffer[32..34]).read_u16::<BigEndian>().expect("Error decoding node size");
    println!("{}", node_size);
    assert_eq!(node_size, 4096);
}

#[test]
fn load_blank_volume_catalog_btree() {
    let volume = HFSVolume::load_file("hfsp-blank.img").expect("Failed to read Volume Header");
    assert!(volume.borrow().catalog_btree.is_some(), "Invalid Catalog B-Tree pointer");
    assert!(volume.borrow().extents_btree.is_some(), "Invalid Extents B-Tree pointer");
    let vol2 = volume.borrow();
    match vol2.catalog_btree {
        Some(ref b) => {
            let btree = b.borrow();
            assert_eq!(btree.node_size, 4096);
            assert_eq!(btree.header.header.nodeSize, 4096);
        },
        None => { assert!(false, "Failed to open B-Tree"); },
    };
    match vol2.extents_btree {
        Some(ref b) => {
            let btree = b.borrow();
            assert_eq!(btree.node_size, 4096);
            assert_eq!(btree.header.header.nodeSize, 4096);
        },
        None => { assert!(false, "Failed to open B-Tree"); },
    };
    //let vol3 = &vol2.catalog_btree;
    //let fork = vol3.unwrap().borrow();
    //assert_eq!(fork.node_size, 32768);
    //assert_eq!(volume.borrow().extents_fork.upgrade().unwrap().borrow().len(), 32768);
    ////let mut buffer = vec![0; 512];
    //let mut buffer = vec![0; 512];
    //fork.read(0, &mut buffer);
    ////let node = BTNodeDescriptor::import(&mut Cursor::new(&mut buffer)).unwrap();
    //let node = BTNodeDescriptor::import(&mut &buffer[..]).unwrap();
    //assert_eq!(node.kind, kBTHeaderNode);
    //assert_eq!(node.bLink, 0);
    //assert_eq!(node.numRecords, 3);
    //assert_eq!(node.reserved, 0);
    //let node_size = (&buffer[32..34]).read_u16::<BigEndian>().expect("Error decoding node size");
    //println!("{}", node_size);
    //assert_eq!(node_size, 4096);
}

// TODO Test completely full offset table
#[test]
fn load_blank_btree_node() {
    let mut node_data = vec![0; 512];
    let node = Node::<CatalogRecord>::load(&node_data);
    assert!(node.is_err(), "All-zero node will have bad offsets");
    (&mut node_data[510..512]).write_u16::<BigEndian>(14).unwrap();
    let node = Node::<CatalogRecord>::load(&node_data);
    assert!(node.is_ok(), "Empty node with valid pointers not OK");
    (&mut node_data[10..12]).write_u16::<BigEndian>(3).unwrap();  // 3 Records
    let node = Node::<CatalogRecord>::load(&node_data);
    assert!(node.is_err(), "zero pointers will have bad offsets");
    //(&mut node_data[510..512]).write_u16::<BigEndian>(14);
}

use super::hfs_strings::fast_unicode_compare;
mod hfs_strings {
    use std::cmp::Ordering::{Less, Equal, Greater};

    use unicode_normalization::UnicodeNormalization;

    use super::fast_unicode_compare;


    #[test]
    fn compare_hfsp_strings() {
        let str1 = "THisIsTHEsame".nfd().collect::<String>().encode_utf16().collect::<Vec<u16>>();
        let str2 = "thisIStheSAME".nfd().collect::<String>().encode_utf16().collect::<Vec<u16>>();
        let acute_a = "THisIsTHEsáme".nfd().collect::<String>().encode_utf16().collect::<Vec<u16>>();
        let grave_e = "thisIStheSAMÈ".nfd().collect::<String>().encode_utf16().collect::<Vec<u16>>();
        let alpha = "Alpha".encode_utf16().collect::<Vec<u16>>();
        let zulu = "zulU".encode_utf16().collect::<Vec<u16>>();
        assert_eq!(fast_unicode_compare(&str1, &str1), Equal);
        assert_eq!(fast_unicode_compare(&str1, &str2), Equal);
        assert_eq!(fast_unicode_compare(&str1, &acute_a), Less);
        assert_eq!(fast_unicode_compare(&str2, &grave_e), Less);
        assert_eq!(fast_unicode_compare(&acute_a, &grave_e), Greater);
        assert_eq!(fast_unicode_compare(&str1, &alpha), Greater);
        assert_eq!(fast_unicode_compare(&str1, &zulu), Less);
    }
}

#[test]
fn create_hfs_string() {
    let str1    = HFSString::from("THisIsTHEsame");
    let str2    = HFSString::from("thisIStheSAME");
    let acute_a = HFSString::from("THisIsTHEsáme");
    let grave_e = HFSString::from("thisIStheSAMÈ");
    let alpha   = HFSString::from("Alpha");
    let zulu    = HFSString::from("zulU");
    assert!(str1 == str1);
    assert!(str1 == str2);
    assert!(str1 < acute_a);
    assert!(str2 < grave_e);
    assert!(acute_a > grave_e);
    assert!(str1 > alpha);
    assert!(str1 < zulu);
}

use std::cmp::Ordering::{Less, Equal, Greater};

#[test]
fn compare_catalog_keys() {
    let str1    = HFSString::from("THisIsTHEsame");
    let str2    = HFSString::from("thisIStheSAME");
    let acute_a = HFSString::from("THisIsTHEsáme");
    let grave_e = HFSString::from("thisIStheSAMÈ");
    let alpha   = HFSString::from("Alpha");
    let zulu    = HFSString::from("zulU");
    let str1_key = CatalogKey { _case_match: false, parent_id: 8, node_name: str1 };
    let str2_key = CatalogKey { _case_match: false, parent_id: 8, node_name: str2 };
    let acute_a_key = CatalogKey { _case_match: false, parent_id: 8, node_name: acute_a };
    let grave_e_key = CatalogKey { _case_match: false, parent_id: 8, node_name: grave_e };
    let alpha_key = CatalogKey { _case_match: false, parent_id: 10, node_name: alpha };
    let zulu_key = CatalogKey { _case_match: false, parent_id: 9, node_name: zulu };

    assert_eq!(str1_key.cmp(&str1_key), Equal);
    assert_eq!(str1_key.cmp(&str2_key), Equal);
    assert_eq!(str1_key.cmp(&acute_a_key), Less);
    assert_eq!(str2_key.cmp(&grave_e_key), Less);
    assert_eq!(acute_a_key.cmp(&grave_e_key), Greater);
    assert_eq!(str1_key.cmp(&alpha_key), Less);
    assert_eq!(str1_key.cmp(&zulu_key), Less);
    assert_eq!(alpha_key.cmp(&zulu_key), Greater);
    assert_eq!(alpha_key.cmp(&grave_e_key), Greater);
    assert_eq!(zulu_key.cmp(&grave_e_key), Greater);

    assert!(str1_key == str1_key);
    assert!(str1_key == str2_key);
    assert!(str1_key < acute_a_key);
    assert!(str2_key < grave_e_key);
    assert!(acute_a_key > grave_e_key);
    assert!(str1_key < alpha_key);
    assert!(str1_key < zulu_key);
    assert!(alpha_key > zulu_key);
    assert!(alpha_key > grave_e_key);
    assert!(zulu_key > grave_e_key);
}

#[test]
fn check_blank_hfs_btree() {
    let volume = HFSVolume::load_file("hfsp-blank.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
    let tree_header = &btree.header.header;
    assert_eq!(tree_header.rootNode, 1);
    assert_eq!(tree_header.firstLeafNode, 1);
    assert_eq!(tree_header.lastLeafNode, 1);
    let node = btree.get_node(tree_header.rootNode as usize);
    if node.is_err() {
        println!("{:?}", node.as_ref().err().unwrap());
    }
    assert!(node.is_ok());
    let node = node.unwrap();
    match node {
        Node::LeafNode(x) => {
            assert_eq!(x.descriptor.numRecords, 2);
            assert_eq!(x.descriptor.numRecords as usize, x.records.len());
            assert_eq!(x.records[0].key.parent_id, 1);
            assert_eq!(x.records[1].key.parent_id, 2);
        },
        _ => {
            assert!(false, "Wrong root node type");
        }
    };
}

#[test]
fn check_small_hfs_btree() {
    let volume = HFSVolume::load_file("hfsp-small.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
    let tree_header = &btree.header.header;
    assert_eq!(tree_header.treeDepth, 2);
    // Multiple leaf nodes are requires so all values
    // must be different
    assert_ne!(tree_header.rootNode, tree_header.firstLeafNode);
    assert_ne!(tree_header.rootNode, tree_header.lastLeafNode);
    assert_ne!(tree_header.firstLeafNode, tree_header.lastLeafNode);
    let node = btree.get_node(tree_header.rootNode as usize);
    if node.is_err() {
        println!("{:?}", node.as_ref().err().unwrap());
    }
    assert!(node.is_ok());
    let node = node.unwrap();
    match node {
        Node::IndexNode(x) => {
            assert_eq!(x.descriptor.numRecords, 3);
            assert_eq!(x.descriptor.numRecords as usize, x.records.len());
            assert_eq!(x.records[0].node_id, tree_header.firstLeafNode);
            assert_eq!(x.records[2].node_id, tree_header.lastLeafNode);
        },
        _ => {
            assert!(false, "Wrong root node type");
        }
    };
}

#[test]
fn load_root_thread_record() {
    let volume = HFSVolume::load_file("hfsp-blank.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
    let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let thread_record_res = btree.get_record(&root_thread_key);
    assert!(thread_record_res.is_ok(), "Failed to find root thread record");
    let result = thread_record_res.unwrap();
    let thread = match result.body {
        CatalogBody::FolderThread(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder thread record"); return;
        },
    };
    let root_key = CatalogKey { _case_match: false, parent_id: 1, node_name: HFSString::from("BLANK") };
    assert_eq!(*thread, root_key);
    //assert_eq!(thread.parent_id, 1);
    //assert_eq!(thread.node_name, root_key.node_name);
    //assert_eq!(tree_header.lastLeafNode, 1);
    //let node = btree.get_node(tree_header.rootNode as usize);
    //assert!(node.is_ok());
    //let node = node.unwrap();
    //match node {
    //    Node::LeafNode(x) => {
    //        assert_eq!(x.descriptor.numRecords, 2);
    //        assert_eq!(x.descriptor.numRecords as usize, x.records.len());
    //        assert_eq!(x.records[0].key.parent_id, 1);
    //        assert_eq!(x.records[1].key.parent_id, 2);
    //    },
    //    _ => {
    //        assert!(false, "Wrong root node type");
    //    }
    //};
}

#[test]
fn load_root_folder_record() {
    let volume = HFSVolume::load_file("hfsp-blank.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
    let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let thread_record_res = btree.get_record(&root_thread_key);
    if thread_record_res.is_err() {
        println!("{:?}", thread_record_res.as_ref().err().unwrap());
    }
    assert!(thread_record_res.is_ok(), "Failed to find root thread record");
    let result = thread_record_res.unwrap();
    let thread = match result.body {
        CatalogBody::FolderThread(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder thread record"); return;
        },
    };
    let root_record_res = btree.get_record(thread);
    if root_record_res.is_err() {
        println!("{:?}", root_record_res.as_ref().err().unwrap());
    }
    assert!(root_record_res.is_ok(), "Failed to find root record");
    let result = root_record_res.unwrap();
    let folder = match result.body {
        CatalogBody::Folder(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder record"); return;
        },
    };
    println!("{:?}", folder);
}

#[test]
fn load_blank_root_folder_listing() {
    let volume = HFSVolume::load_file("hfsp-blank.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow();
    let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let thread_record_res = btree.get_record(&root_thread_key);
    if thread_record_res.is_err() {
        println!("{:?}", thread_record_res.as_ref().err().unwrap());
    }
    assert!(thread_record_res.is_ok(), "Failed to find root thread record");
    let result = thread_record_res.unwrap();
    let thread = match result.body {
        CatalogBody::FolderThread(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder thread record"); return;
        },
    };
    let root_record_res = btree.get_record(thread);
    if root_record_res.is_err() {
        println!("{:?}", root_record_res.as_ref().err().unwrap());
    }
    assert!(root_record_res.is_ok(), "Failed to find root record");
    let result = root_record_res.unwrap();
    let folder = match result.body {
        CatalogBody::Folder(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder record"); return;
        },
    };
    let children_res = vol2.get_children(thread);
    if children_res.is_err() {
        println!("{:?}", children_res.as_ref().err().unwrap());
    }
    assert!(children_res.is_ok(), "Failed to search for children");
    let children = children_res.unwrap();
    assert_eq!(children.len(), 0);
}

#[test]
fn load_small_root_folder_listing() {
    let volume = HFSVolume::load_file("hfsp-small.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow();
    let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let thread_record_res = btree.get_record(&root_thread_key);
    if thread_record_res.is_err() {
        println!("{:?}", thread_record_res.as_ref().err().unwrap());
    }
    assert!(thread_record_res.is_ok(), "Failed to find root thread record");
    let result = thread_record_res.unwrap();
    let thread = match result.body {
        CatalogBody::FolderThread(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder thread record"); return;
        },
    };
    let root_record_res = btree.get_record(thread);
    if root_record_res.is_err() {
        println!("{:?}", root_record_res.as_ref().err().unwrap());
    }
    assert!(root_record_res.is_ok(), "Failed to find root record");
    let result = root_record_res.unwrap();
    let folder = match result.body {
        CatalogBody::Folder(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder record"); return;
        },
    };
    let children_res = vol2.get_children(thread);
    if children_res.is_err() {
        println!("{:?}", children_res.as_ref().err().unwrap());
    }
    assert!(children_res.is_ok(), "Failed to search for children");
    let children = children_res.unwrap();
    assert_ne!(children.len(), 0);
    let names = children.iter().filter_map(|item| match item.body {
        CatalogBody::Folder(_) => {
            Some(item.get_key().node_name.to_string().nfc().collect::<String>())
        },
        CatalogBody::File(_) => {
            Some(item.get_key().node_name.to_string().nfc().collect::<String>())
        },
        _ => None
    }).collect::<Vec<String>>();
    assert!(names.contains(&"hello.txt".to_string()));
    assert!(names.contains(&"files".to_string()));
    assert!(names.contains(&"\0\0\0\0HFS+ Private Data".to_string()));
}

#[test]
fn test_btree_get_record_range_blank() {
    let volume = HFSVolume::load_file("hfsp-blank.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow();
    let tree_header = &btree.header.header;
    assert_eq!(tree_header.treeDepth, 1);  // This test expects only leaf nodes
    let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let thread_record_res = btree.get_record(&root_thread_key);
    if thread_record_res.is_err() {
        println!("{:?}", thread_record_res.as_ref().err().unwrap());
    }
    assert!(thread_record_res.is_ok(), "Failed to find root thread record");
    let result = thread_record_res.unwrap();
    let thread = match result.body {
        CatalogBody::FolderThread(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder thread record"); return;
        },
    };
    let root_record_res = btree.get_record(thread);
    if root_record_res.is_err() {
        println!("{:?}", root_record_res.as_ref().err().unwrap());
    }
    assert!(root_record_res.is_ok(), "Failed to find root record");
    let result = root_record_res.unwrap();
    let folder = match result.body {
        CatalogBody::Folder(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder record"); return;
        },
    };
    let first = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let last = CatalogKey { _case_match: false, parent_id: 5, node_name: HFSString::from("") };
    let records_res = btree.get_record_range(&first, &last);
    assert!(records_res.is_ok(), "Failed to get record range");
    let records = records_res.unwrap();
    assert!(records.len() > 0);
    assert!(records[0].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[0].get_key() < &last, "First key after end of range (exclusive)");
    assert!(records[records.len()-1].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[records.len()-1].get_key() < &last, "First key after end of range (exclusive)");

    let first = CatalogKey { _case_match: false, parent_id: 1, node_name: HFSString::from("") };
    let last = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let records_res = btree.get_record_range(&first, &last);
    assert!(records_res.is_ok(), "Failed to get record range");
    let records = records_res.unwrap();
    assert!(records.len() > 0);
    assert!(records[0].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[0].get_key() < &last, "First key after end of range (exclusive)");
    assert!(records[records.len()-1].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[records.len()-1].get_key() < &last, "First key after end of range (exclusive)");

    let first = CatalogKey { _case_match: false, parent_id: 1, node_name: HFSString::from("") };
    let last = CatalogKey { _case_match: false, parent_id: 20, node_name: HFSString::from("") };
    let records_res = btree.get_record_range(&first, &last);
    assert!(records_res.is_ok(), "Failed to get record range");
    let records = records_res.unwrap();
    assert!(records.len() > 0);
    assert!(records[0].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[0].get_key() < &last, "First key after end of range (exclusive)");
    assert!(records[records.len()-1].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[records.len()-1].get_key() < &last, "First key after end of range (exclusive)");
}

#[test]
fn test_btree_get_record_range_small() {
    let volume = HFSVolume::load_file("hfsp-small.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow();
    let tree_header = &btree.header.header;
    assert_eq!(tree_header.treeDepth, 2);  // This test expects index nodes
    let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let thread_record_res = btree.get_record(&root_thread_key);
    if thread_record_res.is_err() {
        println!("{:?}", thread_record_res.as_ref().err().unwrap());
    }
    assert!(thread_record_res.is_ok(), "Failed to find root thread record");
    let result = thread_record_res.unwrap();
    let thread = match result.body {
        CatalogBody::FolderThread(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder thread record"); return;
        },
    };
    let root_record_res = btree.get_record(thread);
    if root_record_res.is_err() {
        println!("{:?}", root_record_res.as_ref().err().unwrap());
    }
    assert!(root_record_res.is_ok(), "Failed to find root record");
    let result = root_record_res.unwrap();
    let folder = match result.body {
        CatalogBody::Folder(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder record"); return;
        },
    };

    let first = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let last = CatalogKey { _case_match: false, parent_id: 5, node_name: HFSString::from("") };
    let records_res = btree.get_record_range(&first, &last);
    assert!(records_res.is_ok(), "Failed to get record range");
    let records = records_res.unwrap();
    assert!(records.len() > 0);
    assert!(records[0].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[0].get_key() < &last, "First key after end of range (exclusive)");
    assert!(records[records.len()-1].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[records.len()-1].get_key() < &last, "First key after end of range (exclusive)");

    let first = CatalogKey { _case_match: false, parent_id: 1, node_name: HFSString::from("") };
    let last = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let records_res = btree.get_record_range(&first, &last);
    assert!(records_res.is_ok(), "Failed to get record range");
    let records = records_res.unwrap();
    assert!(records.len() > 0);
    assert!(records[0].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[0].get_key() < &last, "First key after end of range (exclusive)");
    assert!(records[records.len()-1].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[records.len()-1].get_key() < &last, "First key after end of range (exclusive)");

    let first = CatalogKey { _case_match: false, parent_id: 1, node_name: HFSString::from("") };
    let last = CatalogKey { _case_match: false, parent_id: 20, node_name: HFSString::from("") };
    let records_res = btree.get_record_range(&first, &last);
    assert!(records_res.is_ok(), "Failed to get record range");
    let records = records_res.unwrap();
    assert!(records.len() > 0);
    assert!(records[0].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[0].get_key() < &last, "First key after end of range (exclusive)");
    assert!(records[records.len()-1].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[records.len()-1].get_key() < &last, "First key after end of range (exclusive)");
}

#[ignore]
#[test]
fn test_btree_get_record_range_many() {
    let volume = HFSVolume::load_file("hfsp-many2.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow();
    let tree_header = &btree.header.header;
    assert_eq!(tree_header.treeDepth, 3);  // This test expects index nodes
    let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let thread_record_res = btree.get_record(&root_thread_key);
    if thread_record_res.is_err() {
        println!("{:?}", thread_record_res.as_ref().err().unwrap());
    }
    assert!(thread_record_res.is_ok(), "Failed to find root thread record");
    let result = thread_record_res.unwrap();
    let thread = match result.body {
        CatalogBody::FolderThread(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder thread record"); return;
        },
    };
    let root_record_res = btree.get_record(thread);
    if root_record_res.is_err() {
        println!("{:?}", root_record_res.as_ref().err().unwrap());
    }
    assert!(root_record_res.is_ok(), "Failed to find root record");
    let result = root_record_res.unwrap();
    let folder = match result.body {
        CatalogBody::Folder(ref x) => {
            x
        },
        _ => {
            assert!(false, "Not a folder record"); return;
        },
    };

    let first = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let last = CatalogKey { _case_match: false, parent_id: 5, node_name: HFSString::from("") };
    let records_res = btree.get_record_range(&first, &last);
    assert!(records_res.is_ok(), "Failed to get record range");
    let records = records_res.unwrap();
    assert!(records.len() > 0);
    assert!(records[0].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[0].get_key() < &last, "First key after end of range (exclusive)");
    assert!(records[records.len()-1].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[records.len()-1].get_key() < &last, "First key after end of range (exclusive)");

    let first = CatalogKey { _case_match: false, parent_id: 1, node_name: HFSString::from("") };
    let last = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let records_res = btree.get_record_range(&first, &last);
    assert!(records_res.is_ok(), "Failed to get record range");
    let records = records_res.unwrap();
    assert!(records.len() > 0);
    assert!(records[0].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[0].get_key() < &last, "First key after end of range (exclusive)");
    assert!(records[records.len()-1].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[records.len()-1].get_key() < &last, "First key after end of range (exclusive)");

    let first = CatalogKey { _case_match: false, parent_id: 1, node_name: HFSString::from("") };
    let last = CatalogKey { _case_match: false, parent_id: 20, node_name: HFSString::from("") };
    let records_res = btree.get_record_range(&first, &last);
    assert!(records_res.is_ok(), "Failed to get record range");
    let records = records_res.unwrap();
    assert!(records.len() > 0);
    assert!(records[0].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[0].get_key() < &last, "First key after end of range (exclusive)");
    assert!(records[records.len()-1].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[records.len()-1].get_key() < &last, "First key after end of range (exclusive)");
}

#[test]
fn load_small_subfolder_path_listing() {
    let volume = HFSVolume::load_file("hfsp-small.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    let children_res = vol2.get_path("/files");
    if children_res.is_err() {
        println!("{:?}", children_res.as_ref().err().unwrap());
    }
    assert!(children_res.is_ok(), "Failed to search path for children");
    let children = children_res.unwrap();
    assert_eq!(children.len(), 10);
    let names = children.iter().filter_map(|item| match item.body {
        CatalogBody::Folder(_) => {
            Some(item.get_key().node_name.to_string().nfc().collect::<String>())
        },
        CatalogBody::File(_) => {
            //println!("{:?}", item.get_key().node_name.
            Some(item.get_key().node_name.to_string().nfc().collect::<String>())
        },
        _ => None
    }).collect::<Vec<String>>();
    assert!(names.contains(&"first".to_string()));
    assert!(names.contains(&"script.sh".to_string()));
    assert!(names.contains(&"Açaí".to_string()));

    let children_res = vol2.get_path("/files/second");
    if children_res.is_err() {
        println!("{:?}", children_res.as_ref().err().unwrap());
    }
    assert!(children_res.is_ok(), "Failed to search path for children");
    let children = children_res.unwrap();
    assert_eq!(children.len(), 0);
}
