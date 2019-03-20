use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Cursor, SeekFrom};
use std::collections::HashMap;
use std::cmp::Ordering;
use std::marker::PhantomData;

use std::fmt;

use std::path::PathBuf;


extern crate backtrace;
use backtrace::Backtrace;

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

#[macro_use]
extern crate bitflags;

extern crate unicode_normalization;
use unicode_normalization::UnicodeNormalization;

extern crate positioned_io;
use positioned_io::ReadAt;

extern crate rand;


mod hfs_strings;
use hfs_strings::fast_unicode_compare;

mod internal;
pub use internal::*;

#[cfg(test)]
mod test;



#[derive(Debug)]
pub enum Error {
    InvalidData(String),
    IOError(io::Error),
    BadNode,
    InvalidRecordKey,
    InvalidRecordType,
    UnsupportedOperation,
    KeyNotFound,
}

impl From<io::Error> for Error {
    fn from(x: io::Error) -> Error {
        Error::IOError(x)
    }
}

impl From<Error> for io::Error {
    fn from(x: Error) -> io::Error {
        match x {
            Error::IOError(y) => y,
            _ => io::Error::new(io::ErrorKind::Other, format!("{:?}", x)),
        }
    }
}

type Result<T> = std::result::Result<T, Error>;


/// HFS Plus strings are UTF-16 Big-Endian and stored using Unicode
/// Normalization Form D per the Unicode 2.0 specification. Strings
/// are kept in 16-bit form in order to support the specific
/// case-insensitive string comparison used in HFS Plus as documented
/// in Apple's Technical Note 1150.
#[derive(Clone)]
pub struct HFSString(Vec<u16>);

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



pub trait Key : fmt::Debug + Ord + PartialOrd + Eq + PartialEq {
    fn import(source: &mut Read) -> Result<Self>
        where Self: Sized;
    fn export(&self, source: &mut Write) -> Result<()>;
}

pub trait Record<K> {
    fn import(source: &mut Read, key: K) -> Result<Self>
        where Self: Sized;
    fn export(&self, source: &mut Write) -> Result<()>;
    fn get_key(&self) -> &K;
}

pub struct IndexRecord<K> {
    key: K,
    node_id: u32,
}

impl<K> Record<K> for IndexRecord<K> {
    fn import(source: &mut Read, key: K) -> Result<Self> {
        let node_id = source.read_u32::<BigEndian>()?;
        Ok(IndexRecord { key, node_id })
    }

    fn export(&self, _source: &mut Write) -> Result<()> {
        Err(Error::UnsupportedOperation)
    }

    fn get_key(&self) -> &K {
        &self.key
    }
}

pub struct HeaderNode {
    pub descriptor: BTNodeDescriptor,
    pub header: BTHeaderRec,
    user_data: Vec<u8>,
    map: Vec<u8>,
}

pub struct MapNode {
    _descriptor: BTNodeDescriptor,
}

pub struct IndexNode<K> {
    descriptor: BTNodeDescriptor,
    records: Vec<IndexRecord<K>>,
}

pub struct LeafNode<R> {
    pub descriptor: BTNodeDescriptor,
    records: Vec<Rc<R>>,
}

pub enum Node<K, R> {
    HeaderNode(HeaderNode),
    MapNode(MapNode),
    IndexNode(IndexNode<K>),
    LeafNode(LeafNode<R>),
}

