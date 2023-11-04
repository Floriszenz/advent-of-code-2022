mod filesystem;
mod filesystem_entry;

use std::{env, fs::File, io::BufReader};

use filesystem::Filesystem;

const AVAILABLE_DISK_SPACE: u32 = 70_000_000;
const REQUIRED_DISK_SPACE: u32 = 30_000_000;

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        let filesystem_descriptor =
            File::open(file_path).expect("Failed to open filesystem_descriptor file");
        let mut filesystem_descriptor = BufReader::new(filesystem_descriptor);
        let mut filesystem = Filesystem::new();

        filesystem.build_with_descriptor(&mut filesystem_descriptor);

        let total_size_of_small_directories = filesystem.fold(0, &mut |sum, current_dir| {
            if current_dir.size() <= 100_000 {
                &*sum + current_dir.size()
            } else {
                *sum
            }
        });

        println!("Total size of directories that are at most 100'000 bytes big: {total_size_of_small_directories}");

        let unused_disk_space = AVAILABLE_DISK_SPACE - filesystem.size();
        let disk_space_that_must_be_freed = REQUIRED_DISK_SPACE - unused_disk_space;

        let total_size_of_directory_to_delete =
            filesystem.fold(u32::MAX, &mut |minimum, current_dir| {
                let size = current_dir.size();

                if size >= disk_space_that_must_be_freed && size < *minimum {
                    size
                } else {
                    *minimum
                }
            });

        println!(
            "Total size of directory that should be deleted: {total_size_of_directory_to_delete}"
        );
    } else {
        panic!("Did not provide path to filesystem_descriptor file");
    }
}
