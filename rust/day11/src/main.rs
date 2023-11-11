mod monkey;
mod operation;

use std::{collections::VecDeque, env, fs};

use crate::monkey::{Monkey, ThrowData};

const NUMBER_OF_ROUNDS: u32 = 10_000;
const RELIEF_REDUCES_WORRY: bool = false;

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        let monkey_descriptors =
            fs::read_to_string(file_path).expect("Failed to open monkey_descriptors file");
        let monkey_descriptors = monkey_descriptors.split("\n\n");
        let mut monkeys = monkey_descriptors.map(Monkey::new).collect::<Vec<Monkey>>();
        let mut thrown_items: VecDeque<ThrowData> = VecDeque::new();
        let worry_level_limit = monkeys
            .iter()
            .map(|monkey| monkey.throw_recipient_determinator())
            .product();

        for _ in 0..NUMBER_OF_ROUNDS {
            for idx in 0..monkeys.len() {
                #[cfg(feature = "debug")]
                println!("Monkey {idx}");

                let monkey = monkeys.get_mut(idx).unwrap();

                // We divide the round into "throw" and "catch" phases to prevent multiple
                // mutable borrow issues with the `monkeys` list.

                // Throw phase
                while monkey.has_items_left() {
                    monkey.inspect_item(RELIEF_REDUCES_WORRY);
                    thrown_items.push_back(monkey.throw_item_to_monkey(worry_level_limit).unwrap());
                }

                // Catch phase
                while let Some(data) = thrown_items.pop_front() {
                    let recipient = monkeys.get_mut(data.recipient).unwrap();

                    recipient.catch_item(data.item);
                }
            }
        }

        let monkeys = monkeys.as_mut_slice();

        monkeys.sort_unstable_by_key(|monkey| monkey.inspection_count());

        let monkey_business_level: u64 = monkeys
            .split_at(monkeys.len() - 2)
            .1
            .iter()
            .map(|monkey| monkey.inspection_count() as u64)
            .product();

        println!("Level of monkey business after {NUMBER_OF_ROUNDS} rounds of stuff-slinging simian shenanigans is: {monkey_business_level}");
    } else {
        panic!("Did not provide path to monkey_descriptors file");
    }
}
