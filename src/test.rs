extern crate unicode_normalization;

use rand::{thread_rng, Rng};

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

fn create_dead_beef(count: usize) -> Vec<u8> {
    [0xDEu8, 0xAD, 0xBE, 0xEF].into_iter().cloned().cycle().take(count).collect()
    //std::iter::repeat([0xDEu8, 0xAD, 0xBE, 0xEF].iter()).take(count).collect()
    //let mut raw_data = Vec::with_capacity(count);
    //for idx in 0..count {
    //    let val = match idx % 4 {
    //        0 => 0xde,
    //        1 => 0xad,
    //        2 => 0xbe,
    //        3 => 0xef,
    //        _ => { panic!("Modulus failure"); }
    //    };
    //    raw_data.push(val);
    //}
    //raw_data
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
fn store_hfs_volume_header() {
    let mut file = File::open("hfsp-blank.img").expect("Failed to open blank volume image");
    file.seek(std::io::SeekFrom::Start(1024)).expect("Failed to seek to volume header");
    //let mut reference_buffer = [0u8; 512];
    //let mut reference_buffer = Vec::new();
    let mut reference_buffer = vec![0u8; 512];
    file.read_exact(&mut reference_buffer).expect("Failed to read volume header");
    let header = blank_volume_header();
    //let mut actual_buffer = [0u8, 512];
    //header.export(&mut Cursor::new(actual_buffer)).expect("Failed to save volume header");
    //let mut actual_buffer = vec![0u8; 012];
    let mut actual_buffer = Vec::new();
    //println!("{:?}", actual_buffer);
    header.export(&mut actual_buffer).expect("Failed to save volume header");
    //header.export(&mut Cursor::new(actual_buffer)).expect("Failed to save volume header");
    //println!("{:?}", actual_buffer);
    assert_eq!(actual_buffer.len(), 512);
    assert!(actual_buffer == reference_buffer, "Actual buffer does not equal reference");
}

#[test]
fn save_restore_random_hfs_volume_header() {
    let mut reference_buffer = [0u8; 512];
    thread_rng().fill(&mut reference_buffer);
    reference_buffer[4] = 0;  // XXX Clear out the Volume Attributes
    reference_buffer[5] = 0;  // As unknown bits are currently dropped
    reference_buffer[6] = 0;  // in translation.  Should this be fixed?
    reference_buffer[7] = 0;
    let header = HFSPlusVolumeHeader::import(&mut &reference_buffer[..]).expect("Failed to read Volume Header");
    let reference_buffer = reference_buffer.to_vec();
    let mut actual_buffer = Vec::new();
    header.export(&mut actual_buffer).expect("Failed to save volume header");
    //println!("{:?}", reference_buffer);
    //println!("{:?}", actual_buffer);
    assert_eq!(actual_buffer.len(), 512);
    assert!(actual_buffer == reference_buffer, "Actual buffer does not equal reference");
}

#[test]
fn load_raw_extent_key() {
    let raw_data = [
        0, 10,          // Key length = kHFSPlusExtentKeyMaximumLength
        0xff,           // Fork type = resource (0xff)
        0,              // Pad
        0, 0, 2, 47,    // Catalog ID = 559
        0, 0, 3, 34,    // Start block = 802
    ];
    let key_result = HFSPlusExtentKey::import(&mut &raw_data[..]);
    assert!(key_result.is_ok(), "Failed to read extent key");
    let key = key_result.unwrap();
    assert_eq!(key.keyLength, kHFSPlusExtentKeyMaximumLength);
    assert_eq!(key.forkType, 255);
    assert_eq!(key.fileID, 559);
    assert_eq!(key.startBlock, 802);

    let mut actual_buffer = Vec::new();
    key.export(&mut actual_buffer).expect("Failed to save extent overflow key");
    assert_eq!(actual_buffer.len(), raw_data.len());
    assert_eq!(actual_buffer, raw_data, "Actual buffer does not equal reference key");
}

#[test]
fn load_btree_extent_key() {
    let raw_data = [
        0, 10,          // Key length = kHFSPlusExtentKeyMaximumLength
        0xff,           // Fork type = resource (0xff)
        0,              // Pad
        0, 0, 2, 47,    // Catalog ID = 559
        0, 0, 3, 34,    // Start block = 802
    ];
    let key_result = ExtentKey::import(&mut &raw_data[..]);
    assert!(key_result.is_ok(), "Failed to read extent key: {:?}", key_result.unwrap_err());
    let key = key_result.unwrap();
    let expected = ExtentKey::new(559, 0xff, 802);
    assert_eq!(key, expected);

    let mut actual_buffer = Vec::new();
    key.export(&mut actual_buffer).expect("Failed to save extent overflow key");
    assert_eq!(actual_buffer.len(), raw_data.len());
    assert_eq!(actual_buffer, raw_data, "Actual buffer does not equal reference key");
}

#[test]
fn test_extent_key_sort_order() {
    let lowest  = ExtentKey::new(27, 0x00, 0);
    let low     = ExtentKey::new(27, 0x00, 802);
    let mid     = ExtentKey::new(27, 0xff, 13);
    let high    = ExtentKey::new(27, 0xff, 400);
    let highest = ExtentKey::new(59, 0x00, 0);

    assert_ne!(lowest, low);
    assert_ne!(lowest, mid);
    assert_ne!(lowest, high);
    assert_ne!(lowest, highest);
    assert_ne!(low, mid);
    assert_ne!(low, high);
    assert_ne!(low, highest);
    assert_ne!(mid, high);
    assert_ne!(mid, highest);
    assert_ne!(high, highest);

    assert_eq!(lowest .partial_cmp(&lowest),  Some(Ordering::Equal));
    assert_eq!(lowest .partial_cmp(&low),     Some(Ordering::Less));
    assert_eq!(lowest .partial_cmp(&mid),     Some(Ordering::Less));
    assert_eq!(lowest .partial_cmp(&high),    Some(Ordering::Less));
    assert_eq!(lowest .partial_cmp(&highest), Some(Ordering::Less));
    assert_eq!(low    .partial_cmp(&lowest),  Some(Ordering::Greater));
    assert_eq!(low    .partial_cmp(&low),     Some(Ordering::Equal));
    assert_eq!(low    .partial_cmp(&mid),     Some(Ordering::Less));
    assert_eq!(low    .partial_cmp(&high),    Some(Ordering::Less));
    assert_eq!(low    .partial_cmp(&highest), Some(Ordering::Less));
    assert_eq!(mid    .partial_cmp(&lowest),  Some(Ordering::Greater));
    assert_eq!(mid    .partial_cmp(&low),     Some(Ordering::Greater));
    assert_eq!(mid    .partial_cmp(&mid),     Some(Ordering::Equal));
    assert_eq!(mid    .partial_cmp(&high),    Some(Ordering::Less));
    assert_eq!(mid    .partial_cmp(&highest), Some(Ordering::Less));
    assert_eq!(high   .partial_cmp(&lowest),  Some(Ordering::Greater));
    assert_eq!(high   .partial_cmp(&low),     Some(Ordering::Greater));
    assert_eq!(high   .partial_cmp(&mid),     Some(Ordering::Greater));
    assert_eq!(high   .partial_cmp(&high),    Some(Ordering::Equal));
    assert_eq!(high   .partial_cmp(&highest), Some(Ordering::Less));
    assert_eq!(highest.partial_cmp(&lowest),  Some(Ordering::Greater));
    assert_eq!(highest.partial_cmp(&low),     Some(Ordering::Greater));
    assert_eq!(highest.partial_cmp(&mid),     Some(Ordering::Greater));
    assert_eq!(highest.partial_cmp(&high),    Some(Ordering::Greater));
    assert_eq!(highest.partial_cmp(&highest), Some(Ordering::Equal));
}

#[test]
fn load_extents_leaf_node() {
    let mut raw_data: Vec<u8> = vec![
        0, 1, 0, 4,         // fLink = 65540
        0, 0, 2, 9,         // bLink = 521
        kBTLeafNode as u8,  // kind = leaf node
        1,                  // height = 1
        0, 2,               // numRecords = 2
        0, 0,               // reserved
    ];
    let extents1_key = ExtentKey::new(59, 0x00, 87);
    let extents1 = [
        HFSPlusExtentDescriptor { startBlock: 2, blockCount: 6, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
    ];
    let extents2_key = ExtentKey::new(59, 0x00, 145);
    let extents2 = [
        HFSPlusExtentDescriptor { startBlock: 9, blockCount: 6, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
    ];
    let extents1_pos = raw_data.len() as u16;
    extents1_key.export(&mut raw_data);
    export_record(&extents1, &mut raw_data).unwrap();
    let extents2_pos = raw_data.len() as u16;
    extents2_key.export(&mut raw_data);
    export_record(&extents2, &mut raw_data).unwrap();
    let free_pos = raw_data.len() as u16;
    raw_data.resize(512 - 2*3, 0);
    raw_data.write_u16::<BigEndian>(free_pos).unwrap();
    raw_data.write_u16::<BigEndian>(extents2_pos).unwrap();
    raw_data.write_u16::<BigEndian>(extents1_pos).unwrap();
    assert_eq!(raw_data.len(), 512);

    let node = Node::<ExtentKey, ExtentRecord>::load(&raw_data)
        .expect("Fail to load extent leaf node");

    let leaf = match node {
        Node::LeafNode(x) => x,
        _ => { panic!("Node is not a leaf"); },
    };
    assert_eq!(leaf.descriptor.fLink, 65540);
    assert_eq!(leaf.descriptor.bLink, 521);
    assert_eq!(leaf.descriptor.height, 1);
    assert_eq!(leaf.descriptor.numRecords, 2);
    assert_eq!(leaf.records.len(), 2);

    assert_eq!(leaf.records[0].get_key(), &extents1_key);
    assert_eq!(&leaf.records[0].body, &extents1);
    assert_eq!(leaf.records[1].get_key(), &extents2_key);
    assert_eq!(&leaf.records[1].body, &extents2);

    let mut actual_buffer = Vec::new();
    leaf.descriptor.export(&mut actual_buffer);
    assert_eq!(actual_buffer.len(), 14);
    assert!(raw_data.starts_with(&actual_buffer));
    let mut node_buffer = [0u8; 512];
    Node::LeafNode(leaf).save(&mut node_buffer[..]).expect("Unable to save extent node");
    assert_eq!(&node_buffer.to_vec(), &raw_data);
}

#[test]
fn load_save_header_node() {
    let mut raw_data: Vec<u8> = vec![
        0, 0, 20, 7,            // fLink = 5127
        0, 0, 4, 0,             // bLink = 1024
        kBTHeaderNode as u8,    // kind = header node
        0,                      // height = 0
        0, 3,                   // numRecords = 3
        0, 0,                   // reserved
    ];

    let header_node: [u8; 106] = [
        0, 1,                   // treeDepth = 1
        0, 0, 0, 0,             // rootNode = 
        0, 0, 0, 0,             // leafRecords = 
        0, 0, 0, 0,             // firstLeafNode = 
        0, 0, 0, 0,             // lastLeafNode = 
        0, 0,                   // nodeSize = 
        0, 0,                   // maxKeyLength = 
        0, 0, 0, 0,             // totalNodes = 
        0, 0, 0, 0,             // freeNodes = 
        0, 0,                   // reserved1 = 
        0, 0, 0, 0,             // clumpSize = 
        0,                      // btreeType = 
        0,                      // keyCompareType = 
        0, 0, 0, 0,             // attributes = 
        0, 0, 0, 0,             // reserved3[16]
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    // Generate random data for the user data record in the header node
    let mut user_data_buffer = [0u8; 128];
    thread_rng().fill(&mut user_data_buffer);
    let map_node = [0u8; 256];
    raw_data.extend(header_node.iter());
    raw_data.extend(user_data_buffer.iter());
    raw_data.extend(map_node.iter());
    // First record always starts at offset 14
    // The Header record is always 106 bytes in length
    // The User Data is always 128 bytes
    // 4 16-bit record pointers are needed at the end (one for next free location)
    // Making the Map Record 256 bytes rounds out the node to 512 bytes
    raw_data.write_u16::<BigEndian>(14+106+128+256).unwrap();
    raw_data.write_u16::<BigEndian>(14+106+128).unwrap();
    raw_data.write_u16::<BigEndian>(14+106).unwrap();
    raw_data.write_u16::<BigEndian>(14).unwrap();
    assert_eq!(raw_data.len(), 512);
    let node = Node::<CatalogKey, CatalogRecord>::load(&raw_data).expect("Unable to load header node");
    let header = match node {
        Node::HeaderNode(x) => x,
        _ => { panic!("Wrong node type"); },
    };
    assert_eq!(header.descriptor.fLink, 5127);
    assert_eq!(header.descriptor.bLink, 1024);
    assert_eq!(header.descriptor.height, 0);
    assert_eq!(header.descriptor.numRecords, 3);
    assert_eq!(&header.user_data[..], &user_data_buffer[..]);
    assert_eq!(&header.map[..], &map_node[..]);

    let mut node_buffer = [0u8; 512];
    Node::HeaderNode::<CatalogKey, CatalogRecord>(header).save(&mut node_buffer[..]).expect("Unable to save header node");
    assert_eq!(&node_buffer.to_vec(), &raw_data);
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
        forks: HashMap::new(),
        catalog_btree: None,
        extents_btree: None,
    }));
    let mut fork = Fork::load(file, volume, &fork_data).expect("Failed to load Fork");
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
        forks: HashMap::new(),
        catalog_btree: None,
        extents_btree: None,
    }));
    let mut fork = Fork::load(file, volume, &fork_data).expect("Failed to load Fork");
    assert!(fork.check().is_ok(), "Errors found in fork data");
}