impl<K: Key, R: Record<K>> Node<K, R> {
    fn load(data: &[u8]) -> Result<Node<K, R>> {
        // TODO Check minimum size
        // TODO Check numRecords within size limits
        let node = BTNodeDescriptor::import(&mut &data[..])?;
        let num_offsets = (node.numRecords+1) as usize;
        let first_offset_pos = data.len() - 2;
        let last_offset_pos = data.len() - num_offsets*2;
        let mut offsets = Vec::with_capacity(num_offsets);
        for idx in 0..num_offsets {
            let offset_pos = first_offset_pos - 2*idx;
            let offset = (&data[offset_pos..offset_pos+2]).read_u16::<BigEndian>()? as usize;
            // BTNodeDescriptor is 14 bytes long
            // All offsets must be between Descriptor and
            // beginning of offset table
            if offset < 14 || offset > last_offset_pos {
                return Err(Error::InvalidData("Invalid record offset value".to_string()));
            }
            offsets.push(offset);
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
                user_data: records[1].to_vec(),
                map: records[2].to_vec(),
            }))
        } else if node.kind == kBTMapNode {
            Ok(Node::MapNode(MapNode {
                _descriptor: node,
            }))
        } else if node.kind == kBTIndexNode {
            let mut r = Vec::<IndexRecord<K>>::new();
            for record in &records {
                let mut v = Cursor::new(record);
                let r2 = K::import(&mut v)?;
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
                let r2 = K::import(&mut v)?;
                println!("File: {:?}", r2);
                r.push(Rc::new(R::import(&mut v, r2)?));
            }
            Ok(Node::LeafNode(LeafNode {
                descriptor: node,
                records: r,
            }))
        } else {
            //Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Node Type"))
            Err(Error::InvalidData("Invalid Node Type".to_string()))
        }
    }

    fn save(&self, data: &mut [u8]) -> Result<()> {
        let data_len = data.len() as u64;
        let mut cursor = Cursor::new(data);
        let mut positions = Vec::new();
        match self {
            Node::HeaderNode(x) => {
                x.descriptor.export(&mut cursor)?;
                positions.push(cursor.position() as u16);
                x.header.export(&mut cursor)?;
                positions.push(cursor.position() as u16);
                cursor.write(&x.user_data)?;
                positions.push(cursor.position() as u16);
                cursor.write(&x.map)?;
            },
            Node::LeafNode(x) => {
                x.descriptor.export(&mut cursor)?;
                for record in &x.records {
                    positions.push(cursor.position() as u16);
                    record.get_key().export(&mut cursor)?;
                    record.export(&mut cursor)?;
                }
            },
            _ => { return Err(Error::UnsupportedOperation); },
        };
        positions.push(cursor.position() as u16);
        cursor.set_position(data_len - 2*positions.len() as u64);
        for position in positions.iter().rev() {
            cursor.write_u16::<BigEndian>(*position)?;
        }
        Ok(())
    }
}


pub struct BTree<F: ReadAt, K, R> {
    fork: F,
    node_size: u16,
    pub header: HeaderNode,
    _key: PhantomData<K>,
    _top_node: Option<R>,
}

impl<F: ReadAt, K: Key, R: Record<K>> BTree<F, K, R> {
    fn open(mut fork: F) -> Result<BTree<F, K, R>> {
        let header;
        // XXX Verify size >= 512
        // Minimum size for any node, needed to get nodeSize field
        let mut buffer = vec![0; 512];
        fork.read_exact_at(0, &mut buffer).expect("Failed to read from fork");
        let node_size = (&buffer[32..34]).read_u16::<BigEndian>().expect("Error decoding node size");
        let remaining = node_size - buffer.len() as u16;
        for _ in 0..remaining {
            buffer.push(0);
        }
        fork.read_exact_at(512, &mut buffer[512..]).expect("Failed to read from fork");
        let header_node = Node::<K, R>::load(&buffer)?;
        header = match header_node {
            Node::HeaderNode(x) => {
                println!("{:?}", x.descriptor);
                x
            },
            _ => {
                return Err(Error::BadNode);
            },
        };
        Ok(BTree {
            fork,
            node_size,
            header,
            _key: PhantomData,
            _top_node: None,
        })
    }

    pub fn get_node(&mut self, node_num: usize) -> Result<Node<K, R>> {
        let mut buffer = vec![0; self.node_size as usize];
        let offset = (node_num*self.node_size as usize) as u64;
        self.fork.read_exact_at(offset, &mut buffer).expect("Failed to read from fork");
        let node = Node::<K, R>::load(&buffer)?;
        Ok(node)
    }

