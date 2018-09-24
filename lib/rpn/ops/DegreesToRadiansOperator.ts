import {RpnCalculator} from "../RpnCalculator";
import {UnaryOperator} from "./UnaryOperator";
import {StackItem} from "../StackItem";
import {PiOperator} from "./PiOperator";
import {Decimal} from "decimal.js";

export class DegreesToRadiansOperator extends UnaryOperator {
    public getKeywords(): string[] {
        return ['deg2rad'];
    }

    protected async executeUnary(a: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal> {
        const aVal = await a.decimalValue(rpnCalculator);
        return aVal.times(PiOperator.PI).dividedBy(180.0);
    }
}
