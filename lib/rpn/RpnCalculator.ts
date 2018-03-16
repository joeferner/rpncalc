import {DecimalStackItem} from "./DecimalStackItem";
import {StackItem} from "./StackItem";
import {Decimal} from "decimal.js";
import {Operator} from "./ops/Operator";
import {PlusOperator} from "./ops/PlusOperator";
import {MultiplyOperator} from "./ops/MultiplyOperator";
import {DivideOperator} from "./ops/DivideOperator";
import {MinusOperator} from "./ops/MinusOperator";
import {PiOperator} from "./ops/PiOperator";
import {SinOperator} from "./ops/SinOperator";
import {CosOperator} from "./ops/CosOperator";
import {TanOperator} from "./ops/TanOperator";
import {ACosOperator} from "./ops/ACosOperator";
import {ASinOperator} from "./ops/ASinOperator";
import {ATanOperator} from "./ops/ATanOperator";
import {ATan2Operator} from "./ops/ATan2Operator";
import {StoreOperator} from "./ops/StoreOperator";
import {ExpressionStackItem} from "./ExpressionStackItem";
import {AngleModeRadians} from "./ops/AngleModeRadians";
import {AngleModeDegrees} from "./ops/AngleModeDegrees";
import {EOperator} from "./ops/EOperator";
import {LnOperator} from "./ops/LnOperator";
import {Log10Operator} from "./ops/Log10Operator";
import {Log2Operator} from "./ops/Log2Operator";
import {LogOperator} from "./ops/LogOperator";
import {DupOperator} from "./ops/DupOperator";
import {NegateOperator} from "./ops/NegateOperator";
import {SquareRootOperator} from "./ops/SquareRootOperator";
import {ClearOperator} from "./ops/ClearOperator";
import {PowerOperator} from "./ops/PowerOperator";
import {NRootOperator} from "./ops/NRootOperator";
import {ModOperator} from "./ops/ModOperator";
import {SetOptionOperator} from "./ops/SetOptionOperator";
import {GetOptionOperator} from "./ops/GetOptionOperator";
import {EvalOperator} from "./ops/EvalOperator";
import {ConvertOperator} from "./ops/ConvertOperator";
import Qty from "js-quantities";
import {InverseOperator} from "./ops/InverseOperator";
import {SwapOperator} from "./ops/SwapOperator";
import {DegreesToRadiansOperator} from "./ops/DegreesToRadiansOperator";
import {RadiansToDegreesOperator} from "./ops/RadiansToDegreesOperator";

export enum AngleMode {
    Degrees,
    Radians
}

export interface CommonDecimalValues {
    values: Decimal[];
    units: string;
}

interface ConfigSave {
    stack: any[],
    variables: { [name: string]: any },
    options: { [name: string]: any }
}

export class RpnCalculator {
    public static OPTION_BASE = 'base';
    public static OPTION_DIGIT_GROUPING = 'digitGrouping';
    public static OPTION_ANGLE_MODE = 'angleMode';

    private _stack: StackItem[];
    private _operators: Operator[];
    private _variables: { [name: string]: StackItem };
    private _options: { [name: string]: any };

    constructor() {
        this._stack = [];
        this._variables = {};
        this._options = {
            [RpnCalculator.OPTION_DIGIT_GROUPING]: false,
            [RpnCalculator.OPTION_BASE]: 10,
            [RpnCalculator.OPTION_ANGLE_MODE]: AngleMode.Degrees
        };
        this._operators = [
            new PlusOperator(),
            new MinusOperator(),
            new MultiplyOperator(),
            new DivideOperator(),
            new PiOperator(),
            new EOperator(),
            new SinOperator(),
            new CosOperator(),
            new TanOperator(),
            new ASinOperator(),
            new ACosOperator(),
            new ATanOperator(),
            new ATan2Operator(),
            new StoreOperator(),
            new AngleModeRadians(),
            new AngleModeDegrees(),
            new LnOperator(),
            new Log2Operator(),
            new Log10Operator(),
            new LogOperator(),
            new DupOperator(),
            new NegateOperator(),
            new SquareRootOperator(),
            new ClearOperator(),
            new PowerOperator(),
            new NRootOperator(),
            new ModOperator(),
            new SetOptionOperator(),
            new GetOptionOperator(),
            new EvalOperator(),
            new ConvertOperator(),
            new InverseOperator(),
            new SwapOperator(),
            new DegreesToRadiansOperator(),
            new RadiansToDegreesOperator()
        ];
    }

