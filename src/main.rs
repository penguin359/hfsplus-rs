use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::fs::File;
//use std::io::{Read, Write, Seek};
use std::io::{Cursor, Error, ErrorKind, Read, Seek};
use std::collections::HashMap;
use std::cmp::Ordering;

use std::fmt;


extern crate backtrace;
use backtrace::Backtrace;

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};

#[macro_use]
extern crate bitflags;

extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;


use hfs_strings::fast_unicode_compare;



struct HFSString(Vec<u16>);

impl fmt::Debug for HFSString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from_utf16(self.0.as_slice()).unwrap())
    }
}

impl fmt::Display for HFSString {
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
    fn import(source: &mut Read, key: CatalogKey) -> std::io::Result<Self>
        where Self: Sized;
    fn get_key(&self) -> &CatalogKey;
}

struct IndexRecord {
    key: CatalogKey,
    node_id: u32,
}

impl Record for IndexRecord {
    fn import(source: &mut Read, key: CatalogKey) -> std::io::Result<Self> {
        let node_id = source.read_u32::<BigEndian>()?;
        Ok(IndexRecord { key, node_id })
    }

    fn get_key(&self) -> &CatalogKey {
        &self.key
    }
}

enum CatalogBody {
    Folder(HFSPlusCatalogFolder),
    File(HFSPlusCatalogFile),
    FolderThread(CatalogKey),
    FileThread(CatalogKey),
}

struct CatalogRecord {
    key: CatalogKey,
    body: CatalogBody,
}

impl Record for CatalogRecord {
    fn import(source: &mut Read, key: CatalogKey) -> std::io::Result<Self> {
        let record_type = source.read_i16::<BigEndian>()?;
        let body = match record_type {
            internal::kHFSPlusFolderRecord => {
                CatalogBody::Folder(HFSPlusCatalogFolder::import(source)?)
            },
            internal::kHFSPlusFileRecord => {
                CatalogBody::File(HFSPlusCatalogFile::import(source)?)
            },
            internal::kHFSPlusFolderThreadRecord => {
                let _reserved = source.read_i16::<BigEndian>()?;
                let parent_id = source.read_u32::<BigEndian>()?;
                let count = source.read_u16::<BigEndian>()?;
                //if key_length != count*2 + 6 {
                //    return Err(HFSError::InvalidRecordKey);
                //}
                let mut node_name = Vec::with_capacity(count as usize);
                for _ in 0..count as usize {
                    node_name.push(source.read_u16::<BigEndian>()?);
                }
                let to_key = CatalogKey { _case_match: false, parent_id, node_name: HFSString(node_name) };
                CatalogBody::FolderThread(to_key)
            },
            internal::kHFSPlusFileThreadRecord => {
                let _reserved = source.read_i16::<BigEndian>()?;
                let parent_id = source.read_u32::<BigEndian>()?;
                let count = source.read_u16::<BigEndian>()?;
                //if key_length != count*2 + 6 {
                //    return Err(HFSError::InvalidRecordKey);
                //}
                let mut node_name = Vec::with_capacity(count as usize);
                for _ in 0..count as usize {
                    node_name.push(source.read_u16::<BigEndian>()?);
                }
                let to_key = CatalogKey { _case_match: false, parent_id, node_name: HFSString(node_name) };
                CatalogBody::FileThread(to_key)
            },
            _ => {
                //return Err(HFSError::InvalidRecordType);
                return Err(Error::new(ErrorKind::InvalidData, format!("Invalid Record Type: {:?}", Backtrace::new())));
            },
        };
        Ok(CatalogRecord { key, body })
    }

