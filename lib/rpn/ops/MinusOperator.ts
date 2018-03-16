import {RpnCalculator} from "../RpnCalculator";
import {BinaryOperator} from "./BinaryOperator";
import {StackItem} from "../StackItem";
import {DecimalStackItem} from "../DecimalStackItem";

export class MinusOperator extends BinaryOperator {
    public getKeywords(): string[] {
        return ['minus', 'subtract', 'sub', '-'];
    }

    protected async executeBinary(a: StackItem, b: StackItem, rpnCalculator: RpnCalculator): Promise<DecimalStackItem> {
        const common = await rpnCalculator.convertToCommonDecimalValues(a, b);
        return new DecimalStackItem(common.values[0].minus(common.values[1]), common.units);
    }
}
