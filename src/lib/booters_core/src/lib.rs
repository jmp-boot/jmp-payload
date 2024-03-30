// SPDX-FileCopyrightText: 2024 The Booters Developers
//
// SPDX-License-Identifier: GPL-3.0-only

#![no_std]

extern crate alloc;

pub mod reexports {
    pub use embedded_io;
    pub use gpt;
    pub mod core {
        pub use crate::traits;
    }
}

pub mod traits {
    /// This module contains the storage-related functionality.
    pub mod storage {
        use alloc::boxed::Box;
        use core::ops::Not;
        use gpt::partition_types::Type as PartType;

        /// This trait represents a base storage device provider.
        pub trait BaseStorageDeviceProvider {
            /// Synchronize the storage device.
            fn sync(&self);

            /// Open a partition on the storage device.
            ///
            /// # Arguments
            ///
            /// * `part` - The name of the partition.
            /// * `fs` - The filesystem type (optional).
            ///
            /// # Returns
            ///
            /// A boxed trait object representing the base partition provider.
            fn open(part: &str, fs: Option<&str>) -> Box<dyn BasePartitionProvider>;

            /// Test if the super block of a partition is valid.
            ///
            /// # Arguments
            ///
            /// * `part` - The name of the partition.
            /// * `fs` - The filesystem type (optional).
            ///
            /// # Returns
            ///
            /// `true` if the super block is valid, `false` otherwise.
            fn test_super_block(part: &str, fs: Option<&str>) -> bool;
        }

        /// This trait represents a base storage I/O provider.
        pub trait BaseStorageIoProvider {
            /// Open a file.
            ///
            /// # Arguments
            ///
            /// * `path` - The path to the file.
            fn open_file(path: &str);

            /// Delete a file.
            ///
            /// # Arguments
            ///
            /// * `path` - The path to the file.
            fn delete_file(path: &str);

            /// Rename a file.
            ///
            /// # Arguments
            ///
            /// * `path` - The path to the file.
            /// * `new_name` - The new name of the file.
            fn rename_file(path: &str, new_name: &str);
        }

        /// This trait represents a base partition provider.
        pub trait BasePartitionProvider {
            /// Get the label of the partition.
            ///
            /// # Arguments
            ///
            /// * `part` - The name of the partition.
            ///
            /// # Returns
            ///
            /// The label of the partition.
            fn get_label(&self, part: &str) -> &'static str;

            /// Get the GPT partition GUID.
            ///
            /// # Returns
            ///
            /// The GPT partition GUID.
            fn gpt_part_guid(&self) -> &'static str;

            /// Check if the partition is clean.
            ///
            /// # Returns
            ///
            /// `true` if the partition is clean, `false` otherwise.
            fn is_clean(&self) -> bool;

            /// Check if the partition is dirty.
            ///
            /// # Returns
            ///
            /// `true` if the partition is dirty, `false` otherwise.
            fn is_dirty(&self) -> bool {
                self.is_clean().not()
            }

            /// Check if the partition is open.
            ///
            /// # Returns
            ///
            /// `true` if the partition is open, `false` otherwise.
            fn is_open(&self) -> bool;

            /// Check if the partition is closed.
            ///
            /// # Returns
            ///
            /// `true` if the partition is closed, `false` otherwise.
            fn is_closed(&self) -> bool {
                self.is_open().not()
            }

            /// Get the GPT partition type.
            ///
            /// # Returns
            ///
            /// The GPT partition type.
            fn gpt_part_type(&self) -> PartType;

            /// Check if the partition is read-only.
            ///
            /// # Returns
            ///
            /// `true` if the partition is read-only, `false` otherwise.
            fn is_ro(&self) -> bool {
                self.is_rw().not()
            }

            /// Check if the partition is read-write.
            ///
            /// # Returns
            ///
            /// `true` if the partition is read-write, `false` otherwise.
            fn is_rw(&self) -> bool;

            /// Test if the super block of the partition is valid.
            ///
            /// # Arguments
            ///
            /// * `part` - The name of the partition.
            /// * `fs` - The filesystem type (optional).
            ///
            /// # Returns
            ///
            /// `true` if the super block is valid, `false` otherwise.
            fn test_super_block(&self, part: &str, fs: Option<&str>) -> bool;
        }

        /// This module contains the device-related functionality.
        pub mod devices {
            use super::{BaseStorageDeviceProvider, BaseStorageIoProvider};
            use core::ops::Not;

