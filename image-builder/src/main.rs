use std::env;
use std::path::PathBuf;

fn main() {
    let kernel = PathBuf::from(env!("CARGO_BIN_FILE_MICROS_KERNEL"));
    let out_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();

    println!("creating disk images for kernel: {}", kernel.display());
    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&uefi_path)
        .unwrap();
    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .unwrap();
}
