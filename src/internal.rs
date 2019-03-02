/// Core Concepts

// HFS Plus Names
//struct HFSUniStr255 {
//        UInt16  length;
//            UniChar unicode[255];
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


/// Volume Header

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
//
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


/// B-Trees

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
//
//enum {
//    kBTLeafNode       = -1,
//    kBTIndexNode      =  0,
//    kBTHeaderNode     =  1,
//    kBTMapNode        =  2
//};
//
//
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
//
//
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


/// Catalog File

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
//
//
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
//
//
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
//
//
//struct HFSPlusCatalogThread {
//    SInt16              recordType;
//    SInt16              reserved;
//    HFSCatalogNodeID    parentID;
//    HFSUniStr255        nodeName;
//};
//typedef struct HFSPlusCatalogThread HFSPlusCatalogThread;


/// Finder Info
// The following is described in Apple's Finder Interface Reference

//struct Point {
//  SInt16              v;
//  SInt16              h;
//};
//typedef struct Point  Point;
//
//struct Rect {
//  SInt16              top;
//  SInt16              left;
//  SInt16              bottom;
//  SInt16              right;
//};
//typedef struct Rect   Rect;
//
///* OSType is a 32-bit value made by packing four 1-byte characters 
//   together. */
//typedef UInt32        FourCharCode;
//typedef FourCharCode  OSType;
//
///* Finder flags (finderFlags, fdFlags and frFlags) */
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
///* Extended flags (extendedFinderFlags, fdXFlags and frXFlags) */
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
//
//struct ExtendedFileInfo {
//  SInt16    reserved1[4];
//  UInt16    extendedFinderFlags;
//  SInt16    reserved2;
//  SInt32    putAwayFolderID;
//};
//typedef struct ExtendedFileInfo   ExtendedFileInfo;
//
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
//
//struct ExtendedFolderInfo {
//  Point     scrollPosition;     /* Scroll position (for icon views) */
//  SInt32    reserved1;
//  UInt16    extendedFinderFlags;
//  SInt16    reserved2;
//  SInt32    putAwayFolderID;
//};
//typedef struct ExtendedFolderInfo   ExtendedFolderInfo;


/// Extents Overflow File

//struct HFSPlusExtentKey {
//    UInt16              keyLength;
//    UInt8               forkType;
//    UInt8               pad;
//    HFSCatalogNodeID    fileID;
//    UInt32              startBlock;
//};
//typedef struct HFSPlusExtentKey HFSPlusExtentKey;


/// Attributes File

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


/// Journal

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


/// Hot Files

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
