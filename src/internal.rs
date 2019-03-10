#![allow(non_snake_case, unused)]
#![allow(non_upper_case_globals, unused_variables)]
#![allow(unused)]  // TODO This needs to be removed once more code is ready

use std::io::{self, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};



//- Core Concepts

// HFS Plus Names
//struct HFSUniStr255 {
//    UInt16  length;
//    UniChar unicode[255];
//};
//typedef struct HFSUniStr255 HFSUniStr255;
//typedef const  HFSUniStr255 *ConstHFSUniStr255Param;


// HFS Plus Permissions
//struct HFSPlusBSDInfo {
//    UInt32  ownerID;
//    UInt32  groupID;
//    UInt8   adminFlags;
//    UInt8   ownerFlags;
//    UInt16  fileMode;
//    union {
//        UInt32  iNodeNum;
//        UInt32  linkCount;
//        UInt32  rawDevice;
//    } special;
//};
//typedef struct HFSPlusBSDInfo HFSPlusBSDInfo;

//#define S_ISUID 0004000     /* set user id on execution */
//#define S_ISGID 0002000     /* set group id on execution */
//#define S_ISTXT 0001000     /* sticky bit */
//
//#define S_IRWXU 0000700     /* RWX mask for owner */
//#define S_IRUSR 0000400     /* R for owner */
//#define S_IWUSR 0000200     /* W for owner */
//#define S_IXUSR 0000100     /* X for owner */
//
//#define S_IRWXG 0000070     /* RWX mask for group */
//#define S_IRGRP 0000040     /* R for group */
//#define S_IWGRP 0000020     /* W for group */
//#define S_IXGRP 0000010     /* X for group */
//
//#define S_IRWXO 0000007     /* RWX mask for other */
//#define S_IROTH 0000004     /* R for other */
//#define S_IWOTH 0000002     /* W for other */
//#define S_IXOTH 0000001     /* X for other */
//
//#define S_IFMT   0170000    /* type of file mask */
//#define S_IFIFO  0010000    /* named pipe (fifo) */
//#define S_IFCHR  0020000    /* character special */
//#define S_IFDIR  0040000    /* directory */
//#define S_IFBLK  0060000    /* block special */
//#define S_IFREG  0100000    /* regular */
//#define S_IFLNK  0120000    /* symbolic link */
//#define S_IFSOCK 0140000    /* socket */
//#define S_IFWHT  0160000    /* whiteout */

#[derive(Debug, Copy, Clone)]
pub struct HFSPlusBSDInfo {
    pub ownerID:                u32,
    pub groupID:                u32,
    pub adminFlags:             u8,
    pub ownerFlags:             u8,
    pub fileMode:               u16,
    pub special:                u32,
}

impl HFSPlusBSDInfo {
    fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            ownerID:            source.read_u32::<BigEndian>()?,
            groupID:            source.read_u32::<BigEndian>()?,
            adminFlags:         source.read_u8()?,
            ownerFlags:         source.read_u8()?,
            fileMode:           source.read_u16::<BigEndian>()?,
            special:            source.read_u32::<BigEndian>()?,
        })
    }
}

pub const S_ISUID:  u16 = 0o0004000;   /* set user id on execution */
pub const S_ISGID:  u16 = 0o0002000;   /* set group id on execution */
pub const S_ISTXT:  u16 = 0o0001000;   /* sticky bit */

pub const S_IRWXU:  u16 = 0o0000700;   /* RWX mask for owner */
pub const S_IRUSR:  u16 = 0o0000400;   /* R for owner */
pub const S_IWUSR:  u16 = 0o0000200;   /* W for owner */
pub const S_IXUSR:  u16 = 0o0000100;   /* X for owner */

pub const S_IRWXG:  u16 = 0o0000070;   /* RWX mask for group */
pub const S_IRGRP:  u16 = 0o0000040;   /* R for group */
pub const S_IWGRP:  u16 = 0o0000020;   /* W for group */
pub const S_IXGRP:  u16 = 0o0000010;   /* X for group */

pub const S_IRWXO:  u16 = 0o0000007;   /* RWX mask for other */
pub const S_IROTH:  u16 = 0o0000004;   /* R for other */
pub const S_IWOTH:  u16 = 0o0000002;   /* W for other */
pub const S_IXOTH:  u16 = 0o0000001;   /* X for other */

