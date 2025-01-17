use std::env;
use std::path::PathBuf;

fn main() {
    let args = env::args().skip(1).collect();
    let (kernel, out_dir) = match <Vec<_> as TryInto<[String; 2]>>::try_into(args) {
        Ok([kernel, out_dir]) => (PathBuf::from(kernel), PathBuf::from(out_dir)),
        _ => {
            println!("2 arguments expected: kernel and out_dir");
            return;
        }
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
