import {RpnCalculator} from "../RpnCalculator";
import {Operator} from "./Operator";
import {ExpressionParser} from "../../expression/ExpressionParser";
import {ExpressionStackItem} from "../ExpressionStackItem";

export class EvalOperator extends Operator {
    private _expressionParser: ExpressionParser;

    constructor() {
        super();
        this._expressionParser = new ExpressionParser();
    }

    public getKeywords(): string[] {
        return ['eval'];
    }

    public execute(rpnCalculator: RpnCalculator, input: string): Promise<void> {
        if (rpnCalculator.size() < 1) {
            return Promise.reject(new Error('Not enough operands'));
        }
        let a = rpnCalculator.peek(0);
        if (!(a instanceof ExpressionStackItem)) {
            return Promise.reject(new Error('Operand must be an expression'));
        }
        return this._expressionParser.execute(a.value, rpnCalculator)
            .then(results => {
                rpnCalculator.pop(1);
                return rpnCalculator.push(results);
            });
    }
}