pub const S_IFMT:   u16 = 0o0170000;   /* type of file mask */
pub const S_IFIFO:  u16 = 0o0010000;   /* named pipe (fifo) */
pub const S_IFCHR:  u16 = 0o0020000;   /* character special */
pub const S_IFDIR:  u16 = 0o0040000;   /* directory */
pub const S_IFBLK:  u16 = 0o0060000;   /* block special */
pub const S_IFREG:  u16 = 0o0100000;   /* regular */
pub const S_IFLNK:  u16 = 0o0120000;   /* symbolic link */
pub const S_IFSOCK: u16 = 0o0140000;   /* socket */
pub const S_IFWHT:  u16 = 0o0160000;   /* whiteout */


// Fork Data Structure
//struct HFSPlusForkData {
//    UInt64                  logicalSize;
//    UInt32                  clumpSize;
//    UInt32                  totalBlocks;
//    HFSPlusExtentRecord     extents;
//};
//typedef struct HFSPlusForkData HFSPlusForkData;
// 
//typedef HFSPlusExtentDescriptor HFSPlusExtentRecord[8];
//
//struct HFSPlusExtentDescriptor {
//    UInt32                  startBlock;
//    UInt32                  blockCount;
//};
//typedef struct HFSPlusExtentDescriptor HFSPlusExtentDescriptor;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HFSPlusForkData {
    pub logicalSize:            u64,
    pub clumpSize:              u32,
    pub totalBlocks:            u32,
    pub extents:                HFSPlusExtentRecord,
}

pub type HFSPlusExtentRecord = [HFSPlusExtentDescriptor; 8];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct HFSPlusExtentDescriptor {
    pub startBlock:             u32,
    pub blockCount:             u32,
}

impl HFSPlusForkData {
    fn new() -> Self {
        Self {
            logicalSize:        0,
            clumpSize:          0,
            totalBlocks:        0,
            extents:            new_record(),
        }
    }

    fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            logicalSize:        source.read_u64::<BigEndian>()?,
            clumpSize:          source.read_u32::<BigEndian>()?,
            totalBlocks:        source.read_u32::<BigEndian>()?,
            extents:            import_record(source)?,
        })
    }

    fn export(&self, source: &mut Write) -> io::Result<()> {
        source.write_u64::<BigEndian>(self.logicalSize)?;
        source.write_u32::<BigEndian>(self.clumpSize)?;
        source.write_u32::<BigEndian>(self.totalBlocks)?;
        export_record(&self.extents, source)?;

        Ok(())
    }
}

//impl HFSPlusExtentRecord {
    pub fn new_record() -> HFSPlusExtentRecord {
        [
           HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0 },
           HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0 },
           HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0 },
           HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0 },
           HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0 },
           HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0 },
           HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0 },
           HFSPlusExtentDescriptor { startBlock: 0, blockCount: 0 },
        ]
    }

    //fn import(source: &mut Read) -> io::Result<Self> {
    pub fn import_record(source: &mut Read) -> io::Result<HFSPlusExtentRecord> {
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

    pub fn export_record(record: &[HFSPlusExtentDescriptor], source: &mut Write) -> io::Result<()> {
        record[0].export(source)?;
        record[1].export(source)?;
        record[2].export(source)?;
        record[3].export(source)?;
        record[4].export(source)?;
        record[5].export(source)?;
        record[6].export(source)?;
        record[7].export(source)?;

        Ok(())
    }
//}

impl HFSPlusExtentDescriptor {
    fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            startBlock:         source.read_u32::<BigEndian>()?,
            blockCount:         source.read_u32::<BigEndian>()?,
        })
    }

    fn export(&self, source: &mut Write) -> io::Result<()> {
        source.write_u32::<BigEndian>(self.startBlock)?;
        source.write_u32::<BigEndian>(self.blockCount)?;

        Ok(())
    }
}



//- Volume Header

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

#[derive(Debug, PartialEq, Eq)]
pub struct HFSPlusVolumeHeader {
    pub signature: u16,
    pub version: u16,
    pub attributes: VolumeAttributes,
    pub lastMountedVersion: u32,
    pub journalInfoBlock: u32,

    pub createDate: u32,
    pub modifyDate: u32,
    pub backupDate: u32,
    pub checkedDate: u32,

    pub fileCount: u32,
    pub folderCount: u32,

    pub blockSize: u32,
    pub totalBlocks: u32,
    pub freeBlocks: u32,

    pub nextAllocation: u32,
    pub rsrcClumpSize: u32,
    pub dataClumpSize: u32,
    pub nextCatalogID: HFSCatalogNodeID,

