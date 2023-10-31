use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn map_item_to_priority(item: char) -> u32 {
    match item {
        'a'..='z' => (item as u32) - ('a' as u32) + 1,
        'A'..='Z' => (item as u32) - ('A' as u32) + 27,
        _ => 0,
    }
}

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        let rucksacks = File::open(file_path).expect("Failed to open rucksacks file");
        let rucksacks = BufReader::new(rucksacks);
        let mut sum_of_priorities = 0;

        for rucksack in rucksacks.lines() {
            let rucksack = rucksack.unwrap();
            let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

            for item in first_compartment.chars() {
                if second_compartment.contains(item) {
                    sum_of_priorities += map_item_to_priority(item);
                    break;
                }
            }
        }

        println!("Sum of priorities: {sum_of_priorities}");
    } else {
        panic!("Did not provide path to rucksacks file");
    }
}