    fn get_record_range_node(&mut self, first: &K, last: &K, node_id: usize) -> Result<Vec<Rc<R>>> {
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
            Node::LeafNode(mut x) => {
                println!("{:?}", x.descriptor);
                let mut return_records = Vec::new();
                loop {
                    for record in &x.records {
                        if record.get_key() >= last {
                            break;
                        } else if record.get_key() >= first {
                            println!("{:?}", record.get_key());
                            return_records.push(Rc::clone(record));
                        }
                    }
                    if x.records[x.records.len()-1].get_key() >= last ||
                       x.descriptor.fLink == 0 {
                        break;
                    }
                    let next_node = self.get_node(x.descriptor.fLink as usize)?;
                    x = match next_node {
                        Node::LeafNode(x) => x,
                        _ => { return Err(Error::InvalidRecordType); }
                    };
                }
                Ok(return_records)
            },
            _ => {
                Err(Error::InvalidRecordType)
            }
        }
    }

    fn get_record_node(&mut self, key: &K, node_id: usize) -> Result<Rc<R>> {
        let node = self.get_node(node_id)?;
        match node {
            Node::IndexNode(x) => {
                println!("{:?}", x.descriptor);
                let mut return_record = &x.records[0];
                if key < return_record.get_key() {
                    return Err(Error::InvalidRecordKey);
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
                        return Err(Error::KeyNotFound);
                    } else if key == record.get_key() {
                        return Ok(Rc::clone(record));
                    }
                }
                Err(Error::KeyNotFound)
            },
            _ => {
                Err(Error::InvalidRecordType)
            }
        }
    }

    fn get_record(&mut self, key: &K) -> Result<Rc<R>> {
        self.get_record_node(&key, self.header.header.rootNode as usize)
    }

    fn get_record_range(&mut self, first: &K, last: &K) -> Result<Vec<Rc<R>>> {
        self.get_record_range_node(first, last, self.header.header.rootNode as usize)
    }
}

type BTreeRc<F, K, R> = Rc<RefCell<BTree<F, K, R>>>;



pub struct Fork<F: ReadAt> {
    file: Rc<RefCell<F>>,
    position: u64,
    catalog_id: HFSCatalogNodeID,
    fork_type: u8,
    volume: Rc<RefCell<HFSVolume<F>>>,
    logical_size: u64,
    extents: Vec<(u32, u32, u64, u64)>,
}

impl<F: ReadAt> Fork<F> {
    pub fn load(file: Rc<RefCell<F>>, catalog_id: HFSCatalogNodeID, fork_type: u8, volume: Rc<RefCell<HFSVolume<F>>>, data: &HFSPlusForkData) -> io::Result<Fork<F>> {
        let block_size = volume.borrow().header.blockSize as u64;
        let mut extents = Vec::with_capacity(8);
        let mut extent_position = 0;
        let mut extent_block = 0;
        println!("Block Size: {}", block_size);
        let mut extents_result = Some(data.extents);
        while let Some(extent_list) = extents_result {
            for extent in &extent_list {
                let extent_size = extent.blockCount as u64 * block_size;
                let extent_end = extent_position + extent_size;
                extent_position = std::cmp::min(data.logicalSize, extent_position);
                let extent_end = std::cmp::min(data.logicalSize, extent_end);
                println!("{} = {} = {} = {}", extent.startBlock, extent.blockCount, extent_position, extent_end);
                extents.push((extent.startBlock, extent.blockCount, extent_position, extent_end));
                extent_position += extent_size;
                extent_block += extent.blockCount;
            }
            extents_result = None;
            if extent_position < data.logicalSize {
                if let Some(et) = &volume.borrow().extents_btree {
                    let search_key = ExtentKey::new(catalog_id, fork_type, extent_block);
                    let extent_record = et.borrow_mut().get_record(&search_key)?;
                    extents_result = Some(extent_record.body);
                }
            }
        }
        Ok(Fork { file, position: 0, catalog_id, fork_type, volume, logical_size: data.logicalSize, extents })
    }

