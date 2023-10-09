import { open } from "node:fs/promises";
import { argv } from "node:process";

const filePath = argv.at(2);

const rucksackList = await open(filePath);
let sumOfPriorities = 0;

for await (const rucksack of rucksackList.readLines()) {
  const firstCompartment = rucksack.substring(0, rucksack.length / 2);
  const secondCompartment = rucksack.substring(rucksack.length / 2);

  // Find common item
  let commonItem;

  firstHalfLoop: for (const firstItem of firstCompartment) {
    for (const secondItem of secondCompartment) {
      if (firstItem === secondItem) {
        commonItem = firstItem;
        break firstHalfLoop;
      }
    }
  }

  if (!commonItem) {
    throw new Error(`Couldn't find any common item in rucksack "${rucksack}"`);
  }

  const commonCharCode = commonItem.charCodeAt(0);

  // Map commonItem to priority
  if (commonCharCode >= 97 && commonCharCode <= 122) {
    sumOfPriorities += commonCharCode - 97 + 1;
  } else if (commonCharCode >= 65 && commonCharCode <= 90) {
    sumOfPriorities += commonCharCode - 65 + 27;
  }
}

console.log(`Sum of all priorities: ${sumOfPriorities}`);
await rucksackList.close();
