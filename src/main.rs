use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fs::File;
//use std::io::{Read, Write, Seek};
use std::io::{Error, ErrorKind, Read, Seek};
use std::collections::HashMap;
use std::cmp::Ordering;

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};

#[macro_use]
extern crate bitflags;

extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;
use hfs_strings::fast_unicode_compare;


use std::fmt;

struct HFSString(Vec<u16>);

impl fmt::Debug for HFSString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf16(self.0.as_slice()).unwrap())
    }
}

impl From<String> for HFSString {
    fn from(str: String) -> Self {
        HFSString(
            str.nfd().collect::<String>().encode_utf16().collect()
        )
    }
}

impl From<&str> for HFSString {
    fn from(str: &str) -> Self {
        HFSString(
            str.nfd().collect::<String>().encode_utf16().collect()
        )
    }
}

//impl HFSString {
//    fn new(data: &[u16]) {
//        HFSString { contents: vec![data] }
//    }
//}

impl PartialOrd for HFSString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(fast_unicode_compare(&self.0[..], &other.0[..]))
    }
}

impl Ord for HFSString {
    fn cmp(&self, other: &Self) -> Ordering {
        fast_unicode_compare(&self.0[..], &other.0[..])
    }
}

impl PartialEq for HFSString {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for HFSString {
}


//struct HFSUniStr255 {
//    UInt16  length;
//    UniChar unicode[255];
//};
//typedef struct HFSUniStr255 HFSUniStr255;
//typedef const  HFSUniStr255 *ConstHFSUniStr255Param;
//
//struct HFSPlusCatalogKey {
//    UInt16              keyLength;
//    HFSCatalogNodeID    parentID;
//    HFSUniStr255        nodeName;
//};
//typedef struct HFSPlusCatalogKey HFSPlusCatalogKey;

//type UniChar = u16;
//
//struct HFSUniStr255 {
//    length: u16,
//    //unicode: [UniChar; 255],
//    unicode: Vec<u16>,
//}
//
//impl HFSUniStr255 {
//    fn import(source: &mut Read) -> std::io::Result<Self> {
//        let length = source.read_u16::<BigEndian>()?;
//        let mut unicode = Vec::with_capacity(length as usize);
//        for _ in 0..length {
//            unicode.push(source.read_u16::<BigEndian>()?);
//        }
//        Ok(Self {
//            length,
//            unicode,
//        })
//    }
//}
//
//impl HFSPlusCatalogKey {
//    fn import(source: &mut Read) -> std::io::Result<Self> {
//        Ok(Self {
//            keyLength: source.read_u16::<BigEndian>()?,
//            parentID: source.read_u32::<BigEndian>()?,
//            nodeName: HFSUniStr255::import(source)?,
//        })
//    }
//}
//
//struct HFSPlusCatalogKey {
//    keyLength: u16,
//    parentID: HFSCatalogNodeID,
//    nodeName: HFSUniStr255,
//}


#[derive(Debug)]
struct CatalogKey {
    _case_match: bool,
    parent_id: HFSCatalogNodeID,
    node_name: HFSString,
}

impl CatalogKey {
    fn import(source: &mut Read) -> HFSResult<Self> {
        let key_length = source.read_u16::<BigEndian>()?;
        if key_length < 6 {
            return Err(HFSError::InvalidRecordKey);
        }
        let parent_id = source.read_u32::<BigEndian>()?;
        let count = source.read_u16::<BigEndian>()?;
        if key_length != count*2 + 6 {
            return Err(HFSError::InvalidRecordKey);
        }
        let mut node_name = Vec::with_capacity(count as usize);
        for _ in 0..count as usize {
            node_name.push(source.read_u16::<BigEndian>()?);
        }
        Ok(Self {
            _case_match: false,
            parent_id,
            node_name: HFSString(node_name),
        })
    }
}

impl PartialOrd for CatalogKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CatalogKey {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.parent_id.cmp(&other.parent_id) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.node_name.cmp(&other.node_name),
        }
    }
}

impl PartialEq for CatalogKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for CatalogKey {
}

impl Key for CatalogKey {
}

