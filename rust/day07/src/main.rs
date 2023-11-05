mod filesystem;
mod filesystem_entry;
mod filesystem_walker;

use std::{env, fs::File, io::BufReader};

use crate::{filesystem::Filesystem, filesystem_walker::FilesystemWalker};

const AVAILABLE_DISK_SPACE: u32 = 70_000_000;
const REQUIRED_DISK_SPACE: u32 = 30_000_000;

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        let filesystem_descriptor =
            File::open(file_path).expect("Failed to open filesystem_descriptor file");
        let mut filesystem_descriptor = BufReader::new(filesystem_descriptor);
        let mut filesystem = Filesystem::new();

        filesystem.build_with_descriptor(&mut filesystem_descriptor);

        let total_size_of_small_directories: u32 = FilesystemWalker::traverse(&filesystem)
            .filter(|entry| entry.is_directory() && entry.size() <= 100_000)
            .map(|entry| entry.size())
            .sum();

        println!("Total size of directories that are at most 100'000 bytes big: {total_size_of_small_directories}");

        let unused_disk_space = AVAILABLE_DISK_SPACE - filesystem.size();
        let disk_space_that_must_be_freed = REQUIRED_DISK_SPACE - unused_disk_space;

        let total_size_of_directory_to_delete: u32 = FilesystemWalker::traverse(&filesystem)
            .filter(|entry| entry.is_directory() && entry.size() >= disk_space_that_must_be_freed)
            .map(|entry| entry.size())
            .min()
            .unwrap();

        println!(
            "Total size of directory that should be deleted: {total_size_of_directory_to_delete}"
        );
    } else {
        panic!("Did not provide path to filesystem_descriptor file");
    }
}
