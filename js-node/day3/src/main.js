import { open } from "node:fs/promises";
import { argv } from "node:process";

/**
 * @param {string} rucksack
 */
function findCommonItemInRucksack(rucksack) {
  const firstCompartment = rucksack.substring(0, rucksack.length / 2);
  const secondCompartment = rucksack.substring(rucksack.length / 2);

  /** @type {string} */
  let commonItem;

  firstHalfLoop: for (const firstItem of firstCompartment) {
    for (const secondItem of secondCompartment) {
      if (firstItem === secondItem) {
        commonItem = firstItem;
        break firstHalfLoop;
      }
    }
  }

  return commonItem;
}

/**
 * @param {string} item
 */
function mapItemToPriority(item) {
  const charCode = item.charCodeAt(0);
  const a = 97,
    z = 122,
    A = 65,
    Z = 90;

  if (charCode >= a && charCode <= z) {
    return charCode - a + 1;
  } else if (charCode >= A && charCode <= Z) {
    return charCode - A + 27;
  }
}

/**
 * @param {string} filePath
 */
async function calculateSumOfIndividualPriorities(filePath) {
  const rucksackList = await open(filePath);
  let sumOfPriorities = 0;

  for await (const rucksack of rucksackList.readLines()) {
    const commonItem = findCommonItemInRucksack(rucksack);

    if (!commonItem) {
      throw new Error(
        `Couldn't find any common item in rucksack "${rucksack}"`,
      );
    }

    sumOfPriorities += mapItemToPriority(commonItem);
  }

  await rucksackList.close();

  return sumOfPriorities;
}

const filePath = argv.at(2);

let sumOfIndividualPriorities =
  await calculateSumOfIndividualPriorities(filePath);

console.log(`Sum of individual priorities: ${sumOfIndividualPriorities}`);
