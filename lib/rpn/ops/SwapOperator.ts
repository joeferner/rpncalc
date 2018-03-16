import {RpnCalculator} from "../RpnCalculator";
import {Operator} from "./Operator";

export class SwapOperator extends Operator {
    public getKeywords(): string[] {
        return ['swap'];
    }

    public async execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        if (rpnCalculator.size() < 2) {
            throw new Error('Not enough operands');
        }
        let a = rpnCalculator.peek(1);
        let b = rpnCalculator.peek(0);
        rpnCalculator.pop(2);
        await rpnCalculator.push([b, a]);
        return Promise.resolve();
    }
}

