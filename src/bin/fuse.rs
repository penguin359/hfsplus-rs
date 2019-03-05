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
