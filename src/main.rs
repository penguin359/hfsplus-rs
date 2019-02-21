use std::fs::File;
//use std::io::{Read, Write, Seek};
use std::io::{Read, Seek};

extern crate byteorder;
use byteorder::{BigEndian, ReadBytesExt};


//typedef UInt32 HFSCatalogNodeID;

type HFSCatalogNodeID = u32;


//struct HFSPlusExtentDescriptor {
//    UInt32                  startBlock;
//    UInt32                  blockCount;
//};
//typedef struct HFSPlusExtentDescriptor HFSPlusExtentDescriptor;

#[allow(non_snake_case)]
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
struct HFSPlusVolumeHeader {
    signature: u16,
    version: u16,
    attributes: u32,
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
            attributes: source.read_u32::<BigEndian>()?,
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
            nextCatalogID: 0,//HFSCatalogNodeID,

            writeCount: source.read_u32::<BigEndian>()?,
            encodingsBitmap: source.read_u64::<BigEndian>()?,

            finderInfo: [0; 8],//[u32; 8],

            allocationFile: HFSPlusForkData::import(source)?,
            extentsFile: HFSPlusForkData::import(source)?,
            catalogFile: HFSPlusForkData::import(source)?,
            attributesFile: HFSPlusForkData::import(source)?,
            startupFile: HFSPlusForkData::import(source)?,
        })
    }
}


fn main() -> std::io::Result<()> {
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

    Ok(())
}
