import { Operation } from "./operation.js";

export class Monkey {
    /** @type {string} */
    name;
    /** @type {number[]} */
    worryLevelsForItems;
    /** @type {Operation} */
    reactionToInspection;
    /** @type {number} */
    throwRecipientDeterminator;
    /** @type {number} */
    throwRecipientA;
    /** @type {number} */
    throwRecipientB;
    /** @type {number} */
    numberOfInspections = 0;

    /** @param {string} description   */
    static parseDescription(description) {
        const monkey = new Monkey();
        const [name, startingItems, operation, test, trueCase, falseCase] = description.split("\n");

        monkey.name = name;

        // Parse items
        monkey.worryLevelsForItems = startingItems
            .trim()
            .substring("Starting items: ".length)
            .split(", ")
            .map(Number);

        // Parse operation
        monkey.reactionToInspection = new Operation(
            operation.trim().substring("Operation: new = ".length)
        );

        // Parse test
        monkey.throwRecipientDeterminator = Number(
            test.trim().substring("Test: divisible by ".length)
        );

        // Parse trueCase
        monkey.throwRecipientA = Number(
            trueCase.trim().substring("If true: throw to monkey ".length)
        );

        // Parse falseCase
        monkey.throwRecipientB = Number(
            falseCase.trim().substring("If false: throw to monkey ".length)
        );

        return monkey;
    }

    get hasItemsToThrow() {
        return this.worryLevelsForItems.length > 0;
    }

    inspectItem() {
        let newWorryLevel = this.worryLevelsForItems[0];

        globalThis.DEBUG &&
            console.log(`\tMonkey inspects an item with a worry level of ${newWorryLevel}.`);

        newWorryLevel = this.reactionToInspection.execute(newWorryLevel);

        globalThis.DEBUG && console.log(`\t\tWorry level grows to ${newWorryLevel}.`);

        newWorryLevel = Math.floor(newWorryLevel / 3);

        globalThis.DEBUG &&
            console.log(
                `\t\tMonkey gets bored with item. Worry level is divided by 3 to ${newWorryLevel}.`
            );

        this.worryLevelsForItems[0] = newWorryLevel;
        this.numberOfInspections++;
    }

    /** @param {number} worryLevel  */
    determineThrowRecipient(worryLevel) {
        if (worryLevel % this.throwRecipientDeterminator === 0) {
            globalThis.DEBUG &&
                console.log(
                    `\t\tCurrent worry level is divisible by ${this.throwRecipientDeterminator}.`
                );

            return this.throwRecipientA;
        }

        globalThis.DEBUG &&
            console.log(
                `\t\tCurrent worry level is not divisible by ${this.throwRecipientDeterminator}.`
            );

        return this.throwRecipientB;
    }

    throwItemToMonkey() {
        const item = this.worryLevelsForItems.shift();
        const recipient = this.determineThrowRecipient(item);

        globalThis.DEBUG &&
            console.log(`\t\tItem with worry level ${item} is thrown to monkey ${recipient}.`);

        return { item, recipient };
    }

    /** @param {number} item  */
    catchItem(item) {
        this.worryLevelsForItems.push(item);
    }

    toString() {
        return `${this.name} ${this.worryLevelsForItems}`;
    }
}
