use std::{
    borrow::BorrowMut,
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        let rearrangement_instructions =
            File::open(file_path).expect("Failed to open rearrangement_instructions file");
        let mut rearrangement_instructions = BufReader::new(rearrangement_instructions);

        let mut stacks: HashMap<u32, Vec<char>> = HashMap::new();

        // Initialize stacks
        let stacks_layout = rearrangement_instructions
            .borrow_mut()
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<Vec<_>>();
        let stacks_layout = stacks_layout.iter().rev().skip(1);

        for crates in stacks_layout {
            for (idx, stack_crate) in crates.char_indices() {
                match stack_crate {
                    '[' | ']' | ' ' => {}
                    _ => {
                        stacks
                            .entry((idx as u32) / 4 + 1)
                            .and_modify(|stack| stack.push(stack_crate))
                            .or_insert(vec![stack_crate]);
                    }
                }
            }
        }

        // Rearrange
        for instruction in rearrangement_instructions.lines().map(|i| i.unwrap()) {
            let mut instruction = instruction.split_whitespace();
            let crates_to_move: u32 = instruction.nth(1).unwrap().parse().unwrap();
            let from_stack: u32 = instruction.nth(1).unwrap().parse().unwrap();
            let to_stack: u32 = instruction.nth(1).unwrap().parse().unwrap();

            for _ in 0..crates_to_move {
                if let Some(crate_to_move) = stacks.get_mut(&from_stack).unwrap().pop() {
                    stacks
                        .entry(to_stack)
                        .and_modify(|stack| stack.push(crate_to_move));
                }
            }
        }

        // Output
        let mut crates_at_top = String::new();

        for stack_id in 1..=stacks.len() {
            let crate_at_top = stacks.get(&(stack_id as u32)).unwrap().last().unwrap();

            crates_at_top.push(*crate_at_top);
        }

        println!("Crates at top: {crates_at_top}");
    } else {
        panic!("Did not provide path to rearrangement_instructions file");
    }
}