    pub writeCount: u32,
    pub encodingsBitmap: u64,

    pub finderInfo: [u32; 8],

    pub allocationFile: HFSPlusForkData,
    pub extentsFile: HFSPlusForkData,
    pub catalogFile: HFSPlusForkData,
    pub attributesFile: HFSPlusForkData,
    pub startupFile: HFSPlusForkData,
}

impl HFSPlusVolumeHeader {
    pub fn import(source: &mut Read) -> io::Result<Self> {
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

    pub fn export(&self, source: &mut Write) -> io::Result<()> {
        source.write_u16::<BigEndian>(self.signature)?;
        source.write_u16::<BigEndian>(self.version)?;
        source.write_u32::<BigEndian>(self.attributes.bits)?;
        source.write_u32::<BigEndian>(self.lastMountedVersion)?;
        source.write_u32::<BigEndian>(self.journalInfoBlock)?;

        source.write_u32::<BigEndian>(self.createDate)?;
        source.write_u32::<BigEndian>(self.modifyDate)?;
        source.write_u32::<BigEndian>(self.backupDate)?;
        source.write_u32::<BigEndian>(self.checkedDate)?;

        source.write_u32::<BigEndian>(self.fileCount)?;
        source.write_u32::<BigEndian>(self.folderCount)?;

        source.write_u32::<BigEndian>(self.blockSize)?;
        source.write_u32::<BigEndian>(self.totalBlocks)?;
        source.write_u32::<BigEndian>(self.freeBlocks)?;

        source.write_u32::<BigEndian>(self.nextAllocation)?;
        source.write_u32::<BigEndian>(self.rsrcClumpSize)?;
        source.write_u32::<BigEndian>(self.dataClumpSize)?;
        source.write_u32::<BigEndian>(self.nextCatalogID)?;

        source.write_u32::<BigEndian>(self.writeCount)?;
        source.write_u64::<BigEndian>(self.encodingsBitmap)?;

        source.write_u32::<BigEndian>(self.finderInfo[0])?;
        source.write_u32::<BigEndian>(self.finderInfo[1])?;
        source.write_u32::<BigEndian>(self.finderInfo[2])?;
        source.write_u32::<BigEndian>(self.finderInfo[3])?;
        source.write_u32::<BigEndian>(self.finderInfo[4])?;
        source.write_u32::<BigEndian>(self.finderInfo[5])?;
        source.write_u32::<BigEndian>(self.finderInfo[6])?;
        source.write_u32::<BigEndian>(self.finderInfo[7])?;

        self.allocationFile.export(source)?;
        self.extentsFile.export(source)?;
        self.catalogFile.export(source)?;
        self.attributesFile.export(source)?;
        self.startupFile.export(source)?;

        Ok(())
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
    pub struct VolumeAttributes: u32 {
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
        const kHFSVolumeUnusedNodeFixBit      = 1 << 31;  // Not mentioned in TN1150
    }
}

pub const HFSP_SIGNATURE: u16 = 0x482b;  // H+ Signature (Big endian)
pub const HFSX_SIGNATURE: u16 = 0x4858;  // HFSX Signature (Big endian)
pub const HFSP_VERSION: u16 = 4;  // H+ Signature (Big endian)
pub const HFSX_VERSION: u16 = 5;  // HFSX Signature (Big endian)



//- B-Trees

//struct BTNodeDescriptor {
//    UInt32    fLink;
//    UInt32    bLink;
//    SInt8     kind;
//    UInt8     height;
//    UInt16    numRecords;
//    UInt16    reserved;
//};
//typedef struct BTNodeDescriptor BTNodeDescriptor;

#[derive(Debug, PartialEq, Eq)]
pub struct BTNodeDescriptor {
    pub fLink: u32,
    pub bLink: u32,
    pub kind: i8,
    pub height: u8,
    pub numRecords: u16,
    pub reserved: u16,
}

impl BTNodeDescriptor {
    pub fn import(source: &mut Read) -> std::io::Result<Self> {
        Ok(Self {
            fLink: source.read_u32::<BigEndian>()?,
            bLink: source.read_u32::<BigEndian>()?,
            kind: source.read_i8()?,
            height: source.read_u8()?,
            numRecords: source.read_u16::<BigEndian>()?,
            reserved: source.read_u16::<BigEndian>()?,
        })
    }

    pub fn export(&self, source: &mut Write) -> std::io::Result<()> {
        source.write_u32::<BigEndian>(self.fLink)?;
        source.write_u32::<BigEndian>(self.bLink)?;
        source.write_i8(self.kind)?;
        source.write_u8(self.height)?;
        source.write_u16::<BigEndian>(self.numRecords)?;
        source.write_u16::<BigEndian>(self.reserved)?;

        Ok(())
    }
}


//enum {
//    kBTLeafNode       = -1,
//    kBTIndexNode      =  0,
//    kBTHeaderNode     =  1,
//    kBTMapNode        =  2
//};

pub const kBTLeafNode     : i8 = -1;
pub const kBTIndexNode    : i8 =  0;
pub const kBTHeaderNode   : i8 =  1;
pub const kBTMapNode      : i8 =  2;


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

#[derive(Debug, PartialEq, Eq)]
pub struct BTHeaderRec {
    pub treeDepth: u16,
    pub rootNode: u32,
    pub leafRecords: u32,
    pub firstLeafNode: u32,
    pub lastLeafNode: u32,
    pub nodeSize: u16,
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
    pub fn import(source: &mut Read) -> std::io::Result<Self> {
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

    pub fn export(&self, source: &mut Write) -> std::io::Result<()> {
        source.write_u16::<BigEndian>(self.treeDepth)?;
        source.write_u32::<BigEndian>(self.rootNode)?;
        source.write_u32::<BigEndian>(self.leafRecords)?;
        source.write_u32::<BigEndian>(self.firstLeafNode)?;
        source.write_u32::<BigEndian>(self.lastLeafNode)?;
        source.write_u16::<BigEndian>(self.nodeSize)?;
        source.write_u16::<BigEndian>(self.maxKeyLength)?;
        source.write_u32::<BigEndian>(self.totalNodes)?;
        source.write_u32::<BigEndian>(self.freeNodes)?;
        source.write_u16::<BigEndian>(self.reserved1)?;
        source.write_u32::<BigEndian>(self.clumpSize)?;
        source.write_u8(self.btreeType)?;
        source.write_u8(self.keyCompareType)?;
        source.write_u32::<BigEndian>(self.attributes)?;
        source.write_u32::<BigEndian>(self.reserved3[0])?;
        source.write_u32::<BigEndian>(self.reserved3[1])?;
        source.write_u32::<BigEndian>(self.reserved3[2])?;
        source.write_u32::<BigEndian>(self.reserved3[3])?;
        source.write_u32::<BigEndian>(self.reserved3[4])?;
        source.write_u32::<BigEndian>(self.reserved3[5])?;
        source.write_u32::<BigEndian>(self.reserved3[6])?;
        source.write_u32::<BigEndian>(self.reserved3[7])?;
        source.write_u32::<BigEndian>(self.reserved3[8])?;
        source.write_u32::<BigEndian>(self.reserved3[9])?;
        source.write_u32::<BigEndian>(self.reserved3[10])?;
        source.write_u32::<BigEndian>(self.reserved3[11])?;
        source.write_u32::<BigEndian>(self.reserved3[12])?;
        source.write_u32::<BigEndian>(self.reserved3[13])?;
        source.write_u32::<BigEndian>(self.reserved3[14])?;
        source.write_u32::<BigEndian>(self.reserved3[15])?;

        Ok(())
    }
}


//enum BTreeTypes{
//    kHFSBTreeType           =   0,      // control file
//    kUserBTreeType          = 128,      // user btree type starts from 128
//    kReservedBTreeType      = 255
//};
//
//
//enum {
//    kBTBadCloseMask           = 0x00000001,
//    kBTBigKeysMask            = 0x00000002,
//    kBTVariableIndexKeysMask  = 0x00000004
//};



//- Catalog File

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

pub type HFSCatalogNodeID = u32;

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

// Catalog File Key
//struct HFSPlusCatalogKey {
//    UInt16              keyLength;
//    HFSCatalogNodeID    parentID;
//    HFSUniStr255        nodeName;
//};
//typedef struct HFSPlusCatalogKey HFSPlusCatalogKey;
//
//
//enum {
//    kHFSPlusFolderRecord        = 0x0001,
//    kHFSPlusFileRecord          = 0x0002,
//    kHFSPlusFolderThreadRecord  = 0x0003,
//    kHFSPlusFileThreadRecord    = 0x0004
//};

pub const kHFSPlusFolderRecord        : i16 = 0x0001;
pub const kHFSPlusFileRecord          : i16 = 0x0002;
pub const kHFSPlusFolderThreadRecord  : i16 = 0x0003;
pub const kHFSPlusFileThreadRecord    : i16 = 0x0004;


//struct HFSPlusCatalogFolder {
//    SInt16              recordType;
//    UInt16              flags;
//    UInt32              valence;
//    HFSCatalogNodeID    folderID;
//    UInt32              createDate;
//    UInt32              contentModDate;
//    UInt32              attributeModDate;
//    UInt32              accessDate;
//    UInt32              backupDate;
//    HFSPlusBSDInfo      permissions;
//    FolderInfo          userInfo;
//    ExtendedFolderInfo  finderInfo;
//    UInt32              textEncoding;
//    UInt32              reserved;
//};
//typedef struct HFSPlusCatalogFolder HFSPlusCatalogFolder;

#[derive(Debug, Copy, Clone)]
pub struct HFSPlusCatalogFolder {
    //pub recordType:           i16,
    pub flags:                  u16,
    pub valence:                u32,
    pub folderID:               HFSCatalogNodeID,
    pub createDate:             u32,
    pub contentModDate:         u32,
    pub attributeModDate:       u32,
    pub accessDate:             u32,
    pub backupDate:             u32,
    pub permissions:            HFSPlusBSDInfo,
    pub userInfo:               FolderInfo,
    pub finderInfo:             ExtendedFolderInfo,
    pub textEncoding:           u32,
    pub reserved:               u32,
}

impl HFSPlusCatalogFolder {
    pub fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            //recordType:         source.read_i16::<BigEndian>()?,
            flags:              source.read_u16::<BigEndian>()?,
            valence:            source.read_u32::<BigEndian>()?,
            folderID:           source.read_u32::<BigEndian>()?,  // HFSCatalogNodeID
            createDate:         source.read_u32::<BigEndian>()?,
            contentModDate:     source.read_u32::<BigEndian>()?,
            attributeModDate:   source.read_u32::<BigEndian>()?,
            accessDate:         source.read_u32::<BigEndian>()?,
            backupDate:         source.read_u32::<BigEndian>()?,
            permissions:        HFSPlusBSDInfo::import(source)?,
            userInfo:           FolderInfo::import(source)?,
            finderInfo:         ExtendedFolderInfo::import(source)?,
            textEncoding:       source.read_u32::<BigEndian>()?,
            reserved:           source.read_u32::<BigEndian>()?,
        })
    }
}


