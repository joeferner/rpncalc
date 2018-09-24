import {RpnCalculator} from "../RpnCalculator";
import {UnaryOperator} from "./UnaryOperator";
import {StackItem} from "../StackItem";
import {Decimal} from "decimal.js";

export class Log2Operator extends UnaryOperator {
    public getKeywords(): string[] {
        return ['log2'];
    }

    protected async executeUnary(a: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal> {
        const aVal = await a.decimalValue(rpnCalculator);
        return aVal.log(2);
    }
}
