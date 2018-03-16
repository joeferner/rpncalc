import {Operator} from "./Operator";
import {RpnCalculator} from "../RpnCalculator";
import {ExpressionStackItem} from "../ExpressionStackItem";
import {isUndefined} from "util";

export class GetOptionOperator extends Operator {
    public getKeywords(): string[] {
        return ['get'];
    }

    public execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        if (rpnCalculator.size() < 1) {
            return Promise.reject(new Error('Not enough operands'));
        }
        let nameStackItem = rpnCalculator.peek(0);
        if (!(nameStackItem instanceof ExpressionStackItem)) {
            return Promise.reject(new Error('First operand must be an expression'));
        }
        let name = (<ExpressionStackItem>nameStackItem).expression;
        const value = rpnCalculator.getOption(name);
        if (isUndefined(value)) {
            return Promise.reject(new Error('Option not found'));
        }
        rpnCalculator.pop(1);
        rpnCalculator.push(value);
        return Promise.resolve();
    }
}
