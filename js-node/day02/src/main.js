import { open } from "node:fs/promises";
import { argv } from "node:process";

const OpponentsChoice = {
  Rock: "A",
  Paper: "B",
  Scissors: "C",
};
const MyChoice = {
  Rock: "X",
  Paper: "Y",
  Scissors: "Z",
};
const OutcomeOfRound = {
  Lose: "X",
  Draw: "Y",
  Win: "Z",
};

/**
 * @param {MyChoice[keyof typeof MyChoice]} shape
 */
function getScoreForShape(shape) {
  switch (shape) {
    case MyChoice.Rock:
      return 1;

    case MyChoice.Paper:
      return 2;

    case MyChoice.Scissors:
      return 3;

    default:
      throw new Error(`You have chosen an invalid shape: ${shape}`);
  }
}

/**
 * @param {MyChoice[keyof typeof MyChoice]} myChoice
 * @param {OpponentsChoice[keyof typeof OpponentsChoice]} opponentsChoice
 */
function determineOutcomeOfRound(myChoice, opponentsChoice) {
  const iAmWinning =
    (myChoice === MyChoice.Rock &&
      opponentsChoice === OpponentsChoice.Scissors) ||
    (myChoice === MyChoice.Paper && opponentsChoice === OpponentsChoice.Rock) ||
    (myChoice === MyChoice.Scissors &&
      opponentsChoice === OpponentsChoice.Paper);

  if (iAmWinning) {
    return OutcomeOfRound.Win;
  }

  const itIsDraw =
    (myChoice === MyChoice.Rock && opponentsChoice === OpponentsChoice.Rock) ||
    (myChoice === MyChoice.Paper &&
      opponentsChoice === OpponentsChoice.Paper) ||
    (myChoice === MyChoice.Scissors &&
      opponentsChoice === OpponentsChoice.Scissors);

  if (itIsDraw) {
    return OutcomeOfRound.Draw;
  }

  return OutcomeOfRound.Lose;
}

/**
 * @param {OutcomeOfRound[keyof typeof OutcomeOfRound]} desiredOutcome
 * @param {OpponentsChoice[keyof typeof OpponentsChoice]} opponentsChoice
 */
function determineChoiceForDesiredOutcome(desiredOutcome, opponentsChoice) {
  if (desiredOutcome === OutcomeOfRound.Win) {
    switch (opponentsChoice) {
      case OpponentsChoice.Rock:
        return MyChoice.Paper;

      case OpponentsChoice.Paper:
        return MyChoice.Scissors;

      case OpponentsChoice.Scissors:
        return MyChoice.Rock;
    }
  } else if (desiredOutcome === OutcomeOfRound.Draw) {
    switch (opponentsChoice) {
      case OpponentsChoice.Rock:
        return MyChoice.Rock;

      case OpponentsChoice.Paper:
        return MyChoice.Paper;

      case OpponentsChoice.Scissors:
        return MyChoice.Scissors;
    }
  } else if (desiredOutcome === OutcomeOfRound.Lose) {
    switch (opponentsChoice) {
      case OpponentsChoice.Rock:
        return MyChoice.Scissors;

      case OpponentsChoice.Paper:
        return MyChoice.Rock;

      case OpponentsChoice.Scissors:
        return MyChoice.Paper;
    }
  }
}

/**
 * @param {string} filePath
 */
async function calculateAssumedTotalScore(filePath) {
  const strategyGuide = await open(filePath);
  let totalScore = 0;

  for await (const line of strategyGuide.readLines({ start: 0 })) {
    const [opponentsChoice, myChoice] = line.split(" ");

    totalScore += getScoreForShape(myChoice);

    const outcomeOfRound = determineOutcomeOfRound(myChoice, opponentsChoice);

    if (outcomeOfRound === OutcomeOfRound.Win) {
      totalScore += 6;
    } else if (outcomeOfRound === OutcomeOfRound.Draw) {
      totalScore += 3;
    }
  }

  await strategyGuide.close();

  return totalScore;
}

/**
 * @param {string} filePath
 */
async function calculateActualTotalScore(filePath) {
  const strategyGuide = await open(filePath);
  let totalScore = 0;

  for await (const line of strategyGuide.readLines({ start: 0 })) {
    const [opponentsChoice, desiredOutcome] = line.split(" ");
    const myChoice = determineChoiceForDesiredOutcome(
      desiredOutcome,
      opponentsChoice,
    );

    totalScore += getScoreForShape(myChoice);

    if (desiredOutcome === OutcomeOfRound.Win) {
      totalScore += 6;
    } else if (desiredOutcome === OutcomeOfRound.Draw) {
      totalScore += 3;
    }
  }

  await strategyGuide.close();

  return totalScore;
}

const filePath = argv.at(2);

const assumedTotalScore = await calculateAssumedTotalScore(filePath);
const actualTotalScore = await calculateActualTotalScore(filePath);

console.log(`Assumed total score for the game is: ${assumedTotalScore}`);
console.log(`Actual total score for the game is: ${actualTotalScore}`);
