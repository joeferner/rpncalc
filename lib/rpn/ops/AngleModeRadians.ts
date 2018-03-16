import {AngleMode, RpnCalculator} from "../RpnCalculator";
import {Operator} from "./Operator";

export class AngleModeRadians extends Operator {
    public getKeywords(): string[] {
        return ['rad', 'radians'];
    }

    public execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        return rpnCalculator.setOption(RpnCalculator.OPTION_ANGLE_MODE, AngleMode.Radians);
    }
}

