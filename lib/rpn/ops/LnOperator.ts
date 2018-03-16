import {RpnCalculator} from "../RpnCalculator";
import {UnaryOperator} from "./UnaryOperator";
import {StackItem} from "../StackItem";

export class LnOperator extends UnaryOperator {
    public getKeywords(): string[] {
        return ['ln'];
    }

    protected async executeUnary(a: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal> {
        const aVal = await a.decimalValue(rpnCalculator);
        return aVal.ln();
    }
}
