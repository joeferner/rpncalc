import {Decimal} from "decimal.js";
import {RpnCalculator} from "./RpnCalculator";
import {StackItem} from "./StackItem";

export class DecimalStackItem extends StackItem {
    private _value: Decimal;
    private _units: string;

    constructor(decimal: Decimal | string | number, units?: string) {
        super();
        if (units) {
            this._value = <Decimal>decimal;
            this._units = units;
        } else if ((<string>decimal).match) {
            let str = <string>decimal;
            const m = str.match(/(.*)_(.*)/);
            this._units = null;
            if (m) {
                str = m[1];
                this._units = m[2];
            }
            this._value = new Decimal(str);
        } else {
            this._value = new Decimal(decimal);
            this._units = null;
        }
    }

    public get value(): any {
        return this._value;
    }

    public get units(): string {
        return this._units;
    }

    public async decimalValue(rpnCalculator: RpnCalculator): Promise<Decimal> {
        return this._value;
    }

    public toString(rpnCalculator: RpnCalculator) {
        const base = rpnCalculator.getOption(RpnCalculator.OPTION_BASE, 10);
        let str;
        if (base === 10) {
            str = DecimalStackItem.toBase10String(this._value, rpnCalculator);
        } else if (base == 16) {
            str = DecimalStackItem.spaceNonBase10Digits(this._value.toHex(), rpnCalculator);
        } else if (base == 8) {
            str = DecimalStackItem.spaceNonBase10Digits(this._value.toOctal(), rpnCalculator);
        } else if (base == 2) {
            str = DecimalStackItem.spaceNonBase10Digits(this._value.toBinary(), rpnCalculator);
        } else {
            throw new Error(`unhandled base: ${base}`);
        }
        if (this.units) {
            str += `_${this.units}`;
        }
        return str;
    }

    private static toBase10String(number: Decimal, rpnCalculator: RpnCalculator) {
        let str = number.toString();
        if (!rpnCalculator.getOption(RpnCalculator.OPTION_DIGIT_GROUPING, false)) {
            return str;
        }
        let whole, decimal;
        let m = str.match(/([0-9]*)\.([0-9]*)/);
        if (m) {
            whole = m[1];
            decimal = m[2];
        } else {
            whole = str;
            decimal = null;
        }

        whole = whole.replace(/(\d)(?=(\d{3})+$)/g, '$1,');
        if (decimal) {
            decimal = decimal.replace(/(\d{3})/g, '$1 ');
        }

        return whole + (decimal ? '.' + decimal : '');
    }

    private static spaceNonBase10Digits(number: string, rpnCalculator: RpnCalculator) {
        let str = number.toString();
        if (!rpnCalculator.getOption(RpnCalculator.OPTION_DIGIT_GROUPING, false)) {
            return str;
        }
        let whole, decimal;
        let m = str.match(/([0][a-z])(.*)/);
        if (!m) {
            return str;
        }
        let prefix = m[1];
        str = m[2];

        m = str.match(/([^.]*)\.([^.]*)/);
        if (m) {
            whole = m[1];
            decimal = m[2];
        } else {
            whole = str;
            decimal = null;
        }

        whole = whole.replace(/(.)(?=(.{4})+$)/g, '$1 ');
        if (decimal) {
            decimal = decimal.replace(/(.{4})/g, '$1 ');
        }

        return prefix + whole + (decimal ? '.' + decimal : '');
    }

    public save(): any {
        return {
            type: 'DecimalStackItem',
            args: [this._value.toString(), this._units]
        }
    }
}