//struct HFSPlusCatalogFile {
//    SInt16              recordType;
//    UInt16              flags;
//    UInt32              reserved1;
//    HFSCatalogNodeID    fileID;
//    UInt32              createDate;
//    UInt32              contentModDate;
//    UInt32              attributeModDate;
//    UInt32              accessDate;
//    UInt32              backupDate;
//    HFSPlusBSDInfo      permissions;
//    FileInfo            userInfo;
//    ExtendedFileInfo    finderInfo;
//    UInt32              textEncoding;
//    UInt32              reserved2;
// 
//    HFSPlusForkData     dataFork;
//    HFSPlusForkData     resourceFork;
//};
//typedef struct HFSPlusCatalogFile HFSPlusCatalogFile;
//
//
//enum {
//    kHFSFileLockedBit       = 0x0000,
//    kHFSFileLockedMask      = 0x0001,
//    kHFSThreadExistsBit     = 0x0001,
//    kHFSThreadExistsMask    = 0x0002
//};

#[derive(Debug, Copy, Clone)]
pub struct HFSPlusCatalogFile {
    //pub recordType:             i16,
    pub flags:                  u16,
    pub reserved1:              u32,
    pub fileID:                 HFSCatalogNodeID,
    pub createDate:             u32,
    pub contentModDate:         u32,
    pub attributeModDate:       u32,
    pub accessDate:             u32,
    pub backupDate:             u32,
    pub permissions:            HFSPlusBSDInfo,
    pub userInfo:               FileInfo,
    pub finderInfo:             ExtendedFileInfo,
    pub textEncoding:           u32,
    pub reserved2:              u32,
 