trait Record {
    fn new(key: CatalogKey) -> Self;
}

struct CatalogRecord {
    key: CatalogKey,
}

impl Record for CatalogRecord {
    fn new(key: CatalogKey) -> CatalogRecord {
        CatalogRecord { key }
    }
}


//typedef UInt32 HFSCatalogNodeID;
//
//enum {
//    kHFSRootParentID            = 1,
//    kHFSRootFolderID            = 2,
//    kHFSExtentsFileID           = 3,
//    kHFSCatalogFileID           = 4,
//    kHFSBadBlockFileID          = 5,
//    kHFSAllocationFileID        = 6,
//    kHFSStartupFileID           = 7,
//    kHFSAttributesFileID        = 8,
//    kHFSRepairCatalogFileID     = 14,
//    kHFSBogusExtentFileID       = 15,
//    kHFSFirstUserCatalogNodeID  = 16
//};

type HFSCatalogNodeID = u32;

#[allow(non_upper_case_globals, unused_variables)]
mod hfs {
use super::HFSCatalogNodeID;
pub const kHFSRootParentID           : HFSCatalogNodeID = 1;
pub const kHFSRootFolderID           : HFSCatalogNodeID = 2;
pub const kHFSExtentsFileID          : HFSCatalogNodeID = 3;
pub const kHFSCatalogFileID          : HFSCatalogNodeID = 4;
pub const kHFSBadBlockFileID         : HFSCatalogNodeID = 5;
pub const kHFSAllocationFileID       : HFSCatalogNodeID = 6;
pub const kHFSStartupFileID          : HFSCatalogNodeID = 7;
pub const kHFSAttributesFileID       : HFSCatalogNodeID = 8;
pub const kHFSRepairCatalogFileID    : HFSCatalogNodeID = 14;
pub const kHFSBogusExtentFileID      : HFSCatalogNodeID = 15;
pub const kHFSFirstUserCatalogNodeID : HFSCatalogNodeID = 16;

pub const kBTLeafNode     :i8  = -1;
pub const kBTIndexNode    :i8  =  0;
pub const kBTHeaderNode   :i8  =  1;
pub const kBTMapNode      :i8  =  2;
}
use self::hfs::*;


//struct HFSPlusExtentDescriptor {
//    UInt32                  startBlock;
//    UInt32                  blockCount;
//};
//typedef struct HFSPlusExtentDescriptor HFSPlusExtentDescriptor;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
struct HFSPlusExtentDescriptor {
    startBlock: u32,
    blockCount: u32,
}


//struct HFSPlusForkData {
//    UInt64                  logicalSize;
//    UInt32                  clumpSize;
//    UInt32                  totalBlocks;
//    HFSPlusExtentRecord     extents;
//};
//typedef struct HFSPlusForkData HFSPlusForkData;
//
//typedef HFSPlusExtentDescriptor HFSPlusExtentRecord[8];

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
struct HFSPlusForkData {
    logicalSize: u64,
    clumpSize: u32,
    totalBlocks: u32,
    extents: HFSPlusExtentRecord,
}

type HFSPlusExtentRecord = [HFSPlusExtentDescriptor; 8];

impl HFSPlusForkData {
    fn import(source: &mut Read) -> std::io::Result<Self> {
        Ok(Self {
            logicalSize: source.read_u64::<BigEndian>()?,
            clumpSize: source.read_u32::<BigEndian>()?,
            totalBlocks: source.read_u32::<BigEndian>()?,
            extents: import_record(source)?,
        })
    }
}

//impl HFSPlusExtentRecord {
    //fn import(source: &mut Read) -> std::io::Result<Self> {
    fn import_record(source: &mut Read) -> std::io::Result<HFSPlusExtentRecord> {
        Ok([
           HFSPlusExtentDescriptor::import(source)?,
           HFSPlusExtentDescriptor::import(source)?,
           HFSPlusExtentDescriptor::import(source)?,
           HFSPlusExtentDescriptor::import(source)?,
           HFSPlusExtentDescriptor::import(source)?,
           HFSPlusExtentDescriptor::import(source)?,
           HFSPlusExtentDescriptor::import(source)?,
           HFSPlusExtentDescriptor::import(source)?,
        ])
    }
