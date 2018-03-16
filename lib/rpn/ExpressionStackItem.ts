import {Decimal} from "decimal.js";
import {RpnCalculator} from "./RpnCalculator";
import {ExpressionParser} from "../expression/ExpressionParser";
import {StackItem} from "./StackItem";

export class ExpressionStackItem extends StackItem {
    private _expression: string;

    constructor(expression: string) {
        super();
        this._expression = expression;
    }

    public get value(): any {
        return this._expression;
    }

    public async decimalValue(rpnCalculator: RpnCalculator): Promise<Decimal> {
        const values = await new ExpressionParser().execute(this._expression, rpnCalculator);
        return values[0].value;
    }

    get expression(): string {
        return this._expression;
    }

    public toString(rpnCalculator: RpnCalculator) {
        return `'${this._expression}'`;
    }

    public save(): any {
        return {
            type: 'ExpressionStackItem',
            args: [this._expression]
        }
    }
}
