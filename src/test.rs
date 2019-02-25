//use crate::*;
use super::*;

use std::io::Cursor;
use std::io::Error;

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
