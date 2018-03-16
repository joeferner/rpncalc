import {RpnCalculator} from "../RpnCalculator";

export abstract class Operator {
    public matches(input: string): boolean {
        for (let s of this.getKeywords()) {
            if (s === input.toLocaleLowerCase()) {
                return true;
            }
        }
        return false;
    }

    public abstract execute(rpnCalculator: RpnCalculator, input: string): Promise<void>;

    public abstract getKeywords(): string[];
}
