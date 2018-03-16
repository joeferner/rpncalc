import {RpnCalculator} from "../RpnCalculator";
import {UnaryOperator} from "./UnaryOperator";
import {StackItem} from "../StackItem";
import {DecimalStackItem} from "../DecimalStackItem";

export class NegateOperator extends UnaryOperator {
    public getKeywords(): string[] {
        return ['neg', 'negate'];
    }

    protected async executeUnary(a: StackItem, rpnCalculator: RpnCalculator): Promise<DecimalStackItem> {
        const aVal = await a.decimalValue(rpnCalculator);
        const units = (a instanceof DecimalStackItem) ? a.units : null;
        return new DecimalStackItem(aVal.neg(), units);
    }
}
