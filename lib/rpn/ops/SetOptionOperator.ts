import {Operator} from "./Operator";
import {RpnCalculator} from "../RpnCalculator";
import {ExpressionStackItem} from "../ExpressionStackItem";

export class SetOptionOperator extends Operator {
    public getKeywords(): string[] {
        return ['set'];
    }

    public execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        if (rpnCalculator.size() < 2) {
            return Promise.reject(new Error('Not enough operands'));
        }
        let nameStackItem = rpnCalculator.peek(1);
        if (!(nameStackItem instanceof ExpressionStackItem)) {
            return Promise.reject(new Error('First operand must be an expression'));
        }
        let name = (<ExpressionStackItem>nameStackItem).expression;
        let value = rpnCalculator.peek(0).value;
        return rpnCalculator.setOption(name, value)
            .then(() => {
                rpnCalculator.pop(2);
            });
    }
}
