use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Seek},
};

fn map_item_to_priority(item: char) -> u32 {
    match item {
        'a'..='z' => (item as u32) - ('a' as u32) + 1,
        'A'..='Z' => (item as u32) - ('A' as u32) + 27,
        _ => 0,
    }
}

fn get_sum_of_individual_priorities(rucksacks: &File) -> u32 {
    let rucksacks = BufReader::new(rucksacks);
    let mut sum_of_priorities = 0;

    for rucksack in rucksacks.lines().map(|r| r.unwrap()) {
        let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

        for item in first_compartment.chars() {
            if second_compartment.contains(item) {
                sum_of_priorities += map_item_to_priority(item);
                break;
            }
        }
    }

    sum_of_priorities
}

fn get_sum_of_group_priorities(rucksacks: &File) -> u32 {
    let rucksacks = BufReader::new(rucksacks);
    let mut rucksacks = rucksacks.lines().map(|r| r.unwrap());
    let mut sum_of_priorities = 0;

    while let (Some(first_rucksack), Some(second_rucksack), Some(third_rucksack)) =
        (rucksacks.next(), rucksacks.next(), rucksacks.next())
    {
        for item in first_rucksack.chars() {
            if second_rucksack.contains(item) && third_rucksack.contains(item) {
                sum_of_priorities += map_item_to_priority(item);
                break;
            }
        }
    }

    sum_of_priorities
}

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        let mut rucksacks = File::open(file_path).expect("Failed to open rucksacks file");
        let sum_of_individual_priorities = get_sum_of_individual_priorities(&rucksacks);
        rucksacks.rewind().unwrap();
        let sum_of_group_priorities = get_sum_of_group_priorities(&rucksacks);

        println!("Sum of individual priorities: {sum_of_individual_priorities}");
        println!("Sum of group priorities: {sum_of_group_priorities}");
    } else {
        panic!("Did not provide path to rucksacks file");
    }
}