#[test]
fn load_fragmented_fork_data() {
    let mut header = empty_v4_volume_header();
    let mut raw_data = create_dead_beef(1024);
    header.blockSize = 4;   // Really small block size to ease testing
    header.catalogFile.logicalSize = 33;
    header.catalogFile.totalBlocks = 9;
    header.catalogFile.extents[0].startBlock = 10;
    header.catalogFile.extents[0].blockCount = 3;
    header.catalogFile.extents[1].startBlock = 7;
    header.catalogFile.extents[1].blockCount = 1;
    header.catalogFile.extents[2].startBlock = 14;
    header.catalogFile.extents[2].blockCount = 2;
    header.catalogFile.extents[3].startBlock = 17;
    header.catalogFile.extents[3].blockCount = 2;
    header.catalogFile.extents[4].startBlock = 4;
    header.catalogFile.extents[4].blockCount = 1;
    raw_data[4*10+0] = 'H' as u8;
    raw_data[4*10+1] = 'e' as u8;
    raw_data[4*10+2] = 'l' as u8;
    raw_data[4*10+3] = 'l' as u8;
    raw_data[4*11+0] = 'o' as u8;
    raw_data[4*11+1] = ',' as u8;
    raw_data[4*11+2] = ' ' as u8;
    raw_data[4*11+3] = 'W' as u8;
    raw_data[4*12+0] = 'o' as u8;
    raw_data[4*12+1] = 'r' as u8;
    raw_data[4*12+2] = 'l' as u8;
    raw_data[4*12+3] = 'd' as u8;
    raw_data[4*07+0] = '!' as u8;
    raw_data[4*07+1] = '\n' as u8;
    raw_data[4*07+2] = 'W' as u8;
    raw_data[4*07+3] = 'h' as u8;
    raw_data[4*14+0] = 'a' as u8;
    raw_data[4*14+1] = 't' as u8;
    raw_data[4*14+2] = ' ' as u8;
    raw_data[4*14+3] = 'i' as u8;
    raw_data[4*15+0] = 's' as u8;
    raw_data[4*15+1] = ' ' as u8;
    raw_data[4*15+2] = 'y' as u8;
    raw_data[4*15+3] = 'o' as u8;
    raw_data[4*17+0] = 'u' as u8;
    raw_data[4*17+1] = 'r' as u8;
    raw_data[4*17+2] = ' ' as u8;
    raw_data[4*17+3] = 'n' as u8;
    raw_data[4*18+0] = 'a' as u8;
    raw_data[4*18+1] = 'm' as u8;
    raw_data[4*18+2] = 'e' as u8;
    raw_data[4*18+3] = '?' as u8;
    raw_data[4*04+0] = '\n' as u8;
    let volume = Rc::new(RefCell::new(HFSVolume {
        file: Rc::new(RefCell::new(Cursor::new(raw_data))),
        header,
        forks: HashMap::new(),
        catalog_btree: None,
        extents_btree: None,
    }));
    let mut fork = Fork::load(Rc::clone(&volume.borrow().file), Rc::clone(&volume), &volume.borrow().header.catalogFile).unwrap();
    let mut buffer = [0u8; 33];
    fork.seek(std::io::SeekFrom::Start(0)).unwrap();
    fork.read(buffer.as_mut()).unwrap();
    //println!("Buffer: {:?}", buffer.to_vec());
    let expected: Vec<u8> = "Hello, World!\nWhat is your name?\n".bytes().collect();
    println!("Expected:");
    for row in 0..(expected.len()+16-1) / 16 {
        print!("  ");
        for idx in 0..8 {
            if row*16 + idx < expected.len() {
                print!("{:02x} ", expected[row*16 + idx]);
            }
        }
        for idx in 8..16 {
            if row*16 + idx < expected.len() {
                print!(" {:02x}", expected[row*16 + idx]);
            }
        }
        println!("");
    }
    println!("Buffer:");
    for row in 0..(buffer.len()+16-1) / 16 {
        print!("  ");
        for idx in 0..8 {
            if row*16 + idx < buffer.len() {
                print!("{:02x} ", buffer[row*16 + idx]);
            }
        }
        for idx in 8..16 {
            if row*16 + idx < buffer.len() {
                print!(" {:02x}", buffer[row*16 + idx]);
            }
        }
        println!("");
    }
    let result = std::str::from_utf8(buffer.as_ref());
    assert_eq!(result.unwrap(), "Hello, World!\nWhat is your name?\n");
}

