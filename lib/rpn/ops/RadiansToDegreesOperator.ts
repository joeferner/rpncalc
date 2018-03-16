import {RpnCalculator} from "../RpnCalculator";
import {UnaryOperator} from "./UnaryOperator";
import {StackItem} from "../StackItem";
import {PiOperator} from "./PiOperator";

export class RadiansToDegreesOperator extends UnaryOperator {
    public getKeywords(): string[] {
        return ['rad2deg'];
    }

    protected async executeUnary(a: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal> {
        const aVal = await a.decimalValue(rpnCalculator);
        return aVal.times(180.0).dividedBy(PiOperator.PI);
    }
}