    pub dataFork:               HFSPlusForkData,
    pub resourceFork:           HFSPlusForkData,
}

impl HFSPlusCatalogFile {
    pub fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            //recordType:         source.read_i16::<BigEndian>()?,
            flags:              source.read_u16::<BigEndian>()?,
            reserved1:          source.read_u32::<BigEndian>()?,
            fileID:             source.read_u32::<BigEndian>()?,
            createDate:         source.read_u32::<BigEndian>()?,
            contentModDate:     source.read_u32::<BigEndian>()?,
            attributeModDate:   source.read_u32::<BigEndian>()?,
            accessDate:         source.read_u32::<BigEndian>()?,
            backupDate:         source.read_u32::<BigEndian>()?,
            permissions:        HFSPlusBSDInfo::import(source)?,
            userInfo:           FileInfo::import(source)?,
            finderInfo:         ExtendedFileInfo::import(source)?,
            textEncoding:       source.read_u32::<BigEndian>()?,
            reserved2:          source.read_u32::<BigEndian>()?,
         
            dataFork:           HFSPlusForkData::import(source)?,
            resourceFork:       HFSPlusForkData::import(source)?,
        })
    }
}

//struct HFSPlusCatalogThread {
//    SInt16              recordType;
//    SInt16              reserved;
//    HFSCatalogNodeID    parentID;
//    HFSUniStr255        nodeName;
//};
//typedef struct HFSPlusCatalogThread HFSPlusCatalogThread;



