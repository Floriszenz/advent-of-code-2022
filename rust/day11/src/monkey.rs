use std::collections::VecDeque;

use crate::operation::Operation;

pub struct ThrowData {
    pub item: u32,
    pub recipient: usize,
}

#[derive(Debug)]
pub struct Monkey {
    worry_levels_for_items: VecDeque<u32>,
    reaction_to_inspection: Operation,
    throw_recipient_determinator: u32,
    throw_recipient_a: usize,
    throw_recipient_b: usize,
    number_of_inspections: u32,
}

impl Monkey {
    pub fn new(descriptor: &str) -> Self {
        let mut descriptor = descriptor.split("\n");
        let _name = descriptor.next().unwrap();
        let worry_levels_for_items = descriptor
            .next()
            .and_then(|line| {
                line.trim()
                    .strip_prefix("Starting items: ")
                    .unwrap()
                    .split(", ")
                    .map(|item| item.parse::<u32>().ok())
                    .collect::<Option<VecDeque<_>>>()
            })
            .unwrap();

        let reaction_to_inspection = descriptor
            .next()
            .and_then(|line| {
                line.trim()
                    .strip_prefix("Operation: new = ")
                    .map(Operation::new)
            })
            .unwrap();

        let throw_recipient_determinator = descriptor
            .next()
            .and_then(|line| {
                line.trim()
                    .strip_prefix("Test: divisible by ")
                    .and_then(|det| det.parse::<u32>().ok())
            })
            .unwrap();

        let throw_recipient_a = descriptor
            .next()
            .and_then(|line| {
                line.trim()
                    .strip_prefix("If true: throw to monkey ")
                    .and_then(|a| a.parse::<usize>().ok())
            })
            .unwrap();

        let throw_recipient_b = descriptor
            .next()
            .and_then(|line| {
                line.trim()
                    .strip_prefix("If false: throw to monkey ")
                    .and_then(|b| b.parse::<usize>().ok())
            })
            .unwrap();

        Self {
            worry_levels_for_items,
            reaction_to_inspection,
            throw_recipient_determinator,
            throw_recipient_a,
            throw_recipient_b,
            number_of_inspections: 0,
        }
    }

    pub fn has_items_left(&self) -> bool {
        !self.worry_levels_for_items.is_empty()
    }

    pub fn inspect_item(&mut self) {
        if let Some(current_worry_level) = self.worry_levels_for_items.get_mut(0) {
            #[cfg(feature = "debug")]
            println!("  Monkey inspects an item with a worry level of {current_worry_level}.");

            let new_worry_level = self.reaction_to_inspection.execute(*current_worry_level);

            #[cfg(feature = "debug")]
            println!("    Worry level grows to {new_worry_level}.");

            let new_worry_level = new_worry_level / 3;

            #[cfg(feature = "debug")]
            println!("    Monkey gets bored with item. Worry level is divided by 3 to {new_worry_level}.");

            *current_worry_level = new_worry_level;
            self.number_of_inspections += 1;
        }
    }

    pub fn throw_item_to_monkey(&mut self) -> Option<ThrowData> {
        self.worry_levels_for_items.pop_front().map(|item| {
            let recipient = if item % self.throw_recipient_determinator == 0 {
                #[cfg(feature = "debug")]
                println!(
                    "    Current worry level is divisible by {}.",
                    self.throw_recipient_determinator
                );
                self.throw_recipient_a
            } else {
                #[cfg(feature = "debug")]
                println!(
                    "    Current worry level is not divisible by {}.",
                    self.throw_recipient_determinator
                );
                self.throw_recipient_b
            };

            #[cfg(feature = "debug")]
            println!("    Item with worry level {item} is thrown to monkey {recipient}.");

            ThrowData { item, recipient }
        })
    }

    pub fn catch_item(&mut self, item: u32) {
        self.worry_levels_for_items.push_back(item);
    }

    pub fn inspection_count(&self) -> u32 {
        self.number_of_inspections
    }
}