//}

impl HFSPlusExtentDescriptor {
    fn import(source: &mut Read) -> std::io::Result<Self> {
        Ok(Self {
            startBlock: source.read_u32::<BigEndian>()?,
            blockCount: source.read_u32::<BigEndian>()?,
        })
    }
}


//struct BTNodeDescriptor {
//    UInt32    fLink;
//    UInt32    bLink;
//    SInt8     kind;
//    UInt8     height;
//    UInt16    numRecords;
//    UInt16    reserved;
//};
//typedef struct BTNodeDescriptor BTNodeDescriptor;
//
//enum {
//    kBTLeafNode       = -1,
//    kBTIndexNode      =  0,
//    kBTHeaderNode     =  1,
//    kBTMapNode        =  2
//};

//#[allow(non_upper_case_globals)]
//mod hfs {
//const kBTLeafNode     :i8  = -1;
//const kBTIndexNode    :i8  =  0;
//const kBTHeaderNode   :i8  =  1;
//const kBTMapNode      :i8  =  2;
//}


#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
struct BTNodeDescriptor {
    fLink: u32,
    bLink: u32,
    kind: i8,
    height: u8,
    numRecords: u16,
    reserved: u16,
}

impl BTNodeDescriptor {
    fn import(source: &mut Read) -> std::io::Result<Self> {
        Ok(Self {
            fLink: source.read_u32::<BigEndian>()?,
            bLink: source.read_u32::<BigEndian>()?,
            kind: source.read_i8()?,
            height: source.read_u8()?,
            numRecords: source.read_u16::<BigEndian>()?,
            reserved: source.read_u16::<BigEndian>()?,
        })
    }
}


//struct BTHeaderRec {
//    UInt16    treeDepth;
//    UInt32    rootNode;
//    UInt32    leafRecords;
//    UInt32    firstLeafNode;
//    UInt32    lastLeafNode;
//    UInt16    nodeSize;
//    UInt16    maxKeyLength;
//    UInt32    totalNodes;
//    UInt32    freeNodes;
//    UInt16    reserved1;
//    UInt32    clumpSize;      // misaligned
//    UInt8     btreeType;
//    UInt8     keyCompareType;
//    UInt32    attributes;     // long aligned again
//    UInt32    reserved3[16];
//};
//typedef struct BTHeaderRec BTHeaderRec;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
struct BTHeaderRec {
    treeDepth: u16,
    rootNode: u32,
    leafRecords: u32,
    firstLeafNode: u32,
    lastLeafNode: u32,
    nodeSize: u16,
    maxKeyLength: u16,
    totalNodes: u32,
    freeNodes: u32,
    reserved1: u16,
    clumpSize: u32,
    btreeType: u8,
    keyCompareType: u8,
    attributes: u32,
    reserved3: [u32; 16],
}

impl BTHeaderRec {
    fn import(source: &mut Read) -> std::io::Result<Self> {
        Ok(Self {
            treeDepth: source.read_u16::<BigEndian>()?,
            rootNode: source.read_u32::<BigEndian>()?,
            leafRecords: source.read_u32::<BigEndian>()?,
            firstLeafNode: source.read_u32::<BigEndian>()?,
            lastLeafNode: source.read_u32::<BigEndian>()?,
            nodeSize: source.read_u16::<BigEndian>()?,
            maxKeyLength: source.read_u16::<BigEndian>()?,
            totalNodes: source.read_u32::<BigEndian>()?,
            freeNodes: source.read_u32::<BigEndian>()?,
            reserved1: source.read_u16::<BigEndian>()?,
            clumpSize: source.read_u32::<BigEndian>()?,
            btreeType: source.read_u8()?,
            keyCompareType: source.read_u8()?,
            attributes: source.read_u32::<BigEndian>()?,
            reserved3: [
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
            ],
        })
    }
}


#[derive(Debug)]
enum HFSError {
    InvalidData(String),
    IOError(Error),
    BadNode,
    InvalidRecordKey,
}

