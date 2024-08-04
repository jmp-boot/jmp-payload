// SPDX-FileCopyrightText: 2024 The JMP.boot Developers
//
// SPDX-License-Identifier: GPL-3.0-only

#![no_main]
#![no_std]

use log::info;
use uefi::prelude::*;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();
    info!("Welcome to JMP.boot!");
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}
