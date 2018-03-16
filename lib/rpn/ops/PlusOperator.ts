import {RpnCalculator} from "../RpnCalculator";
import {BinaryOperator} from "./BinaryOperator";
import {StackItem} from "../StackItem";
import {DecimalStackItem} from "../DecimalStackItem";

export class PlusOperator extends BinaryOperator {
    public getKeywords(): string[] {
        return ['add', 'plus', '+'];
    }

    protected async executeBinary(a: StackItem, b: StackItem, rpnCalculator: RpnCalculator): Promise<StackItem> {
        const common = await rpnCalculator.convertToCommonDecimalValues(a, b);
        return new DecimalStackItem(common.values[0].plus(common.values[1]), common.units);
    }
}