impl From<Error> for HFSError {
    fn from(x: Error) -> HFSError {
        HFSError::IOError(x)
    }
}

impl From<HFSError> for Error {
    fn from(x: HFSError) -> Error {
        match x {
            HFSError::IOError(y) => y,
            _ => Error::new(ErrorKind::Other, "HFS Error"),
        }
    }
}

type HFSResult<T> = Result<T, HFSError>;


trait Key : Ord + PartialOrd + Eq + PartialEq {
}

struct HeaderNode {
    descriptor: BTNodeDescriptor,
    header: BTHeaderRec,
    user_data: Vec<u8>,
    map: Vec<u8>,
}

struct MapNode {
    descriptor: BTNodeDescriptor,
}

struct IndexNode {
    descriptor: BTNodeDescriptor,
}

struct LeafNode<R: Record> {
    descriptor: BTNodeDescriptor,
    records: Vec<R>,
}

enum Node<R: Record> {
    HeaderNode(HeaderNode),
    MapNode(MapNode),
    IndexNode(IndexNode),
    LeafNode(LeafNode<R>),
}

impl<R: Record> Node<R> {
    fn load(data: &[u8]) -> Result<Node<R>, HFSError> {
        // TODO Check minimum size
        // TODO Check numRecords within size limits
        let node = BTNodeDescriptor::import(&mut &data[..])?;
        //println!("Node: {:?}", node);
        //println!("Node len: {}", data.len());
        let num_offsets = (node.numRecords+1) as usize;
        let first_offset_pos = data.len() - 2;
        let last_offset_pos = data.len() - num_offsets*2;
        let mut offsets = Vec::with_capacity(num_offsets);
        //println!("Bytes: {:?}", data[(data.len()-num_offsets*2)..]);
        //for b in data.iter() {
        //    print!("{:02x} ", b);
        //}
        for idx in 0..num_offsets {
            let offset_pos = first_offset_pos - 2*idx;
            let offset = (&data[offset_pos..offset_pos+2]).read_u16::<BigEndian>()? as usize;
            // BTNodeDescriptor is 14 bytes long
            // All offsets must be between Descriptor and
            // beginning of offset table
            if offset < 14 || offset > last_offset_pos {
                return Err(HFSError::InvalidData("Invalid record offset value".to_string()));
            }
            offsets.push(offset);
            //println!("  Offset: {}", offset);
        }
        let mut records = Vec::new();
        for idx in 0..num_offsets-1 {
            let first = offsets[idx];
            let last = offsets[idx+1];
            records.push(&data[first..last]);
        }
        if node.kind == kBTHeaderNode {
            println!("Header Length: {}", records[0].len());
            Ok(Node::HeaderNode(HeaderNode {
                descriptor: node,
                header: BTHeaderRec::import(&mut records[0])?,
                user_data: Vec::new(),
                map: Vec::new(),
            }))
        } else if node.kind == kBTMapNode {
            Ok(Node::MapNode(MapNode {
                descriptor: node,
            }))
        } else if node.kind == kBTIndexNode {
            Ok(Node::IndexNode(IndexNode {
                descriptor: node,
            }))
        } else if node.kind == kBTLeafNode {
            let mut r = Vec::<R>::new();
            for record in &records {
                let r2 = CatalogKey::import(&mut &record[..])?;
                println!("File: {:?}", r2);
                r.push(R::new(r2));
            }
            Ok(Node::LeafNode(LeafNode {
                descriptor: node,
                records: r,
            }))
        } else {
            //Err(Error::new(ErrorKind::InvalidData, "Invalid Node Type"))
            Err(HFSError::InvalidData("Invalid Node Type".to_string()))
        }
    }
}


struct BTree<R> {
    fork: Rc<RefCell<Fork>>,
    node_size: u16,
    header: HeaderNode,
    top_node: Option<R>,
}

