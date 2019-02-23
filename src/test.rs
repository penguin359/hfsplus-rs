//use crate::*;
use super::*;

#[test]
fn load_blank_volume() {
    let expected = HFSPlusVolumeHeader {
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
    };

    let mut file = File::open("hfsp-blank.img").unwrap();
    file.seek(std::io::SeekFrom::Start(1024)).expect("Failed to seek in file");
    let actual = HFSPlusVolumeHeader::import(&mut file).expect("Failed to read Volume Header");
    assert_eq!(expected, actual);
}
