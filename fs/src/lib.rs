#![no_std]
#![deny(missing_docs)]
extern crate alloc;

mod block_cache;
mod block_dev;
mod layout;
mod bitmap;
mod fs;
mod vfs;

///the block size
pub const BLOCK_SZ: usize = 512;
pub use block_dev::BlockDevice;
use block_cache::{block_cache_sync_all, get_block_cache};
use layout::*;
use bitmap::Bitmap;
pub use fs::FileSystem;
use vfs::Inode;