/// write_extents_overflow_file(source, extents);
///
/// Helper function for creating a simple extents overflow file
/// This is a very simple function just used for constructing a test vector
/// for a small filesystem. It assumes that the provided key/records are
/// pre-sorted, less than 256, and all fit in one leaf node. It also assumes
/// a small node size of 512 bytes which is not typical, but easier for
/// testing.
///
/// This is written to hand-craft the data and avoid relying on must of the
/// normal filesystem code that will be ultimately tested.
///
fn write_extents_overflow_file(source: &mut Write, extents: &[(ExtentKey, HFSPlusExtentRecord)]) {
    let mut raw_header_data: Vec<u8> = vec![
        0, 0, 0, 0,             // fLink = 0
        0, 0, 0, 0,             // bLink = 0
        kBTHeaderNode as u8,    // kind = header node
        0,                      // height = 0
        0, 3,                   // numRecords = 3
        0, 0,                   // reserved
    ];

    let header_node: [u8; 106] = [
        0, 1,                   // treeDepth = 1
        0, 0, 0, 1,             // rootNode = 1
        0, 0, 0, 1,             // leafRecords = 1
        0, 0, 0, 1,             // firstLeafNode = 1
        0, 0, 0, 1,             // lastLeafNode = 1
        2, 0,                   // nodeSize = 512
        0, 10,                  // maxKeyLength = kHFSPlusExtentKeyMaximumLength
        0, 0, 0, 1,             // totalNodes = 1
        0, 0, 0, 0,             // freeNodes = 0
        0, 0,                   // reserved1
        0, 0, 2, 0,             // clumpSize = 512
        0,                      // btreeType = kHFSBTreeType
        0,                      // keyCompareType = 0
        0, 0, 0, 2,             // attributes = kBTBigKeysMask
        0, 0, 0, 0,             // reserved3[16]
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    let user_data_buffer = [0u8; 128];
    let map_node = [0u8; 256];
    raw_header_data.extend(header_node.iter());
    raw_header_data.extend(user_data_buffer.iter());
    raw_header_data.extend(map_node.iter());
    // First record always starts at offset 14
    // The Header record is always 106 bytes in length
    // The User Data is always 128 bytes
    // 4 16-bit record pointers are needed at the end (one for next free location)
    // Making the Map Record 256 bytes rounds out the node to 512 bytes
    raw_header_data.write_u16::<BigEndian>(14+106+128+256).unwrap();
    raw_header_data.write_u16::<BigEndian>(14+106+128).unwrap();
    raw_header_data.write_u16::<BigEndian>(14+106).unwrap();
    raw_header_data.write_u16::<BigEndian>(14).unwrap();
    assert_eq!(raw_header_data.len(), 512);

    let mut raw_data: Vec<u8> = vec![
        0, 0, 0, 0,         // fLink = 0
        0, 0, 0, 0,         // bLink = 0
        kBTLeafNode as u8,  // kind = leaf node
        1,                  // height = 1
        0, 0,               // numRecords = 0
        0, 0,               // reserved
    ];
    raw_data[11] = extents.len() as u8;  // Overwrite the LSB of numRecords
    let mut positions = Vec::new();
    for (key, record) in extents {
        positions.push(raw_data.len() as u16);
        key.export(&mut raw_data);
        export_record(&record[..], &mut raw_data).unwrap();
    }
    positions.push(raw_data.len() as u16);
    assert!(raw_data.len() <= 512 - 2*positions.len(), "Overflowed extents leaf record");
    raw_data.resize(512 - 2*positions.len(), 0);
    for position in positions {
        raw_data.write_u16::<BigEndian>(position).unwrap();
    }
    assert_eq!(raw_data.len(), 512);
}

#[ignore]
#[test]
fn load_dummy_overflow_file() {
    let extents = [
        (ExtentKey::new(41, 0x32, 87), [
            HFSPlusExtentDescriptor { startBlock:  2, blockCount: 2, },
            HFSPlusExtentDescriptor { startBlock: 10, blockCount: 1, },
            HFSPlusExtentDescriptor { startBlock: 12, blockCount: 1, },
            HFSPlusExtentDescriptor { startBlock:  4, blockCount: 1, },
            HFSPlusExtentDescriptor { startBlock: 15, blockCount: 3, },
            HFSPlusExtentDescriptor { startBlock:  0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock:  0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock:  0, blockCount: 0, },
        ]),
        (ExtentKey::new(59, 0x00, 145), [
            HFSPlusExtentDescriptor { startBlock: 9, blockCount: 6, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
            HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0, },
        ]),
    ];
    let mut buffer = Vec::new();
    write_extents_overflow_file(&mut buffer, &extents[..]);
    assert_eq!(buffer.len(), 1024);  // Size of 2 nodes
    //let btree = BTree::open(Rc::new(RefCell::new(Cursor::new(buffer)))).except("Failed to open overflow b-tree");
}

#[ignore]
#[test]
fn load_fragmented_fork_data_with_overflow() {
    let mut header = empty_v4_volume_header();
    let mut raw_data = create_dead_beef(1024);
    header.blockSize = 4;   // Really small block size to ease testing
    header.catalogFile.logicalSize = 256;
    header.catalogFile.totalBlocks = 9;
    let mut overflow_extents = [
        new_record(),
        new_record(),
        new_record(),
    ];
    let extents = [
        (  5, 3), ( 11, 2), ( 17, 5), (  3, 1),  // 11*4 = 44 blocks (total  44)
        ( 40, 2), ( 25, 4), ( 50, 1), ( 54, 3),  // 10*4 = 40 blocks (total  84)
        ( 60, 1), ( 64, 1), ( 66, 5), ( 73, 1),  //  8*4 = 32 blocks (total 116)
        ( 76, 2), ( 79, 4), ( 85, 1), ( 74, 2),  //  9*4 = 36 blocks (total 152)
        ( 30, 1), ( 34, 1), ( 36, 1), ( 85, 1),  //  4*4 = 16 blocks (total 168)
        (100, 5), (110, 6), (120, 2), (125, 4),  // 17*4 = 68 blocks (total 236)
        (130, 2), (157, 1), (140, 2), (  0, 0),  //  5*4 = 20 blocks (total 256)
    ];
    let mut expected = Vec::new();
    let mut value = 0u16;
    for (idx, (start, size)) in extents.iter().enumerate() {
        let group = match idx {
            x if x < 8  => &mut header.catalogFile.extents[idx],
            x if x < 16 => &mut overflow_extents[0][idx-8],
            x if x < 24 => &mut overflow_extents[1][idx-16],
            x if x < 32 => &mut overflow_extents[2][idx-24],
            _ => { panic!("No extents left"); }
        };

        group.startBlock = *start as u32;
        group.blockCount = *size as u32;
        for i in 0..4**size {
            raw_data[*start + i] = value as u8;
            expected.push(value as u8);
            value += 1;
        }
    }
    let volume = Rc::new(RefCell::new(HFSVolume {
        file: Rc::new(RefCell::new(Cursor::new(raw_data))),
        header,
        forks: HashMap::new(),
        catalog_btree: None,
        extents_btree: None,
    }));
    let mut fork = Fork::load(Rc::clone(&volume.borrow().file), Rc::clone(&volume), &volume.borrow().header.catalogFile).unwrap();
    let mut buffer = [0u8; 256];
    fork.seek(std::io::SeekFrom::Start(0)).unwrap();
    fork.read(buffer.as_mut()).unwrap();
    //println!("Buffer: {:?}", buffer.to_vec());
    println!("Expected:");
    for row in 0..(expected.len()+16-1) / 16 {
        print!("  ");
        for idx in 0..8 {
            if row*16 + idx < expected.len() {
                print!("{:02x} ", expected[row*16 + idx]);
            }
        }
        for idx in 8..16 {
            if row*16 + idx < expected.len() {
                print!(" {:02x}", expected[row*16 + idx]);
            }
        }
        println!("");
    }
    println!("Buffer:");
    for row in 0..(buffer.len()+16-1) / 16 {
        print!("  ");
        for idx in 0..8 {
            if row*16 + idx < buffer.len() {
                print!("{:02x} ", buffer[row*16 + idx]);
            }
        }
        for idx in 8..16 {
            if row*16 + idx < buffer.len() {
                print!(" {:02x}", buffer[row*16 + idx]);
            }
        }
        println!("");
    }
    assert_eq!(buffer.len(), expected.len());
    assert_eq!(buffer.to_vec(), expected);
}

#[test]
fn load_beyond_end_of_fork_extents() {
    let mut header = empty_v4_volume_header();
    let mut raw_data = create_dead_beef(1024);
    header.blockSize = 4;   // Really small block size to ease testing
    header.catalogFile.logicalSize = 33;
    header.catalogFile.totalBlocks = 9;
    header.catalogFile.extents[0].startBlock = 10;
    header.catalogFile.extents[0].blockCount = 3;
    header.catalogFile.extents[1].startBlock = 7;
    header.catalogFile.extents[1].blockCount = 1;
    header.catalogFile.extents[2].startBlock = 14;
    header.catalogFile.extents[2].blockCount = 2;
    header.catalogFile.extents[3].startBlock = 17;
    header.catalogFile.extents[3].blockCount = 2;
    header.catalogFile.extents[4].startBlock = 4;
    header.catalogFile.extents[4].blockCount = 1;
    let volume = Rc::new(RefCell::new(HFSVolume {
        file: Rc::new(RefCell::new(Cursor::new(raw_data))),
        header,
        forks: HashMap::new(),
        catalog_btree: None,
        extents_btree: None,
    }));
    let mut fork = Fork::load(Rc::clone(&volume.borrow().file), Rc::clone(&volume), &volume.borrow().header.catalogFile).unwrap();
    let mut buffer = [0u8; 37];  // Note, this is one more byte than in fork extents
    fork.seek(SeekFrom::Start(0)).unwrap();
    let result = fork.read(buffer.as_mut());
    assert!(result.is_err(), "Failed to trigger error in read()");
}

#[test]
fn load_beyond_end_of_fork_data() {
    let mut header = empty_v4_volume_header();
    let mut raw_data = create_dead_beef(1024);
    header.blockSize = 4;   // Really small block size to ease testing
    header.catalogFile.logicalSize = 33;
    header.catalogFile.totalBlocks = 9;
    header.catalogFile.extents[0].startBlock = 10;
    header.catalogFile.extents[0].blockCount = 3;
    header.catalogFile.extents[1].startBlock = 7;
    header.catalogFile.extents[1].blockCount = 1;
    header.catalogFile.extents[2].startBlock = 14;
    header.catalogFile.extents[2].blockCount = 2;
    header.catalogFile.extents[3].startBlock = 17;
    header.catalogFile.extents[3].blockCount = 2;
    header.catalogFile.extents[4].startBlock = 4;
    header.catalogFile.extents[4].blockCount = 1;
    let volume = Rc::new(RefCell::new(HFSVolume {
        file: Rc::new(RefCell::new(Cursor::new(raw_data))),
        header,
        forks: HashMap::new(),
        catalog_btree: None,
        extents_btree: None,
    }));
    let mut fork = Fork::load(Rc::clone(&volume.borrow().file), Rc::clone(&volume), &volume.borrow().header.catalogFile).unwrap();
    let mut buffer = [0u8; 34];  // Note, this is one more byte than in fork data
                                 // but still resides inside fork extent
    fork.seek(SeekFrom::Start(0)).unwrap();
    let result = fork.read(buffer.as_mut());
    assert!(result.is_err(), "Failed to trigger error in read()");
}

//#[test]
//fn load_blank_volume_catalog_fork() {
//    let volume = HFSVolume::load_file("hfsp-blank.img").expect("Failed to read Volume Header");
//    assert!(volume.borrow().catalog_btree.is_some(), "Invalid catalog fork pointer");
//    assert!(volume.borrow().extents_btree.is_some(), "Invalid extents fork pointer");
//    let vol = volume.borrow();
//    let mut catalog_btree = vol.catalog_btree.unwrap().borrow_mut();
//    let fork = catalog_btree.fork;
//    assert_eq!(fork.len(), 32768);
//    let mut extents_btree = vol.extents_btree.unwrap().borrow_mut();
//    let extents_fork = extents_btree.fork;
//    assert_eq!(extents_fork.len(), 32768);
//    //let mut buffer = vec![0; 512];
//    let mut buffer = vec![0; 512];
//    fork.read(0, &mut buffer).expect("Failed to read from fork");
//    //let node = BTNodeDescriptor::import(&mut Cursor::new(&mut buffer)).unwrap();
//    let node = BTNodeDescriptor::import(&mut &buffer[..]).unwrap();
//    assert_eq!(node.kind, kBTHeaderNode);
//    assert_eq!(node.bLink, 0);
//    assert_eq!(node.numRecords, 3);
//    assert_eq!(node.reserved, 0);
//    let node_size = (&buffer[32..34]).read_u16::<BigEndian>().expect("Error decoding node size");
//    println!("{}", node_size);
//    assert_eq!(node_size, 4096);
//}

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
    let node = Node::<CatalogKey, CatalogRecord>::load(&node_data);
    assert!(node.is_err(), "All-zero node will have bad offsets");
    (&mut node_data[510..512]).write_u16::<BigEndian>(14).unwrap();
    let node = Node::<CatalogKey, CatalogRecord>::load(&node_data);
    assert!(node.is_ok(), "Empty node with valid pointers not OK");
    (&mut node_data[10..12]).write_u16::<BigEndian>(3).unwrap();  // 3 Records
    let node = Node::<CatalogKey, CatalogRecord>::load(&node_data);
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
    let root_node = {
        let btree = vol2.catalog_btree.as_ref().unwrap().borrow();
        let tree_header = &btree.header.header;
        assert_eq!(tree_header.rootNode, 1);
        assert_eq!(tree_header.firstLeafNode, 1);
        assert_eq!(tree_header.lastLeafNode, 1);
        tree_header.rootNode
    };
    let node = {
        let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
        btree.get_node(root_node as usize)
    };
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow();
    let tree_header = &btree.header.header;
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
    let root_node = {
        let btree = vol2.catalog_btree.as_ref().unwrap().borrow();
        let tree_header = &btree.header.header;
        assert_eq!(tree_header.treeDepth, 2);
        // Multiple leaf nodes are requires so all values
        // must be different
        assert_ne!(tree_header.rootNode, tree_header.firstLeafNode);
        assert_ne!(tree_header.rootNode, tree_header.lastLeafNode);
        assert_ne!(tree_header.firstLeafNode, tree_header.lastLeafNode);
        tree_header.rootNode
    };
    let node = {
        let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
        btree.get_node(root_node as usize)
    };
    let btree = vol2.catalog_btree.as_ref().unwrap().borrow();
    let tree_header = &btree.header.header;
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
    let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
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
    let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
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
    let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let thread_record_res = {
        let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
        btree.get_record(&root_thread_key)
    };
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
    let root_record_res = {
        let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
        btree.get_record(thread)
    };
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
    let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let thread_record_res = {
        let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
        btree.get_record(&root_thread_key)
    };
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
    let root_record_res = {
        let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
        btree.get_record(thread)
    };
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
    let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
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
    let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
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

#[test]
fn test_btree_get_record_range_many() {
    let volume = HFSVolume::load_file("hfsp-many2.img").expect("Failed to read Volume Header");
    let vol2 = volume.borrow();
    {
        let btree = vol2.catalog_btree.as_ref().unwrap().borrow();
        let tree_header = &btree.header.header;
        assert_eq!(tree_header.treeDepth, 3);  // This test expects index nodes
    }
    let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let thread_record_res = {
        let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
        btree.get_record(&root_thread_key)
    };
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
    let root_record_res = {
        let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
        btree.get_record(thread)
    };
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
    let records_res = {
        let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
        btree.get_record_range(&first, &last)
    };
    assert!(records_res.is_ok(), "Failed to get record range");
    let records = records_res.unwrap();
    assert!(records.len() > 0);
    assert!(records[0].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[0].get_key() < &last, "First key after end of range (exclusive)");
    assert!(records[records.len()-1].get_key() >= &first, "First key before beginning of range (inclusive)");
    assert!(records[records.len()-1].get_key() < &last, "First key after end of range (exclusive)");

    let first = CatalogKey { _case_match: false, parent_id: 1, node_name: HFSString::from("") };
    let last = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
    let mut btree = vol2.catalog_btree.as_ref().unwrap().borrow_mut();
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