            /// This trait represents an eMMC storage provider.
            pub trait EmmcStorageProvider:
                BaseStorageDeviceProvider + BaseStorageIoProvider
            {
                /// Check if the storage device is forbidden.
                ///
                /// # Returns
                ///
                /// `true` if the storage device is forbidden, `false` otherwise.
                fn is_forbidden(&self) -> bool;

                /// Check if the storage device is used for user data.
                ///
                /// # Returns
                ///
                /// `true` if the storage device is used for user data, `false` otherwise.
                fn is_userdata(&self) -> bool;

                /// Check if the storage device is write-protected.
                ///
                /// # Returns
                ///
                /// `true` if the storage device is write-protected, `false` otherwise.
                fn is_write_protected(&self) -> bool {
                    self.is_forbidden()
                }

                /// Check if the storage device is permitted.
                ///
                /// # Returns
                ///
                /// `true` if the storage device is permitted, `false` otherwise.
                fn is_permitted(&self) -> bool {
                    self.is_forbidden().not()
                }

                /// Check if the storage device is a ROM storage.
                ///
                /// # Returns
                ///
                /// `true` if the storage device is a ROM storage, `false` otherwise.
                fn is_rom_storage(&self) -> bool;
            }

            /// This trait represents an SD storage provider.
            pub trait SdStorageProvider: BaseStorageDeviceProvider + BaseStorageIoProvider {
                /// Check if the storage device is a ROM storage.
                ///
                /// # Returns
                ///
                /// `true` if the storage device is a ROM storage, `false` otherwise.
                fn is_rom_storage(&self) -> bool;

                /// Get the SD voltage.
                ///
                /// # Returns
                ///
                /// The SD voltage.
                fn get_sd_voltage(&self) -> u16;
            }
        }
    }

    pub mod low_level {
        #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
        pub enum Endianness {
            Little,
            Big,
            #[default]
            Undefined,
        }

        pub trait BaseCpuProvider {
            fn get_endianness(&self) -> self::Endianness;
            fn get_num_cores(&self) -> u8;
        }

        /// Trait representing a Boot ROM provider.
        pub trait BromProvider {
            /// Returns the Boot ROM version.
            fn get_brom_version(&self) -> &'static str;
        }
    }

    pub mod boot_modes {
        /// This trait represents a Fastboot device and defines various methods to interact with it.
        pub trait Fastboot {
            /// Checks if the device is connected via USB.
            ///
            /// Returns `true` if the device is connected via USB, `false` otherwise.
            fn is_usb_connected(&self) -> bool;

            /// Checks if there is a transmission in progress.
            ///
            /// Returns `true` if there is a transmission in progress, `false` otherwise.
            fn is_tx_in_progress(&self) -> bool;

            /// Checks if the storage is available.
            ///
            /// Returns `true` if the storage is available, `false` otherwise.
            fn is_storage_available(&self) -> bool;

            /// Retrieves the current slot of the device.
            ///
            /// Returns `Some(slot)` if the device is an A/B device and the current slot is `slot`,
            /// or `None` if the device is not an A/B device.
            fn current_slot(&self) -> Option<char> {
                None // Default value - non A/B device.
            }

            /// Checks if the current slot of the device is slot A.
            ///
            /// Returns `true` if the current slot is A, `false` otherwise.
            fn current_slot_is_a(&self) -> bool {
                if let Some(slot) = self.current_slot() {
                    // Return true if current slot is A, false otherwise.
                    return slot == 'a';
                }

                false
            }

            /// Checks if the current slot of the device is slot B.
            ///
            /// Returns `true` if the current slot is B, `false` otherwise.
            fn current_slot_is_b(&self) -> bool {
                if let Some(slot) = self.current_slot() {
                    // Return true if current slot is B, false otherwise.
                    return slot == 'b';
                }

                false
            }

            /// Checks if the device is an A/B device.
            ///
            /// Returns `true` if the device is an A/B device, `false` otherwise.
            fn is_ab_device(&self) -> bool {
                // Return true if A/B device, f otherwise.
                // This is determined by checking if `current_slot()` returns Some.
                self.current_slot().is_some()
            }

            /// Checks if the device is an A-only device.
            ///
            /// Returns `true` if the device is an A-only device, `false` otherwise.
            fn is_a_only_device(&self) -> bool {
                // Return true if A-only device, false otherwise.
                // This is determined by checking if `current_slot()` returns None.
                self.current_slot().is_none()
            }

            /// Retrieves the product name of the device.
            ///
            /// Returns the product name as a static string.
            fn get_product_name(&self) -> &'static str;

            /// Retrieves the manufacturer name of the device.
            ///
            /// Returns the manufacturer name as a static string.
            fn get_manufacturer_name(&self) -> &'static str;
        }
    }
}
