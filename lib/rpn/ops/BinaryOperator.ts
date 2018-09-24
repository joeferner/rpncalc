import {Operator} from "./Operator";
import {RpnCalculator} from "../RpnCalculator";
import {StackItem} from "../StackItem";
import {Decimal} from "decimal.js";

export abstract class BinaryOperator extends Operator {
    public async execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        if (rpnCalculator.size() < 2) {
            return Promise.reject(new Error('Not enough operands'));
        }
        let a = await rpnCalculator.peek(1);
        let b = await rpnCalculator.peek(0);
        const result = await this.executeBinary(a, b, rpnCalculator);
        rpnCalculator.pop(2);
        return rpnCalculator.push(result);
    }

    protected abstract executeBinary(a: StackItem, b: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal | StackItem>;
}
