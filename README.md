# boot_frame

This is a 'bootloader framework'. It has archicecture-specific crates, used for
generating bootloader payloads for devices, and has a friendly, extendable API
for building custom bootloaders.

Initially, we are targeting Mediatek devices, but we have stub crates in place
for x86 [MBR] BIOS, x86_64 [MBR] BIOS and x86_64 UEFI.

## License

This is [licensed][license] under the MIT license.

[license]: /LICENSE
