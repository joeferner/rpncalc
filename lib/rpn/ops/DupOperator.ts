import {RpnCalculator} from "../RpnCalculator";
import {Operator} from "./Operator";

export class DupOperator extends Operator {
    public getKeywords(): string[] {
        return ['dup'];
    }

    public execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        if (rpnCalculator.size() < 1) {
            return Promise.reject(new Error('Not enough operands'));
        }
        let a = rpnCalculator.peek(0);
        rpnCalculator.push(a);
        return Promise.resolve();
    }
}
