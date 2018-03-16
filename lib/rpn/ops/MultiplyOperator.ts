import {RpnCalculator} from "../RpnCalculator";
import {BinaryOperator} from "./BinaryOperator";
import {StackItem} from "../StackItem";
import {DecimalStackItem} from "../DecimalStackItem";
import Qty from "js-quantities";

export class MultiplyOperator extends BinaryOperator {
    public getKeywords(): string[] {
        return ['multiply', 'mul', 'times', '*'];
    }

    protected async executeBinary(a: StackItem, b: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal | DecimalStackItem> {
        const aVal = await a.decimalValue(rpnCalculator);
        const bVal = await b.decimalValue(rpnCalculator);
        let results = aVal.mul(bVal);

        if (a instanceof DecimalStackItem || b instanceof DecimalStackItem) {
            const aUnits = (a instanceof DecimalStackItem) ? a.units : null;
            const bUnits = (b instanceof DecimalStackItem) ? b.units : null;
            if (aUnits || bUnits) {
                const q = new Qty(1, aUnits).mul(new Qty(1, bUnits));
                return new DecimalStackItem(results.mul(q.scalar), q.units());
            }
        }

        return results;
    }
}