    fn check(&self) -> io::Result<()> {
        let sum: u32 = self.extents.iter().map(|x| x.1).sum();
        let actual_size = sum as u64 * self.volume.borrow().header.blockSize as u64;
        if actual_size != self.logical_size {
            return Err(io::Error::new(io::ErrorKind::Other, "Size does not add up"));
        }
        Ok(())
    }

    pub fn read_all(&mut self) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0; self.logical_size as usize];
        self.read_exact_at(0, &mut buffer)?;
        Ok(buffer)
    }

    fn len(&self) -> u64 {
        self.logical_size
    }
}

impl<F: ReadAt> ReadAt for Fork<F> {
    fn read_at(&self, offset: u64, buffer: &mut [u8]) -> io::Result<usize> {
        // TODO Check limits and verify offset is valid
        // Needs test to show it will fail
        let volume = self.volume.borrow();
        let mut file = self.file.borrow_mut();
        let block_size = volume.header.blockSize as u64;
        let mut bytes_read = 0;
        for extent in &self.extents {
            println!("{:?}", extent);
            let (start_block, _, extent_begin, extent_end) = *extent;
            println!("{} - {} - {}", start_block, extent_begin, extent_end);
            let extent_length = extent_end - extent_begin;
            if offset >= extent_end {
                continue;
            }
            //if (offset + bytes_read as u64) < extent_begin {
            //    break;
            //}
            let extent_offset = if offset > extent_begin {
                offset - extent_begin  // Partial first extent
            } else {
                0
            };
            let read_offset = start_block as u64 * block_size + extent_offset;
            let bytes_remaining = buffer.len() - bytes_read;
            let bytes_to_read = std::cmp::min(extent_length, bytes_remaining as u64);
            file.read_exact_at(read_offset, &mut buffer[bytes_read as usize..bytes_read+bytes_to_read as usize])?;
            bytes_read += bytes_to_read as usize;
            if bytes_read >= buffer.len() {
                println!("All bytes read");
                break;
            }
        }
        println!("Start: {}", self.extents[0].0 as u64);
        println!("Fork read: {}", buffer.len());
        println!("Bytes read: {}", bytes_read);
        // TODO This should return a byte count and happily accept
        // a short read, but for now, treat as read_exact()
        if bytes_read < buffer.len() {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "No more extents to read"))
        } else {
            Ok(bytes_read)
        }
    }
}

//type ForkRc<F> = Rc<RefCell<Fork<F>>>;



#[derive(Debug, Clone)]
pub struct CatalogKey {
    _case_match: bool,
    parent_id: HFSCatalogNodeID,
    pub node_name: HFSString,
}

