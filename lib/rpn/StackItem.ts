import {Decimal} from "decimal.js";
import {RpnCalculator} from "./RpnCalculator";

export abstract class StackItem {
    public abstract toString(rpnCalculator: RpnCalculator): string;

    public abstract decimalValue(rpnCalculator: RpnCalculator): Promise<Decimal>;

    public abstract get value(): any;

    public abstract save(): any;
}
