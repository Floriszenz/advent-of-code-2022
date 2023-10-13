import { open } from "node:fs/promises";
import { argv } from "node:process";

const filePath = argv.at(2);
const input = await open(filePath);

const stackInput = [];
let processInstructions = false;
/** @type {Record<string, string[]>} */
const stackPlace = {};

for await (const line of input.readLines()) {
  if (line.trim() === "") {
    processInstructions = true;

    // Add stack ids to stack place
    const stackIds = stackInput.pop().trim().split("   ");

    for (const id of stackIds) {
      stackPlace[id] = [];
    }

    // Add crates to stacks
    for (const procedure of stackInput.reverse()) {
      for (let index = 1; index <= Object.keys(stackPlace).length; index++) {
        const crate = procedure[4 * (index - 1) + 1];

        if (crate != " ") {
          stackPlace[`${index}`].push(crate);
        }
      }
    }

    continue;
  }

  if (processInstructions) {
    // Parse info from procedure
    const { crateCount, from, to } =
      /^move (?<crateCount>\d+) from (?<from>\d+) to (?<to>\d+)$/.exec(
        line,
      ).groups;

    for (let index = 0; index < Number(crateCount); index++) {
      const crateToMove = stackPlace[from].pop();

      stackPlace[to].push(crateToMove);
    }
  } else {
    stackInput.push(line);
  }
}

await input.close();

// Output highest crate on each stack
let output = "";

for (let index = 1; index <= Object.keys(stackPlace).length; index++) {
  const highestCrateOfStack = stackPlace[`${index}`].at(-1);

  output += highestCrateOfStack;
}

console.log(`Crates on top of each stack after rearrangement: "${output}"`);