impl<R: Record> BTree<R> {
    fn open(fork_rc: ForkRc) -> Result<BTree<R>, HFSError> {
        let node_size;
        let header;
        {
        let fork = fork_rc.borrow_mut();
        //let node_size = 0;
        let mut buffer = vec![0; 512];
        fork.read(0, &mut buffer).expect("Failed to read from fork");
        //let node = BTNodeDescriptor::import(&mut &buffer[..]).unwrap();
        //assert_eq!(node.kind, kBTHeaderNode);
        //assert_eq!(node.bLink, 0);
        //assert_eq!(node.numRecords, 3);
        //assert_eq!(node.reserved, 0);
        node_size = (&buffer[32..34]).read_u16::<BigEndian>().expect("Error decoding node size");
        let remaining = node_size - buffer.len() as u16;
        for _ in 0..remaining {
            buffer.push(0);
        }
        fork.read(512, &mut buffer[512..]).expect("Failed to read from fork");
        let header_node = Node::<CatalogRecord>::load(&buffer)?;
        header = match header_node {
            Node::HeaderNode(x) => {
                println!("{:?}", x.descriptor);
                x
            },
            _ => {
                return Err(HFSError::BadNode);
            },
        };
        }
        // XXX Verify size >= 512
        //
        //println!("{}", node_size);
        //assert_eq!(node_size, 4096);
        //file.read_exact(&mut header_node)?;  // Minimum size for any node, needed to get nodeSize field
        //println!("{} -> {:?}", header_node.len(), header_node);
        //println!("{} -> {:?}", header_node.len(), "f");
        //let node_size = (&header_node[32..34]).read_u16::<BigEndian>()?;
        //println!("{}", node_size);
        Ok(BTree {
            fork: fork_rc,
            node_size,
            header,
            top_node: None,
        })
    }

    fn get_node(&self, node_num: usize) -> HFSResult<Node<R>> {
        {
        let fork = self.fork.borrow_mut();
        let mut buffer = vec![0; self.node_size as usize];
        fork.read((node_num*self.node_size as usize) as u64, &mut buffer).expect("Failed to read from fork");
        let node = Node::<R>::load(&buffer)?;
        //header = match header_node {
        //    Node::HeaderNode(x) => {
        //        println!("{:?}", x.descriptor);
        //        x
        //    },
        //    _ => {
        //        return Err(HFSError::BadNode);
        //    },
        //};
        Ok(node)
        }
    }
}

type BTreeRc<R> = Rc<RefCell<BTree<R>>>;


//enum {
//    /* Bits 0-6 are reserved */
//    kHFSVolumeHardwareLockBit       =  7,
//    kHFSVolumeUnmountedBit          =  8,
//    kHFSVolumeSparedBlocksBit       =  9,
//    kHFSVolumeNoCacheRequiredBit    = 10,
//    kHFSBootVolumeInconsistentBit   = 11,
//    kHFSCatalogNodeIDsReusedBit     = 12,
//    kHFSVolumeJournaledBit          = 13,
//    /* Bit 14 is reserved */
//    kHFSVolumeSoftwareLockBit       = 15
//    /* Bits 16-31 are reserved */
//};

bitflags! {
    struct VolumeAttributes: u32 {
        /* Bits 0-6 are reserved */
        const kHFSVolumeHardwareLockBit       = 1 <<  7;
        const kHFSVolumeUnmountedBit          = 1 <<  8;
        const kHFSVolumeSparedBlocksBit       = 1 <<  9;
        const kHFSVolumeNoCacheRequiredBit    = 1 << 10;
        const kHFSBootVolumeInconsistentBit   = 1 << 11;
        const kHFSCatalogNodeIDsReusedBit     = 1 << 12;
        const kHFSVolumeJournaledBit          = 1 << 13;
        /* Bit 14 is reserved */
        const kHFSVolumeSoftwareLockBit       = 1 << 15;
        /* Bits 16-30 are reserved */
        const kHFSVolumeUnusedNodeFixBit      = 1 << 31;
    }
}


