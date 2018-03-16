import {AngleMode, RpnCalculator} from "../RpnCalculator";
import {UnaryOperator} from "./UnaryOperator";
import {PiOperator} from "./PiOperator";
import {StackItem} from "../StackItem";

export abstract class AngleInputUnaryOperator extends UnaryOperator {
    protected async executeUnary(a: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal> {
        const aVal = await a.decimalValue(rpnCalculator);
        const angleInRadians = AngleInputUnaryOperator.toRadians(aVal, rpnCalculator);
        return this.executeUnaryOnAngle(angleInRadians, rpnCalculator);
    }

    protected abstract executeUnaryOnAngle(angleInRadians: Decimal, rpnCalculator: RpnCalculator): Decimal;

    public static toRadians(a: Decimal, rpnCalculator: RpnCalculator): Decimal {
        switch (rpnCalculator.getOption(RpnCalculator.OPTION_ANGLE_MODE)) {
            case AngleMode.Radians:
                return a;
            case AngleMode.Degrees:
                return a.times(PiOperator.PI).dividedBy(180.0);
            default:
                throw new Error(`unhandled angle mode: ${rpnCalculator.getOption(RpnCalculator.OPTION_ANGLE_MODE)}`);
        }
    }
}
