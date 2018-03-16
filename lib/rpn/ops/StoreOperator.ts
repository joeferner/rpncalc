import {RpnCalculator} from "../RpnCalculator";
import {Operator} from "./Operator";
import {ExpressionStackItem} from "../ExpressionStackItem";

export class StoreOperator extends Operator {
    public getKeywords(): string[] {
        return ['store', 'sto'];
    }

    public execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        if (rpnCalculator.size() < 2) {
            return Promise.reject(new Error('Not enough operands'));
        }
        let value = rpnCalculator.peek(1);
        let name = <ExpressionStackItem>rpnCalculator.peek(0);
        if (!name.expression) {
            return Promise.reject(new Error('2nd argument must be an expression'));
        }
        try {
            rpnCalculator.store(name.expression, value);
        } catch (err) {
            return Promise.reject(err);
        }
        rpnCalculator.pop(2);
        return Promise.resolve();
    }
}