impl Key for CatalogKey {
    fn import(source: &mut Read) -> Result<Self> {
        let key_length = source.read_u16::<BigEndian>()?;
        if key_length < 6 {
            return Err(Error::InvalidRecordKey);
        }
        let parent_id = source.read_u32::<BigEndian>()?;
        let count = source.read_u16::<BigEndian>()?;
        if key_length != count*2 + 6 {
            return Err(Error::InvalidRecordKey);
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

    fn export(&self, _source: &mut Write) -> Result<()> {
        Err(Error::UnsupportedOperation)
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

pub enum CatalogBody {
    Folder(HFSPlusCatalogFolder),
    File(HFSPlusCatalogFile),
    FolderThread(CatalogKey),
    FileThread(CatalogKey),
}

pub struct CatalogRecord {
    pub key: CatalogKey,
    pub body: CatalogBody,
}

impl Record<CatalogKey> for CatalogRecord {
    fn import(source: &mut Read, key: CatalogKey) -> Result<Self> {
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
                //    return Err(Error::InvalidRecordKey);
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
                //    return Err(Error::InvalidRecordKey);
                //}
                let mut node_name = Vec::with_capacity(count as usize);
                for _ in 0..count as usize {
                    node_name.push(source.read_u16::<BigEndian>()?);
                }
                let to_key = CatalogKey { _case_match: false, parent_id, node_name: HFSString(node_name) };
                CatalogBody::FileThread(to_key)
            },
            _ => {
                return Err(Error::InvalidRecordType);
                //return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Invalid Record Type: {:?}", Backtrace::new())));
            },
        };
        Ok(CatalogRecord { key, body })
    }

    fn export(&self, _source: &mut Write) -> Result<()> {
        Err(Error::UnsupportedOperation)
    }

    fn get_key(&self) -> &CatalogKey {
        &self.key
    }
}



#[derive(Debug)]
pub struct ExtentKey(HFSPlusExtentKey);

impl ExtentKey {
    fn new(file_id: HFSCatalogNodeID, fork_type: u8, start_block: u32) -> Self {
        ExtentKey(HFSPlusExtentKey {
            keyLength:          10,
            forkType:           fork_type,
            pad:                0,
            fileID:             file_id,
            startBlock:         start_block,
        })
    }
}

impl Key for ExtentKey {
    fn import(source: &mut Read) -> Result<Self> {
        Ok(ExtentKey(HFSPlusExtentKey::import(source)?))
    }

    fn export(&self, source: &mut Write) -> Result<()> {
        self.0.export(source)?;
        Ok(())
    }
}

impl PartialOrd for ExtentKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExtentKey {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.fileID.cmp(&other.0.fileID) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => match self.0.forkType.cmp(&other.0.forkType) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => self.0.startBlock.cmp(&other.0.startBlock),
            },
        }
    }
}

impl PartialEq for ExtentKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for ExtentKey {
}


pub struct ExtentRecord {
    pub key: ExtentKey,
    pub body: HFSPlusExtentRecord,
}

impl Record<ExtentKey> for ExtentRecord {
    fn import(source: &mut Read, key: ExtentKey) -> Result<Self> {
        let body = import_record(source)?;
        Ok(ExtentRecord { key, body })
    }

    fn export(&self, source: &mut Write) -> Result<()> {
        export_record(&self.body, source)?;
        Ok(())
    }

    fn get_key(&self) -> &ExtentKey {
        &self.key
    }
}



pub struct HFSVolume<F: ReadAt> {
    pub file: Rc<RefCell<F>>,
    header: HFSPlusVolumeHeader,
    forks: HashMap<HFSCatalogNodeID, Rc<RefCell<Fork<F>>>>,
    pub catalog_btree: Option<BTreeRc<Fork<F>, CatalogKey, CatalogRecord>>,
    pub extents_btree: Option<BTreeRc<Fork<F>, ExtentKey, ExtentRecord>>,
}