//struct HFSPlusVolumeHeader {
//    UInt16              signature;
//    UInt16              version;
//    UInt32              attributes;
//    UInt32              lastMountedVersion;
//    UInt32              journalInfoBlock;
//
//    UInt32              createDate;
//    UInt32              modifyDate;
//    UInt32              backupDate;
//    UInt32              checkedDate;
//
//    UInt32              fileCount;
//    UInt32              folderCount;
//
//    UInt32              blockSize;
//    UInt32              totalBlocks;
//    UInt32              freeBlocks;
//
//    UInt32              nextAllocation;
//    UInt32              rsrcClumpSize;
//    UInt32              dataClumpSize;
//    HFSCatalogNodeID    nextCatalogID;
//
//    UInt32              writeCount;
//    UInt64              encodingsBitmap;
//
//    UInt32              finderInfo[8];
//
//    HFSPlusForkData     allocationFile;
//    HFSPlusForkData     extentsFile;
//    HFSPlusForkData     catalogFile;
//    HFSPlusForkData     attributesFile;
//    HFSPlusForkData     startupFile;
//};
//typedef struct HFSPlusVolumeHeader HFSPlusVolumeHeader;

const HFSP_SIGNATURE: u16 = 0x482b;  // H+ Signature (Big endian)
const HFSX_SIGNATURE: u16 = 0x4858;  // HFSX Signature (Big endian)
const HFSP_VERSION: u16 = 4;  // H+ Signature (Big endian)
const HFSX_VERSION: u16 = 5;  // HFSX Signature (Big endian)

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
struct HFSPlusVolumeHeader {
    signature: u16,
    version: u16,
    attributes: VolumeAttributes,
    lastMountedVersion: u32,
    journalInfoBlock: u32,

    createDate: u32,
    modifyDate: u32,
    backupDate: u32,
    checkedDate: u32,

    fileCount: u32,
    folderCount: u32,

    blockSize: u32,
    totalBlocks: u32,
    freeBlocks: u32,

    nextAllocation: u32,
    rsrcClumpSize: u32,
    dataClumpSize: u32,
    nextCatalogID: HFSCatalogNodeID,

    writeCount: u32,
    encodingsBitmap: u64,

    finderInfo: [u32; 8],

    allocationFile: HFSPlusForkData,
    extentsFile: HFSPlusForkData,
    catalogFile: HFSPlusForkData,
    attributesFile: HFSPlusForkData,
    startupFile: HFSPlusForkData,
}

impl HFSPlusVolumeHeader {
    fn import(source: &mut Read) -> std::io::Result<Self> {
        Ok(Self {
            signature: source.read_u16::<BigEndian>()?,
            version: source.read_u16::<BigEndian>()?,
            attributes: VolumeAttributes::from_bits_truncate(source.read_u32::<BigEndian>()?),
            lastMountedVersion: source.read_u32::<BigEndian>()?,
            journalInfoBlock: source.read_u32::<BigEndian>()?,

            createDate: source.read_u32::<BigEndian>()?,
            modifyDate: source.read_u32::<BigEndian>()?,
            backupDate: source.read_u32::<BigEndian>()?,
            checkedDate: source.read_u32::<BigEndian>()?,

            fileCount: source.read_u32::<BigEndian>()?,
            folderCount: source.read_u32::<BigEndian>()?,

            blockSize: source.read_u32::<BigEndian>()?,
            totalBlocks: source.read_u32::<BigEndian>()?,
            freeBlocks: source.read_u32::<BigEndian>()?,

            nextAllocation: source.read_u32::<BigEndian>()?,
            rsrcClumpSize: source.read_u32::<BigEndian>()?,
            dataClumpSize: source.read_u32::<BigEndian>()?,
            nextCatalogID: source.read_u32::<BigEndian>()?,  // XXX HFSCatalogNodeID,

            writeCount: source.read_u32::<BigEndian>()?,
            encodingsBitmap: source.read_u64::<BigEndian>()?,

            finderInfo: [
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
                source.read_u32::<BigEndian>()?,
            ],

            allocationFile: HFSPlusForkData::import(source)?,
            extentsFile: HFSPlusForkData::import(source)?,
            catalogFile: HFSPlusForkData::import(source)?,
            attributesFile: HFSPlusForkData::import(source)?,
            startupFile: HFSPlusForkData::import(source)?,
        })
    }
}


