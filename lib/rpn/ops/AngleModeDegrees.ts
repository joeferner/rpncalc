import {AngleMode, RpnCalculator} from "../RpnCalculator";
import {Operator} from "./Operator";

export class AngleModeDegrees extends Operator {
    public getKeywords(): string[] {
        return ['deg', 'degrees'];
    }

    public execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        return rpnCalculator.setOption(RpnCalculator.OPTION_ANGLE_MODE, AngleMode.Degrees);
    }
}

