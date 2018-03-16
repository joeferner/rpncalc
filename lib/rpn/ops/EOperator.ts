import {RpnCalculator} from "../RpnCalculator";
import {Operator} from "./Operator";
import {Decimal} from "decimal.js";

export class EOperator extends Operator {
    public static E: Decimal = new Decimal(
        '2.718281828459045235360287471352662497757247093699959574966967627724076630353' +
        '547594571382178525166427427466391932003059921817413596629043572900334295260' +
        '595630738132328627943490763233829880753195251019011573834187930702154089149' +
        '934884167509244761460668082264800168477411853742345442437107539077744992069');

    public getKeywords(): string[] {
        return ['e'];
    }

    public execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        return rpnCalculator.push(EOperator.E);
    }
}
