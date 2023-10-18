import { readFile } from "node:fs/promises";
import { argv } from "node:process";

import { Monkey } from "./monkey.js";

globalThis.DEBUG = false;

const filePath = argv.at(2);
/** @type {string} */
const inputFile = await readFile(filePath, { encoding: "utf8" });
const monkeys = inputFile.split("\n\n").map((description) => Monkey.parseDescription(description));
const numberOfRounds = 20;

for (let round = 0; round < numberOfRounds; round++) {
    for (const monkey of monkeys) {
        globalThis.DEBUG && console.log(monkey.name);

        while (monkey.hasItemsToThrow) {
            monkey.inspectItem();

            const { item, recipient } = monkey.throwItemToMonkey();

            monkeys[recipient].catchItem(item);
        }
    }
}

globalThis.DEBUG && console.log(monkeys.map((monkey) => monkey.toString()).join("\n"));

const mostActiveMonkeys = monkeys
    .map((monkey) => monkey.numberOfInspections)
    .sort((a, b) => b - a)
    .slice(0, 2);
const monkeyBusinessLevel = mostActiveMonkeys[0] * mostActiveMonkeys[1];

console.log(
    `Level of monkey business after ${numberOfRounds} rounds of stuff-slinging simian shenanigans is: ${monkeyBusinessLevel}`
);