struct Fork {
    file: Rc<RefCell<File>>,
    volume: Rc<RefCell<HFSVolume>>,
    logical_size: u64,
    extents: Vec<(u32, u32)>,
}

impl Fork {
    fn load(file: Rc<RefCell<File>>, volume: Rc<RefCell<HFSVolume>>, data: &HFSPlusForkData) -> std::io::Result<Fork> {
        //Err(Error::new(ErrorKind::Other, "f"))
        let mut extents = Vec::with_capacity(8);
        for extent in &data.extents {
            extents.push((extent.startBlock, extent.blockCount));
        }
        Ok(Fork { file, volume, logical_size: data.logicalSize, extents })
    }

    fn check(&self) -> std::io::Result<()> {
        let sum: u32 = self.extents.iter().map(|x| x.1).sum();
        let actual_size = sum as u64 * self.volume.borrow().header.blockSize as u64;
        if actual_size != self.logical_size {
            return Err(Error::new(ErrorKind::Other, "Size does not add up"));
        }
        Ok(())
    }

    fn read(&self, offset: u64, buffer: &mut [u8]) -> std::io::Result<()> {
        let volume = self.volume.borrow();
        let mut file = self.file.borrow_mut();
        println!("Start: {}", self.extents[0].0 as u64);
        file.seek(std::io::SeekFrom::Start(self.extents[0].0 as u64 * volume.header.blockSize as u64 + offset))?;
        println!("Fork read: {}", buffer.len());
        file.read_exact(buffer)?;
        Ok(())
    }

    fn len(&self) -> u64 {
        self.logical_size
    }
}

type ForkRc = Rc<RefCell<Fork>>;

struct HFSVolume {
    file: Rc<RefCell<File>>,
    header: HFSPlusVolumeHeader,
    catalog_fork: Weak<RefCell<Fork>>,
    extents_fork: Weak<RefCell<Fork>>,
    forks: HashMap<HFSCatalogNodeID, Rc<RefCell<Fork>>>,
    catalog_btree: Option<BTreeRc<CatalogRecord>>,
    extents_btree: Option<BTreeRc<CatalogRecord>>,
}

impl HFSVolume {
    fn load(mut file: File) -> std::io::Result<Rc<RefCell<HFSVolume>>> {
        file.seek(std::io::SeekFrom::Start(1024))?;
        let header = HFSPlusVolumeHeader::import(&mut file)?;
        let _hfsx_volume = match header.signature {
            HFSP_SIGNATURE => {
                println!("HFS+ Volume");
                false
            },
            HFSX_SIGNATURE => {
                println!("HFSX Volume");
                true
            },
            _ => {
                println!("Unknown Volume");
                return Err(Error::new(ErrorKind::InvalidData, "Invalid volume signature"));
            },
        };
        //if !hfsx_volume {
        //    if header.version != HFSP_VERSION {
        //        println!("Unsupported version for HFS+ Volume");
        //        return Ok(());
        //    }
        //} else {
        //    if header.version != HFSX_VERSION {
        //        println!("Unsupported version for HFSX Volume");
        //        return Ok(());
        //    }
        //}
        let file = Rc::new(RefCell::new(file));
        let volume = Rc::new(RefCell::new(HFSVolume {
            file,
            header,
            catalog_fork: Weak::new(),
            extents_fork: Weak::new(),
            forks: HashMap::new(),
            catalog_btree: None,
            extents_btree: None,
        }));
        let catalog_fork = Rc::new(RefCell::new(Fork::load(Rc::clone(&volume.borrow().file), Rc::clone(&volume), &volume.borrow().header.catalogFile)?));
        let extents_fork = Rc::new(RefCell::new(Fork::load(Rc::clone(&volume.borrow().file), Rc::clone(&volume), &volume.borrow().header.extentsFile)?));
        volume.borrow_mut().catalog_fork = Rc::downgrade(&catalog_fork);
        volume.borrow_mut().extents_fork = Rc::downgrade(&extents_fork);
        volume.borrow_mut().forks.insert(kHFSCatalogFileID, Rc::clone(&catalog_fork));
        volume.borrow_mut().forks.insert(kHFSExtentsFileID, Rc::clone(&extents_fork));
        volume.borrow_mut().catalog_btree = Some(Rc::new(RefCell::new(BTree::open(catalog_fork)?)));
        volume.borrow_mut().extents_btree = Some(Rc::new(RefCell::new(BTree::open(extents_fork)?)));
        Ok(volume)
    }

