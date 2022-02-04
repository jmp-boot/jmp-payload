#![no_std]
#![no_main]
#![feature(abi_efiapi)]

use core::panic::PanicInfo;
use uefi::prelude::*;
use uefi::ResultExt;

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap_success();

    Status::SUCCESS
}