//- Finder Info
// The following is described in Apple's Finder Interface Reference

//struct Point {
//  SInt16              v;
//  SInt16              h;
//};
//typedef struct Point  Point;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub v:                      i16,
    pub h:                      i16,
}

impl Point {
    fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            v:                  source.read_i16::<BigEndian>()?,
            h:                  source.read_i16::<BigEndian>()?,
        })
    }
}


//struct Rect {
//  SInt16              top;
//  SInt16              left;
//  SInt16              bottom;
//  SInt16              right;
//};
//typedef struct Rect   Rect;

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub top:	                i16,
    pub left:	                i16,
    pub bottom:	                i16,
    pub right:	                i16,
}

impl Rect {
    fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            top:	        source.read_i16::<BigEndian>()?,
            left:	        source.read_i16::<BigEndian>()?,
            bottom:	        source.read_i16::<BigEndian>()?,
            right:	        source.read_i16::<BigEndian>()?,
        })
    }
}

// /* OSType is a 32-bit value made by packing four 1-byte characters 
//   together. */
//typedef UInt32        FourCharCode;
//typedef FourCharCode  OSType;

pub type OSType = u32;

//
// /* Finder flags (finderFlags, fdFlags and frFlags) */
//enum {
//  kIsOnDesk       = 0x0001,     /* Files and folders (System 6) */
//  kColor          = 0x000E,     /* Files and folders */
//  kIsShared       = 0x0040,     /* Files only (Applications only) If */
//                                /* clear, the application needs */
//                                /* to write to its resource fork, */
//                                /* and therefore cannot be shared */
//                                /* on a server */
//  kHasNoINITs     = 0x0080,     /* Files only (Extensions/Control */
//                                /* Panels only) */
//                                /* This file contains no INIT resource */
//  kHasBeenInited  = 0x0100,     /* Files only.  Clear if the file */
//                                /* contains desktop database resources */
//                                /* ('BNDL', 'FREF', 'open', 'kind'...) */
//                                /* that have not been added yet.  Set */
//                                /* only by the Finder. */
//                                /* Reserved for folders */
//  kHasCustomIcon  = 0x0400,     /* Files and folders */
//  kIsStationery   = 0x0800,     /* Files only */
//  kNameLocked     = 0x1000,     /* Files and folders */
//  kHasBundle      = 0x2000,     /* Files only */
//  kIsInvisible    = 0x4000,     /* Files and folders */
//  kIsAlias        = 0x8000      /* Files only */
//};
//
// /* Extended flags (extendedFinderFlags, fdXFlags and frXFlags) */
//enum {
//  kExtendedFlagsAreInvalid    = 0x8000, /* The other extended flags */
//                                        /* should be ignored */
//  kExtendedFlagHasCustomBadge = 0x0100, /* The file or folder has a */
//                                        /* badge resource */
//  kExtendedFlagHasRoutingInfo = 0x0004  /* The file contains routing */
//                                        /* info resource */
//};
//
//struct FileInfo {
//  OSType    fileType;           /* The type of the file */
//  OSType    fileCreator;        /* The file's creator */
//  UInt16    finderFlags;
//  Point     location;           /* File's location in the folder. */
//  UInt16    reservedField;
//};
//typedef struct FileInfo   FileInfo;

