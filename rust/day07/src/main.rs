mod filesystem;
mod filesystem_entry;

use std::{env, fs::File, io::BufReader};

use filesystem::Filesystem;

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
    } else {
        panic!("Did not provide path to filesystem_descriptor file");
    }
}
