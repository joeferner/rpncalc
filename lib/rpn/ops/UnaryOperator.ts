import {RpnCalculator} from "../RpnCalculator";
import {Operator} from "./Operator";
import {StackItem} from "../StackItem";
import {DecimalStackItem} from "../DecimalStackItem";
import {Decimal} from "decimal.js";

export abstract class UnaryOperator extends Operator {
    public async execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        if (rpnCalculator.size() < 1) {
            return Promise.reject(new Error('Not enough operands'));
        }
        let a = await rpnCalculator.peek(0);
        const result = await this.executeUnary(a, rpnCalculator);
        rpnCalculator.pop(1);
        return rpnCalculator.push(result);
    }

    protected abstract executeUnary(a: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal | DecimalStackItem>;
}
