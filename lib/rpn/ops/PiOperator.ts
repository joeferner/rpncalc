import {RpnCalculator} from "../RpnCalculator";
import {Operator} from "./Operator";
import {Decimal} from "decimal.js";

export class PiOperator extends Operator {
    public static PI: Decimal = new Decimal(
        '3.141592653589793238462643383279502884197169399375105820974944592307816406286' +
        '208998628034825342117067982148086513282306647093844609550582231725359408128481' +
        '117450284102701938521105559644622948954930381964428810975665933446128475648233' +
        '786783165271201909145648566923460348610454326648213393607260249141273724587006');

    public getKeywords(): string[] {
        return ['pi'];
    }

    public execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        return rpnCalculator.push(PiOperator.PI);
    }
}
