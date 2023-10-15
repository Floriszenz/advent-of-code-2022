import { open } from "node:fs/promises";
import { argv } from "node:process";

const filePath = argv.at(2);

let stacksDescriptor = [];
let isBuildingStacksDone = false;
/** @type {Record<string, string[]>} */
let containerTerminal = {};

function resetGlobalState() {
  stacksDescriptor = [];
  isBuildingStacksDone = false;
  containerTerminal = {};
}

function initializeStacks() {
  const stackIds = stacksDescriptor.pop().trim().split("   ");

  for (const id of stackIds) {
    containerTerminal[id] = [];
  }
}

function moveCratesToInitialStackPosition() {
  for (const layerOfCargo of stacksDescriptor.reverse()) {
    for (
      let index = 1;
      index <= Object.keys(containerTerminal).length;
      index++
    ) {
      const crate = layerOfCargo[4 * (index - 1) + 1];

      if (crate != " ") {
        containerTerminal[`${index}`].push(crate);
      }
    }
  }
}

function buildInitialStacks() {
  initializeStacks();
  moveCratesToInitialStackPosition();
  isBuildingStacksDone = true;
}

/** @typedef {{amount: number; from: string; to: string}} RearrangementProcedure */

/**
 * @param {string} procedure
 * @returns {RearrangementProcedure}
 */
function parseRearrangementProcedure(procedure) {
  const { amount, from, to } =
    /^move (?<amount>\d+) from (?<from>\d+) to (?<to>\d+)$/.exec(
      procedure,
    ).groups;

  return {
    amount: Number(amount),
    from,
    to,
  };
}

/**
 * @param {RearrangementProcedure} procedure
 */
function rearrangeWithCrateMover9000({ amount, from, to }) {
  for (let index = 0; index < amount; index++) {
    const crateToMove = containerTerminal[from].pop();

    containerTerminal[to].push(crateToMove);
  }
}

/**
 * @param {RearrangementProcedure} procedure
 */
function rearrangeWithCrateMover9001({ amount, from, to }) {
  const cratesToMove = containerTerminal[from].splice(
    containerTerminal[from].length - amount,
    amount,
  );

  containerTerminal[to].push(...cratesToMove);
}

function getCratesOnTopOfEachStack() {
  let output = "";

  for (let index = 1; index <= Object.keys(containerTerminal).length; index++) {
    const topCrateOfStack = containerTerminal[`${index}`].at(-1);

    output += topCrateOfStack;
  }

  return output;
}

/**
 * @param {(RearrangementProcedure) => void} rearrangementStrategy
 */
async function runRearrangementSimulation(rearrangementStrategy) {
  resetGlobalState();

  const input = await open(filePath);

  for await (const line of input.readLines()) {
    if (line.trim() === "") {
      buildInitialStacks();
      continue;
    }

    if (isBuildingStacksDone) {
      const procedure = parseRearrangementProcedure(line);

      rearrangementStrategy(procedure);
    } else {
      stacksDescriptor.push(line);
    }
  }

  await input.close();

  return getCratesOnTopOfEachStack();
}

const cratesOnTopWithCrateMover9000 = await runRearrangementSimulation(
  rearrangeWithCrateMover9000,
);
const cratesOnTopWithCrateMover9001 = await runRearrangementSimulation(
  rearrangeWithCrateMover9001,
);

console.log(
  `Crates on top of each stack after rearrangement with CrateMover9000: "${cratesOnTopWithCrateMover9000}"`,
);
console.log(
  `Crates on top of each stack after rearrangement with CrateMover9001: "${cratesOnTopWithCrateMover9001}"`,
);
