import {AngleMode, RpnCalculator} from "../RpnCalculator";
import {UnaryOperator} from "./UnaryOperator";
import {PiOperator} from "./PiOperator";
import {StackItem} from "../StackItem";
import {Decimal} from "decimal.js";

export abstract class AngleOutputUnaryOperator extends UnaryOperator {
    protected async executeUnary(a: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal> {
        const aVal = await a.decimalValue(rpnCalculator);
        return AngleOutputUnaryOperator.fromRadians(this.executeUnaryAngleResult(aVal, rpnCalculator), rpnCalculator);
    }

    protected abstract executeUnaryAngleResult(a: Decimal, rpnCalculator: RpnCalculator): Decimal;

    public static fromRadians(angleInRadians: Decimal, rpnCalculator: RpnCalculator): Decimal {
        switch (rpnCalculator.getOption(RpnCalculator.OPTION_ANGLE_MODE)) {
            case AngleMode.Radians:
                return angleInRadians;
            case AngleMode.Degrees:
                return angleInRadians.times(180.0).dividedBy(PiOperator.PI);
            default:
                throw new Error(`unhandled angle mode: ${rpnCalculator.getOption(RpnCalculator.OPTION_ANGLE_MODE)}`);
        }
    }
}
