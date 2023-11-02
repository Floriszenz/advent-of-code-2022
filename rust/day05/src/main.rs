use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Seek, SeekFrom},
};

type ContainerTerminal = HashMap<u32, Vec<char>>;

fn initialize_stacks(stacks_layout: &mut dyn BufRead) -> ContainerTerminal {
    let mut stacks: ContainerTerminal = HashMap::new();
    let stacks_layout = stacks_layout
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let stacks_layout = stacks_layout.iter().rev().skip(1);

    for crates in stacks_layout {
        for (idx, stack_crate) in crates.char_indices() {
            match stack_crate {
                '[' | ']' | ' ' => { /* ignore characters */ }
                _ => {
                    stacks
                        .entry((idx as u32) / 4 + 1)
                        .and_modify(|stack| stack.push(stack_crate))
                        .or_insert(vec![stack_crate]);
                }
            }
        }
    }

    stacks
}

fn rearrange_with_crate_mover_9000(stacks: &mut ContainerTerminal, instructions: &mut dyn BufRead) {
    for instruction in instructions.lines().map(|i| i.unwrap()) {
        let mut instruction = instruction.split_whitespace();
        let crates_to_move: u32 = instruction.nth(1).and_then(|i| i.parse().ok()).unwrap();
        let from_stack: u32 = instruction.nth(1).and_then(|i| i.parse().ok()).unwrap();
        let to_stack: u32 = instruction.nth(1).and_then(|i| i.parse().ok()).unwrap();

        for _ in 0..crates_to_move {
            if let Some(crate_to_move) = stacks.get_mut(&from_stack).unwrap().pop() {
                stacks
                    .entry(to_stack)
                    .and_modify(|stack| stack.push(crate_to_move));
            }
        }
    }
}

fn rearrange_with_crate_mover_9001(stacks: &mut ContainerTerminal, instructions: &mut dyn BufRead) {
    for instruction in instructions.lines().map(|i| i.unwrap()) {
        let mut instruction = instruction.split_whitespace();
        let crates_to_move: usize = instruction.nth(1).and_then(|i| i.parse().ok()).unwrap();
        let from_stack: u32 = instruction.nth(1).and_then(|i| i.parse().ok()).unwrap();
        let to_stack: u32 = instruction.nth(1).and_then(|i| i.parse().ok()).unwrap();

        let from_stack = stacks.get_mut(&from_stack).unwrap();
        let mut crates_to_move = from_stack.split_off(from_stack.len() - crates_to_move);

        stacks
            .entry(to_stack)
            .and_modify(|stack| stack.append(&mut crates_to_move));
    }
}

fn get_crates_at_top(stacks: &mut ContainerTerminal) -> String {
    let mut crates_at_top = String::new();

    for stack_id in 1..=stacks.len() {
        let crate_at_top = stacks
            .get(&(stack_id as u32))
            .and_then(|crates| crates.last())
            .unwrap();
        crates_at_top.push(*crate_at_top);
    }

    crates_at_top
}

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        let rearrangement_instructions =
            File::open(file_path).expect("Failed to open rearrangement_instructions file");
        let mut rearrangement_instructions = BufReader::new(rearrangement_instructions);
        let mut stacks = initialize_stacks(&mut rearrangement_instructions);
        let buf_position = rearrangement_instructions.stream_position().unwrap();

        {
            let mut stacks = stacks.clone();

            rearrange_with_crate_mover_9000(&mut stacks, &mut rearrangement_instructions);

            let crates_at_top = get_crates_at_top(&mut stacks);

            println!("Crates at top (CrateMover9000): {crates_at_top}");
        }

        rearrangement_instructions
            .seek(SeekFrom::Start(buf_position))
            .unwrap();

        {
            rearrange_with_crate_mover_9001(&mut stacks, &mut rearrangement_instructions);

            let crates_at_top = get_crates_at_top(&mut stacks);

            println!("Crates at top (CrateMover9001): {crates_at_top}");
        }
    } else {
        panic!("Did not provide path to rearrangement_instructions file");
    }
}
