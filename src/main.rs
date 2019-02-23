use std::fs::File;
//use std::io::{Read, Write, Seek};
use std::io::{Read, Seek};

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};

#[macro_use]
extern crate bitflags;


//typedef UInt32 HFSCatalogNodeID;

type HFSCatalogNodeID = u32;


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
use fuse::{FileAttr, FileType, Filesystem, Request, ReplyAttr, ReplyData, ReplyEntry, ReplyDirectory};

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

#[cfg(test)]
mod test;