#[derive(Debug, Copy, Clone)]
pub struct FileInfo {
    pub fileType:               OSType, /* The type of the file */
    pub fileCreator:	        OSType, /* The file's creator */
    pub finderFlags:	        u16,
    pub location:               Point,  /* File's location in the folder. */
    pub reservedField:	        u16,
}

impl FileInfo {
    fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            fileType:	        source.read_u32::<BigEndian>()?,
            fileCreator:	source.read_u32::<BigEndian>()?,
            finderFlags:	source.read_u16::<BigEndian>()?,
            location:	        Point::import(source)?,
            reservedField:	source.read_u16::<BigEndian>()?,
        })
    }
}

//struct ExtendedFileInfo {
//  SInt16    reserved1[4];
//  UInt16    extendedFinderFlags;
//  SInt16    reserved2;
//  SInt32    putAwayFolderID;
//};
//typedef struct ExtendedFileInfo   ExtendedFileInfo;

#[derive(Debug, Copy, Clone)]
pub struct ExtendedFileInfo {
    pub reserved1:        	[i16; 4],
    pub extendedFinderFlags:	u16,
    pub reserved2:              i16,
    pub putAwayFolderID:        i32,
}

impl ExtendedFileInfo {
    fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            reserved1:	        [
                source.read_i16::<BigEndian>()?,
                source.read_i16::<BigEndian>()?,
                source.read_i16::<BigEndian>()?,
                source.read_i16::<BigEndian>()?,
            ],
            extendedFinderFlags:source.read_u16::<BigEndian>()?,
            reserved2:	        source.read_i16::<BigEndian>()?,
            putAwayFolderID:	source.read_i32::<BigEndian>()?,
        })
    }
}

//struct FolderInfo {
//  Rect      windowBounds;       /* The position and dimension of the */
//                                /* folder's window */
//  UInt16    finderFlags;
//  Point     location;           /* Folder's location in the parent */
//                                /* folder. If set to {0, 0}, the Finder */
//                                /* will place the item automatically */
//  UInt16    reservedField;
//};
//typedef struct FolderInfo   FolderInfo;

#[derive(Debug, Copy, Clone)]
pub struct FolderInfo {
    pub windowBounds:	        Rect,   /* The position and dimension of the */
                                        /* folder's window */
    pub finderFlags:	        u16,
    pub location:	        Point,  /* Folder's location in the parent */
                                        /* folder. If set to {0, 0}, the Finder */
                                        /* will place the item automatically */
    pub reservedField:	        u16,
}

impl FolderInfo {
    fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            windowBounds:       Rect::import(source)?,
            finderFlags:        source.read_u16::<BigEndian>()?,
            location:           Point::import(source)?,
            reservedField:      source.read_u16::<BigEndian>()?,
        })
    }
}

//struct ExtendedFolderInfo {
//  Point     scrollPosition;     /* Scroll position (for icon views) */
//  SInt32    reserved1;
//  UInt16    extendedFinderFlags;
//  SInt16    reserved2;
//  SInt32    putAwayFolderID;
//};
//typedef struct ExtendedFolderInfo   ExtendedFolderInfo;

#[derive(Debug, Copy, Clone)]
pub struct ExtendedFolderInfo {
    pub scrollPosition:	        Point,  /* Scroll position (for icon views) */
    pub reserved1:              i32,
    pub extendedFinderFlags:    u16,
    pub reserved2:              i16,
    pub putAwayFolderID:        i32,
}

impl ExtendedFolderInfo {
    fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            scrollPosition:	Point::import(source)?,
            reserved1:          source.read_i32::<BigEndian>()?,
            extendedFinderFlags:source.read_u16::<BigEndian>()?,
            reserved2:          source.read_i16::<BigEndian>()?,
            putAwayFolderID:    source.read_i32::<BigEndian>()?,
        })
    }
}



//- Extents Overflow File

//struct HFSPlusExtentKey {
//    UInt16              keyLength;
//    UInt8               forkType;
//    UInt8               pad;
//    HFSCatalogNodeID    fileID;
//    UInt32              startBlock;
//};
//typedef struct HFSPlusExtentKey HFSPlusExtentKey;

#[derive(Debug)]
pub struct HFSPlusExtentKey {
    pub keyLength:              u16,
    pub forkType:	        u8,
    pub pad:	                u8,
    pub fileID:	                HFSCatalogNodeID,
    pub startBlock:	        u32,
}

