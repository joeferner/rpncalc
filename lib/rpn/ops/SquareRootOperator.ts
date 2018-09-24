import {RpnCalculator} from "../RpnCalculator";
import {UnaryOperator} from "./UnaryOperator";
import {StackItem} from "../StackItem";
import {Decimal} from "decimal.js";

export class SquareRootOperator extends UnaryOperator {
    public getKeywords(): string[] {
        return ['sqrt', 'squareRoot'];
    }

    protected async executeUnary(a: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal> {
        const aVal = await a.decimalValue(rpnCalculator);
        return aVal.sqrt();
    }
}