    public async push(input: string | Decimal | StackItem | string[] | Decimal[] | StackItem[]): Promise<void> {
        if (Array.isArray(input)) {
            if (input.length === 0) {
                return Promise.resolve();
            }
            return this.push(input[0])
                .then(() => {
                    return this.push((<any[]>input).slice(1));
                });
        }

        if (input instanceof StackItem) {
            this._stack.push(<StackItem>input);
            return Promise.resolve();
        }

        if (typeof input === 'string') {
            input = input.trim();

            for (let op of this._operators) {
                if (op.matches(input)) {
                    return op.execute(this, input);
                }
            }

            if (input in this._variables) {
                const value = this._variables[input];
                this._stack.push(value);
                return Promise.resolve();
            }

            if (input.startsWith("'") && input.endsWith("'")) {
                this._stack.push(new ExpressionStackItem(input.substr(1, input.length - 2)));
                return Promise.resolve();
            }
        }

        input = '' + input; // convert to string
        if (input.toLocaleLowerCase() === 'false') {
            input = '0';
        } else if (input.toLocaleLowerCase() === 'true') {
            input = '1';
        }

        try {
            let str = <string>input;
            str = str.replace(/,/g, '')
                .replace(/ /g, '');
            this._stack.push(new DecimalStackItem(str));
            return Promise.resolve();
        } catch (err) {
            return Promise.reject(err);
        }
    }

    public pop(count?: number): Promise<StackItem[]> {
        count = count || 1;
        if (this._stack.length > 0) {
            const results = this._stack.slice(this._stack.length - count);
            this._stack = this._stack.slice(0, this._stack.length - count);
            return Promise.resolve(results);
        } else {
            return Promise.reject(new Error('No items on stack to pop'));
        }
    }

    public size(): number {
        return this._stack.length;
    }

    public peek(i: number): StackItem {
        return this._stack[this._stack.length - 1 - i];
    }

    public getAutoCompletes(): string[] {
        let arr: string[] = [];
        for (let op of this._operators) {
            arr = arr.concat(op.getKeywords());
        }
        return arr;
    }

    public store(name: string, value: StackItem) {
        for (let op of this._operators) {
            if (op.matches(name)) {
                throw new Error('Cannot overwrite operators');
            }
        }

        this._variables[name] = value;
    }

    public setOption(name: string, value: any): Promise<void> {
        if (value instanceof Decimal) {
            value = value.toNumber();
        }

        switch (name) {
            case RpnCalculator.OPTION_BASE:
                switch (value) {
                    case 2:
                    case 8:
                    case 10:
                    case 16:
                        break;
                    default:
                        return Promise.reject(new Error('Invalid base'));
                }
                break;
        }

        this._options[name] = value;
        return Promise.resolve();
    }

    public getOption(name: string, defaultValue?: any): any {
        if (name in this._options) {
            return this._options[name];
        }
        return defaultValue;
    }

    toStringStack(): string {
        let s = '';
        for (let i = this._stack.length - 1; i >= 0; i--) {
            if (i !== this._stack.length - 1) {
                s = s + '\n';
            }
            s = s + this._stack[i].toString(this);
        }
        return s;
    }

    public async convertToCommonDecimalValues(...args: StackItem[]): Promise<CommonDecimalValues> {
        let values: Decimal[] = [];
        let toUnit = null;
        for (let i = 0; i < args.length; i++) {
            values.push(await args[i].decimalValue(this));
            if (!toUnit && args[i] instanceof DecimalStackItem) {
                toUnit = new Qty(1, (<DecimalStackItem>args[i]).units).units();
            }
        }

        if (toUnit) {
            for (let i = 0; i < args.length; i++) {
                const units = (<DecimalStackItem>args[i]).units;
                if (units) {
                    const q = new Qty(1, units).to(toUnit);
                    values[i] = values[i].mul(q.scalar);
                }
            }
        }

        return {
            values,
            units: toUnit
        }
    }

    public async convert(value: DecimalStackItem, toUnits: string): Promise<DecimalStackItem> {
        if (value.units) {
            const q = new Qty(1, value.units).to(toUnits);
            return new DecimalStackItem(value.value.mul(q.scalar), q.units());
        } else {
            return new DecimalStackItem(value.value, toUnits);
        }
    }

    loadConfig(config: any) {
        if (config.stack) {
            this._stack = config.stack.map((si: any) => RpnCalculator.loadStackItem(si));
        }
        if (config.variables) {
            this._variables = {};
            for (let v in config.variables) {
                this._variables[v] = RpnCalculator.loadStackItem(config.variables[v]);
            }
        }
        if (config.options) {
            this._options = config.options;
        }
    }

    getConfig(): any {
        const config: ConfigSave = {
            stack: this._stack.map(si => si.save()),
            variables: {},
            options: this._options
        };
        for (let v in this._variables) {
            config.variables[v] = this._variables[v].save();
        }
        return config;
    }

    private static loadStackItem(data: any): StackItem {
        switch (data.type) {
            case 'ExpressionStackItem':
                return new ExpressionStackItem(data.args[0]);
            case 'DecimalStackItem':
                return new DecimalStackItem(data.args[0], data.args[1]);
            default:
                throw new Error(`Unhandled type: ${data.type}`);
        }
    }

    public cloneWithoutStackItems(): RpnCalculator {
        const result = new RpnCalculator();
        for (let v in this._variables) {
            result._variables[v] = this._variables[v];
        }
        for (let v in this._options) {
            result._options[v] = this._options[v];
        }
        return result;
    }
}

