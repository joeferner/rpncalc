import {RpnCalculator} from "../RpnCalculator";
import {BinaryOperator} from "./BinaryOperator";
import {StackItem} from "../StackItem";
import {DecimalStackItem} from "../DecimalStackItem";
import Qty from "js-quantities";
import {Decimal} from "decimal.js";

export class DivideOperator extends BinaryOperator {
    public getKeywords(): string[] {
        return ['divide', 'div', 'divideBy', '/'];
    }

    protected async executeBinary(a: StackItem, b: StackItem, rpnCalculator: RpnCalculator): Promise<Decimal | DecimalStackItem> {
        const aVal = await a.decimalValue(rpnCalculator);
        const bVal = await b.decimalValue(rpnCalculator);
        if (bVal.toNumber() === 0.0) {
            throw new Error('Divide by 0');
        }
        let results = aVal.dividedBy(bVal);

        if (a instanceof DecimalStackItem || b instanceof DecimalStackItem) {
            const aUnits = (a instanceof DecimalStackItem) ? a.units : null;
            const bUnits = (b instanceof DecimalStackItem) ? b.units : null;
            if (aUnits || bUnits) {
                const q = new Qty(1, aUnits).div(new Qty(1, bUnits));
                return new DecimalStackItem(results.mul(q.scalar), q.units());
            }
        }

        return results;
    }
}