    fn get_key(&self) -> &CatalogKey {
        &self.key
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
    InvalidRecordType,
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
    _user_data: Vec<u8>,
    _map: Vec<u8>,
}

struct MapNode {
    _descriptor: BTNodeDescriptor,
}

struct IndexNode {
    descriptor: BTNodeDescriptor,
    records: Vec<IndexRecord>,
}

struct LeafNode<R: Record> {
    descriptor: BTNodeDescriptor,
    records: Vec<Rc<R>>,
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
                _user_data: Vec::new(),
                _map: Vec::new(),
            }))
        } else if node.kind == kBTMapNode {
            Ok(Node::MapNode(MapNode {
                _descriptor: node,
            }))
        } else if node.kind == kBTIndexNode {
            let mut r = Vec::<IndexRecord>::new();
            for record in &records {
                let mut v = Cursor::new(record);
                let r2 = CatalogKey::import(&mut v)?;
                println!("File: {:?}", r2);
                r.push(IndexRecord::import(&mut v, r2)?);
            }
            Ok(Node::IndexNode(IndexNode {
                descriptor: node,
                records: r,
            }))
        } else if node.kind == kBTLeafNode {
            let mut r = Vec::<Rc<R>>::new();
            for record in &records {
                let mut v = Cursor::new(record);
                let r2 = CatalogKey::import(&mut v)?;
                println!("File: {:?}", r2);
                r.push(Rc::new(R::import(&mut v, r2)?));
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
    _top_node: Option<R>,
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
            _top_node: None,
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

    fn get_record_range_node(&self, first: &CatalogKey, last: &CatalogKey, node_id: usize) -> HFSResult<Vec<Rc<R>>> {
        let node = self.get_node(node_id)?;
        match node {
            Node::IndexNode(x) => {
                println!("{:?}", x.descriptor);
                let mut return_record = &x.records[0];
                if return_record.get_key() >= last {
                    return Ok(Vec::new());
                }
                for record in x.records.iter().skip(1) {
                    if first < record.get_key() {
                        break;
                    }
                    return_record = record;
                }
                self.get_record_range_node(first, last, return_record.node_id as usize)
            },
            Node::LeafNode(x) => {
                println!("{:?}", x.descriptor);
                let mut return_records = Vec::new();
                println!("Foobar");
                println!("First: {:?}", first);
                println!("Last: {:?}", last);
                for record in &x.records {
                    if record.get_key() >= last {
                        break;
                    } else if record.get_key() >= first {
                        println!("{:?}", record.get_key());
                        return_records.push(Rc::clone(record));
                    }
                }
                Ok(return_records)
            },
            _ => {
                Err(HFSError::InvalidRecordType)
            }
        }
    }

    fn get_record_node(&self, key: &CatalogKey, node_id: usize) -> HFSResult<Rc<R>> {
        let node = self.get_node(node_id)?;
        match node {
            Node::IndexNode(x) => {
                println!("{:?}", x.descriptor);
                let mut return_record = &x.records[0];
                if key < return_record.get_key() {
                    return Err(HFSError::InvalidRecordKey);
                }
                for record in x.records.iter().skip(1) {
                    if key < record.get_key() {
                        break;
                    }
                    return_record = record;
                }
                self.get_record_node(&key, return_record.node_id as usize)
            },
            Node::LeafNode(x) => {
                println!("{:?}", x.descriptor);
                for record in &x.records {
                    if key < record.get_key() {
                        return Err(HFSError::InvalidRecordKey);
                    } else if key == record.get_key() {
                        return Ok(Rc::clone(record));
                    }
                }
                Err(HFSError::InvalidRecordKey)
            },
            _ => {
                Err(HFSError::InvalidRecordType)
            }
        }
    }

    fn get_record(&self, key: &CatalogKey) -> HFSResult<Rc<R>> {
        self.get_record_node(&key, self.header.header.rootNode as usize)
    }

    fn get_record_range(&self, first: &CatalogKey, last: &CatalogKey) -> HFSResult<Vec<Rc<R>>> {
        self.get_record_range_node(first, last, self.header.header.rootNode as usize)
    }
}

type BTreeRc<R> = Rc<RefCell<BTree<R>>>;


mod internal;
use internal::*;



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

    fn get_children_id(&self, node_id: HFSCatalogNodeID) -> HFSResult<Vec<Rc<CatalogRecord>>> {
        let btree = self.catalog_btree.as_ref().unwrap().borrow();
        let first = CatalogKey { _case_match: false, parent_id: node_id, node_name: HFSString::from("") };
        let last = CatalogKey { _case_match: false, parent_id: node_id+1, node_name: HFSString::from("") };
        //match btree.get_record_range(&first, &last) {
        //    Ok(x) => Ok(x.iter().filter(|item| match item {
        //    _ => true
        //}).collect()),
        //    Err(x) => Err(x)
        //}
        let r = btree.get_record_range(&first, &last);
        match r {
            Ok(x) => {
                Ok(x.into_iter().filter(|item| {println!("{:?}", item.get_key()); match item.body {
                    CatalogBody::Folder(_) => true,
                    CatalogBody::File(_) => true,
                    _ => false,
                }}).collect())
            },
            Err(x) => Err(x),
        }
    }

    fn get_children(&self, key: &CatalogKey) -> HFSResult<Vec<Rc<CatalogRecord>>> {
        let btree = self.catalog_btree.as_ref().unwrap().borrow();
        match &btree.get_record(key)?.body {
            CatalogBody::Folder(x) => self.get_children_id(x.folderID),
            _ => Err(HFSError::InvalidRecordKey)
        }
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