impl HFSPlusExtentKey {
    pub fn import(source: &mut Read) -> io::Result<Self> {
        Ok(Self {
            keyLength:          source.read_u16::<BigEndian>()?,
            forkType:	        source.read_u8()?,
            pad:	        source.read_u8()?,
            fileID:	        source.read_u32::<BigEndian>()?,
            startBlock:	        source.read_u32::<BigEndian>()?,
        })
    }

    pub fn export(&self, source: &mut Write) -> io::Result<()> {
        source.write_u16::<BigEndian>(self.keyLength)?;
        source.write_u8(self.forkType)?;
        source.write_u8(self.pad)?;
        source.write_u32::<BigEndian>(self.fileID)?;
        source.write_u32::<BigEndian>(self.startBlock)?;

        Ok(())
    }
}



//- Attributes File

//enum {
//    kHFSPlusAttrInlineData  = 0x10,
//    kHFSPlusAttrForkData    = 0x20,
//    kHFSPlusAttrExtents     = 0x30
//};

// Fork Data Attributes
//struct HFSPlusAttrForkData {
//    UInt32          recordType;
//    UInt32          reserved;
//    HFSPlusForkData theFork;
//};
//typedef struct HFSPlusAttrForkData HFSPlusAttrForkData;

// Extension Attributes
//struct HFSPlusAttrExtents {
//    UInt32                  recordType;
//    UInt32                  reserved;
//    HFSPlusExtentRecord     extents;
//};
//typedef struct HFSPlusAttrExtents HFSPlusAttrExtents;


//enum {
//    kHardLinkFileType = 0x686C6E6B,  /* 'hlnk' */
//    kHFSPlusCreator   = 0x6866732B   /* 'hfs+' */
//};
//
//enum {
//    kSymLinkFileType  = 0x736C6E6B, /* 'slnk' */
//    kSymLinkCreator   = 0x72686170  /* 'rhap' */
//};



//- Journal

//struct JournalInfoBlock {
//    UInt32    flags;
//    UInt32    device_signature[8];
//    UInt64    offset;
//    UInt64    size;
//    UInt32    reserved[32];
//};
//typedef struct JournalInfoBlock JournalInfoBlock;
//
//enum {
//    kJIJournalInFSMask          = 0x00000001,
//    kJIJournalOnOtherDeviceMask = 0x00000002,
//    kJIJournalNeedInitMask      = 0x00000004
//};
//
//typedef struct journal_header {
//    UInt32    magic;
//    UInt32    endian;
//    UInt64    start;
//    UInt64    end;
//    UInt64    size;
//    UInt32    blhdr_size;
//    UInt32    checksum;
//    UInt32    jhdr_size;
//} journal_header;
//
//#define JOURNAL_HEADER_MAGIC  0x4a4e4c78
//#define ENDIAN_MAGIC          0x12345678
//
//typedef struct block_list_header {
//    UInt16    max_blocks;
//    UInt16    num_blocks;
//    UInt32    bytes_used;
//    UInt32    checksum;
//    UInt32    pad;
//    block_info  binfo[1];
//} block_list_header;
//
//typedef struct block_info {
//    UInt64    bnum;
//    UInt32    bsize;
//    UInt32    next;
//} block_info;



//- Hot Files

//#define HFC_MAGIC   0xFF28FF26
//#define HFC_VERSION 1
//#define HFC_DEFAULT_DURATION     (3600 * 60)
//#define HFC_MINIMUM_TEMPERATURE  16
//#define HFC_MAXIMUM_FILESIZE     (10 * 1024 * 1024)
//char hfc_tag[] = "CLUSTERED HOT FILES B-TREE     ";
//
//struct HotFilesInfo {
//    UInt32  magic;
//    UInt32  version;
//    UInt32  duration;    /* duration of sample period */
//    UInt32  timebase;    /* recording period start time */
//    UInt32  timeleft;    /* recording period stop time */
//    UInt32  threshold;
//    UInt32  maxfileblks;
//    UInt32  maxfilecnt;
//    UInt8   tag[32];
//};
//typedef struct HotFilesInfo HotFilesInfo;
//
//struct HotFileKey {
//    UInt16   keyLength;
//    UInt8    forkType;
//    UInt8    pad;
//    UInt32   temperature;
//    UInt32   fileID;
//};
//typedef struct HotFileKey HotFileKey;
//
//#define HFC_LOOKUPTAG   0xFFFFFFFF
//#define HFC_KEYLENGTH   (sizeof(HotFileKey) - sizeof(UInt32))
