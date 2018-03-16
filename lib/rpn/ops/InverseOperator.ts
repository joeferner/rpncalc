import {RpnCalculator} from "../RpnCalculator";
import {UnaryOperator} from "./UnaryOperator";
import {StackItem} from "../StackItem";
import {Decimal} from "decimal.js";

export class InverseOperator extends UnaryOperator {
    public getKeywords(): string[] {
        return ['inv', 'inverse'];
    }

    protected async executeUnary(a: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal> {
        const aVal = await a.decimalValue(rpnCalculator);
        if (aVal.toNumber() === 0.0) {
            throw new Error('Divide by 0');
        }
        return new Decimal(1).dividedBy(aVal);
    }
}
