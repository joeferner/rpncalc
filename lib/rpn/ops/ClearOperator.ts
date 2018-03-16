import {RpnCalculator} from "../RpnCalculator";
import {Operator} from "./Operator";

export class ClearOperator extends Operator {
    public getKeywords(): string[] {
        return ['clr', 'clear'];
    }

    public execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        if (rpnCalculator.size() > 0) {
            rpnCalculator.pop(rpnCalculator.size());
        }
        return Promise.resolve();
    }
}