    fn load_file(filename: &str) -> std::io::Result<Rc<RefCell<HFSVolume>>> {
        let file = File::open(filename)?;
        HFSVolume::load(file)
    }
}


fn _main() -> std::io::Result<()> {
    let mut file = File::open("src/image")?;
    file.seek(std::io::SeekFrom::Start(1024))?;
    let header = HFSPlusVolumeHeader::import(&mut file)?;
    println!("Volume: {:?}", header);
    let hfsx_volume = match header.signature {
        HFSP_SIGNATURE => {
            println!("HFS+ Volume");
            false
        },
        HFSX_SIGNATURE => {
            println!("HFSX Volume");
            true
        },
        _ => {
            println!("Unknown Volume");
            return Ok(());
        },
    };
    if !hfsx_volume {
        if header.version != HFSP_VERSION {
            println!("Unsupported version for HFS+ Volume");
            return Ok(());
        }
    } else {
        if header.version != HFSX_VERSION {
            println!("Unsupported version for HFSX Volume");
            return Ok(());
        }
    }
    println!("Start: {}", header.catalogFile.extents[0].startBlock as u64);
    file.seek(std::io::SeekFrom::Start(header.catalogFile.extents[0].startBlock as u64))?;
    //let mut header_node = Vec::with_capacity(512);
    let mut header_node = vec![0; 512];
    //let mut header_node = [0; 512];
    println!("{} -> {:?}", header_node.len(), "f");
    file.read_exact(&mut header_node)?;  // Minimum size for any node, needed to get nodeSize field
    println!("{} -> {:?}", header_node.len(), header_node);
    println!("{} -> {:?}", header_node.len(), "f");
    let node_size = (&header_node[32..34]).read_u16::<BigEndian>()?;
    println!("{}", node_size);
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


extern crate libc;
extern crate time;

extern crate fuse;

//use std::io::{UserFile, UserDir};
//use std::path::PathBuf;
//use std::mem;
use libc::{ENOENT, ENOSYS};
use time::Timespec;
use std::env;
use std::path::Path;
//use fuse::{FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};
use fuse::{FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyDirectory};

struct JsonFilesystem;

impl Filesystem for JsonFilesystem {
    fn getattr(&mut self, _req: &Request, ino: u64, reply: ReplyAttr) {
        println!("getattr(ino={})", ino);
        let ts = Timespec::new(0, 0);
        let attr = FileAttr {
            ino: 1,
            size: 0,
            blocks: 0,
            atime: ts,
            mtime: ts,
            ctime: ts,
            crtime: ts,
            kind: FileType::Directory,
            perm: 0o755,
            nlink: 0,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
        };
        let ttl = Timespec::new(1, 0);
        if ino == 1 {
            reply.attr(&ttl, &attr);
        } else {
            reply.error(ENOSYS);
        }
    }

    fn readdir(&mut self, _req: &Request, ino: u64, fh: u64, offset: i64, mut reply: ReplyDirectory) {
        println!("readdir(ino={}, fh={}, offset={})", ino, fh, offset);
        if ino == 1 {
            if offset == 0 {
                reply.add(1, 0, FileType::Directory, &Path::new("."));
                reply.add(1, 1, FileType::Directory, &Path::new(".."));
            }
            reply.ok();
        } else {
            reply.error(ENOENT);
        }
    }
}

fn main() -> std::io::Result<()> {
    _main()?;
    let mountpoint = match env::args().nth(1) {
        Some(path) => path,
        None => {
            println!("Usage: {} <MOUNTPOINT>", env::args().nth(0).unwrap());
            return Ok(());
        }
    };

    fuse::mount(JsonFilesystem, &mountpoint, &[])?;

    Ok(())
}

mod hfs_strings;

#[cfg(test)]
mod test;
