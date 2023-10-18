export class Operation {
    /** @typedef {"old" | number} Operand */

    /** @type {Operand} */
    operandA;
    /** @type {Operand} */
    operandB;
    /** @type {(a: number, b: number) => number} */
    operator;

    /** @param {string} formula  */
    constructor(formula) {
        const [operandA, operator, operandB] = formula.split(" ");

        this.operandA = operandA === "old" ? "old" : Number(operandA);
        this.operandB = operandB === "old" ? "old" : Number(operandB);

        switch (operator) {
            case "+":
                this.operator = (a, b) => a + b;
                break;

            case "*":
                this.operator = (a, b) => a * b;
                break;

            default:
                throw new Error(`Invalid operation in formula: ${operator}`);
        }
    }

    /** @param {number} oldValue */
    execute(oldValue) {
        return this.operator(
            this.operandA === "old" ? oldValue : this.operandA,
            this.operandB === "old" ? oldValue : this.operandB
        );
    }
}
