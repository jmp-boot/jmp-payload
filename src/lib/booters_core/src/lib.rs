// SPDX-FileCopyrightText: 2023 The Booters Developers
//
// SPDX-License-Identifier: GPL-3.0-only

#![no_std]

pub mod reexports {
    pub use gpt;
    pub use embedded_io;
}

pub mod storage_traits {
    use gpt::partition_types::Type as PartType;

    pub trait BaseStorageProvider {
        fn close(&self, force: Option<bool>);
        fn gpt_part_guid(&self) -> &'static str;
        fn gpt_part_type(&self) -> PartType;
        fn is_closed(&self) -> bool {
            !self.is_open()
        }
        fn is_dirty(&self) -> bool;
        fn is_open(&self) -> bool;
        fn is_ro(&self) -> bool {
            !self.is_rw()
        }
        fn is_rw(&self) -> bool;
        fn open(part: &str, fs: Option<&str>, auto_guess_fs: bool);
        fn test_super_block(part: &str);
    }

    pub trait BaseStorageIoProvider {
        fn open_file(path: &str);
        fn delete_file(path: &str);
        fn rename_file(path: &str, new_name: &str);
    }

    pub trait EmmcStorageProvider : BaseStorageProvider + BaseStorageIoProvider {
        fn is_forbidden(&self) -> bool;
        fn is_userdata(&self) -> bool;
        fn is_write_protected(&self) -> bool {
            !self.is_forbidden()
        }
    }

    pub trait SdStorageProvider : BaseStorageProvider + BaseStorageIoProvider {
        fn is_rom_storage(&self) -> bool;
        fn is_userdata(&self) -> bool;
    }
}
