import {Parser} from "jison";
import * as path from "path";
import * as fs from "fs";
import {RpnCalculator} from "../rpn/RpnCalculator";
import {StackItem} from "../rpn/StackItem";

enum PartType {
    BINARY,
    UNARY,
    VALUE,
    FUNCTION_CALL
}

interface Part {
    type: PartType;
    op?: string;
    a?: Part;
    b?: Part;
    value?: string;
    functionName?: string;
    args?: Part[];
}

export class ExpressionParser {
    private grammer: string;

    constructor() {
        this.grammer = fs.readFileSync(ExpressionParser.findJisonFile(), 'utf8');
    }

    private static findJisonFile() {
        let s = path.join(__dirname, 'expression.jison');
        if (fs.existsSync(s)) {
            return s;
        }
        s = path.join(__dirname, '../../../lib/expression/expression.jison');
        if (fs.existsSync(s)) {
            return s;
        }
        throw new Error('could not find jison file');
    }

    public async execute(expr: string, rpnCalculator: RpnCalculator): Promise<StackItem[]> {
        try {
            const parser = new Parser(this.grammer);
            parser.yy = {
                binary: (a: Part, b: Part, op: string) => {
                    return {type: PartType.BINARY, a, b, op};
                },
                unary: (a: Part, op: string) => {
                    return {type: PartType.UNARY, a, op};
                },
                value: (value: string) => {
                    return {type: PartType.VALUE, value};
                },
                functionCall: (functionName: string, args: Part[]) => {
                    return {type: PartType.FUNCTION_CALL, functionName, args};
                }
            };
            const calc = rpnCalculator.cloneWithoutStackItems();
            const tree: Part[] = parser.parse(expr);
            return ExpressionParser.executeTreeArray(tree, calc)
                .then(() => {
                    return calc.pop(calc.size());
                });
        } catch (e) {
            return Promise.reject(e);
        }
    }

    private static executeTree(tree: Part, calc: RpnCalculator): Promise<void> {
        switch (tree.type) {
            case PartType.VALUE:
                return calc.push(tree.value);

            case PartType.BINARY:
                return ExpressionParser.executeTree(tree.a, calc)
                    .then(() => {
                        return ExpressionParser.executeTree(tree.b, calc)
                            .then(() => {
                                return calc.push(tree.op);
                            });
                    });

            case PartType.UNARY:
                return ExpressionParser.executeTree(tree.a, calc)
                    .then(() => {
                        return calc.push(tree.op);
                    });

            case PartType.FUNCTION_CALL:
                return ExpressionParser.executeTreeArray(tree.args, calc)
                    .then(() => {
                        return calc.push(tree.functionName);
                    });

            default:
                return Promise.reject(new Error(`unhandled tree part type: ${tree.type}`));
        }
    }

    private static executeTreeArray(args: Part[], calc: RpnCalculator): Promise<void> {
        if (args.length === 0) {
            return Promise.resolve();
        }
        return this.executeTree(args[0], calc)
            .then(() => {
                return this.executeTreeArray(args.slice(1), calc);
            });
    }
}
