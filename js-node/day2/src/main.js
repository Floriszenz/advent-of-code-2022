import { open } from "node:fs/promises";
import { argv } from "node:process";

const filePath = argv.at(2);
const file = await open(filePath);

let totalScore = 0;

for await (const line of file.readLines()) {
  const [opponentsChoice, myChoice] = line.split(" ");

  // Get score for chosen shape
  switch (myChoice) {
    case "X":
      totalScore += 1;
      break;

    case "Y":
      totalScore += 2;
      break;

    case "Z":
      totalScore += 3;
      break;

    default:
      throw new Error(`You have chosen an invalid shape: ${myChoice}`);
      break;
  }

  // Determine outcome of the round
  const iAmWinning =
    (myChoice === "X" && opponentsChoice === "C") ||
    (myChoice === "Y" && opponentsChoice === "A") ||
    (myChoice === "Z" && opponentsChoice === "B");
  const itIsDraw =
    (myChoice === "X" && opponentsChoice === "A") ||
    (myChoice === "Y" && opponentsChoice === "B") ||
    (myChoice === "Z" && opponentsChoice === "C");

  if (iAmWinning) {
    totalScore += 6;
  } else if (itIsDraw) {
    totalScore += 3;
  }
}

console.log(`Total score for the game is: ${totalScore}`);