impl<F: ReadAt> HFSVolume<F> {
    pub fn load(mut file: F) -> io::Result<Rc<RefCell<HFSVolume<F>>>> {
        //let mut contents = [0u8; 512];
        //file.read_exact_at(1024, &mut contents);
        //let header = HFSPlusVolumeHeader::import(&mut Cursor::new(&contents[..]))?;
        let header = HFSPlusVolumeHeader::import(&mut positioned_io::Cursor::new_pos(&file, 1024))?;
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
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid volume signature"));
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
            forks: HashMap::new(),
            catalog_btree: None,
            extents_btree: None,
        }));
        let catalog_fork = Fork::load(Rc::clone(&volume.borrow().file), kHFSCatalogFileID, 0, Rc::clone(&volume), &volume.borrow().header.catalogFile)?;
        let extents_fork = Fork::load(Rc::clone(&volume.borrow().file), kHFSExtentsFileID, 0, Rc::clone(&volume), &volume.borrow().header.extentsFile)?;
        volume.borrow_mut().catalog_btree = Some(Rc::new(RefCell::new(BTree::open(catalog_fork)?)));
        volume.borrow_mut().extents_btree = Some(Rc::new(RefCell::new(BTree::open(extents_fork)?)));
        Ok(volume)
    }

    pub fn get_children_id(&self, node_id: HFSCatalogNodeID) -> Result<Vec<Rc<CatalogRecord>>> {
        let mut btree = self.catalog_btree.as_ref().expect("Can't unwrap").borrow_mut();
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

    fn get_children(&self, key: &CatalogKey) -> Result<Vec<Rc<CatalogRecord>>> {
        let record = {
            let mut btree = self.catalog_btree.as_ref().unwrap().borrow_mut();
            btree.get_record(key)?
        };
        match &record.body {
            CatalogBody::Folder(x) => self.get_children_id(x.folderID),
            _ => Err(Error::InvalidRecordKey)
        }
    }

    fn get_catalog(&self) -> std::cell::RefMut<BTree<Fork<F>, CatalogKey, CatalogRecord>> {
        self.catalog_btree.as_ref().unwrap().borrow_mut()
    }

    pub fn get_path_record(&self, filename: &str) -> Result<CatalogRecord> {
        let path = PathBuf::from(filename);
        let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
        let result = self.get_catalog().get_record(&root_thread_key)?;
        let thread = match result.body {
            CatalogBody::FolderThread(ref x) => {
                x
            },
            _ => {
                return Err(Error::InvalidRecordType);
            },
        };
        let result = self.get_catalog().get_record(thread)?;
        let mut parent = match result.body {
            CatalogBody::Folder(x) => {
                x
            },
            _ => {
                return Err(Error::InvalidRecordType);
            },
        };
        for i in &path {
            let val = i.to_str().unwrap();
            println!("{}", val);
            if val == "/" {
                continue;
            }
            let parent_id = parent.folderID;
            let child_key = CatalogKey { _case_match: false, parent_id, node_name: HFSString::from(val) };
            let result = self.get_catalog().get_record(&child_key)?;
            parent = match result.body {
                CatalogBody::Folder(x) => {
                    x
                },
                CatalogBody::File(x) => {
                    return Ok(CatalogRecord { key: result.key.clone(), body: CatalogBody::File(x) });
                },
                _ => {
                    return Err(Error::InvalidRecordType);
                },
            };
        }
        Ok(CatalogRecord { key: result.key.clone(), body: CatalogBody::Folder(parent) })
    }

    pub fn get_path(&self, filename: &str) -> Result<Vec<Rc<CatalogRecord>>> {
        let path = PathBuf::from(filename);
        let root_thread_key = CatalogKey { _case_match: false, parent_id: 2, node_name: HFSString::from("") };
        let result = self.get_catalog().get_record(&root_thread_key)?;
        let thread = match result.body {
            CatalogBody::FolderThread(ref x) => {
                x
            },
            _ => {
                return Err(Error::InvalidRecordType);
            },
        };
        let result = self.get_catalog().get_record(thread)?;
        let parent = match result.body {
            CatalogBody::Folder(ref x) => {
                x
            },
            _ => {
                return Err(Error::InvalidRecordType);
            },
        };
        let mut parent_id = parent.folderID;
        for i in &path {
            let val = i.to_str().unwrap();
            println!("{}", val);
            if val == "/" {
                continue;
            }
            let child_key = CatalogKey { _case_match: false, parent_id, node_name: HFSString::from(val) };
            let result = self.get_catalog().get_record(&child_key)?;
            let parent = match result.body {
                CatalogBody::Folder(ref x) => {
                    x
                },
                _ => {
                    return Err(Error::InvalidRecordType);
                },
            };
            parent_id = parent.folderID;
        }
        self.get_children_id(parent_id)
    }
}

impl HFSVolume<File> {
    pub fn load_file(filename: &str) -> io::Result<Rc<RefCell<HFSVolume<File>>>> {
        let file = File::open(filename)?;
        HFSVolume::load(file)
    }
}
