use std::env;
use std::path::PathBuf;

fn main() {
    let kernel = PathBuf::from(env!("CARGO_BIN_FILE_MICROS_KERNEL"));
    let Some(out_dir) = env::args().skip(1).next().map(PathBuf::from) else {
        println!("out_dir argument is missing");
        return;
    };
    std::fs::create_dir_all(out_dir.clone()).unwrap();

    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&uefi_path)
        .unwrap();
    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .unwrap();
}
