import {RpnCalculator} from "../RpnCalculator";
import {Operator} from "./Operator";
import {ExpressionStackItem} from "../ExpressionStackItem";
import {DecimalStackItem} from "../DecimalStackItem";

export class ConvertOperator extends Operator {
    public getKeywords(): string[] {
        return ['convert'];
    }

    public async execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        if (rpnCalculator.size() < 2) {
            throw new Error('Not enough operands');
        }
        let value = rpnCalculator.peek(1);
        if (!(value instanceof DecimalStackItem)) {
            throw new Error('1st argument must be a number');
        }
        let toUnits = <ExpressionStackItem>rpnCalculator.peek(0);
        if (!toUnits.expression) {
            throw new Error('2nd argument must be an expression');
        }

        let result = await rpnCalculator.convert(<DecimalStackItem>value, toUnits.expression);
        rpnCalculator.pop(2);
        await rpnCalculator.push(result);
        return Promise.resolve();
    }
}

