import {Component} from "ink";
import {AngleMode, RpnCalculator} from "../rpn/RpnCalculator";

const React = require('./InkToReactBridge').default;

interface StatusLineProps {
    rpnCalculator: RpnCalculator;
    error: Error;
}

interface StatusLineState {

}

export class StatusLine extends Component<StatusLineProps, StatusLineState> {
    private getBaseString(): string {
        switch (this.props.rpnCalculator.getOption(RpnCalculator.OPTION_BASE, 10)) {
            case 2:
                return 'bin';
            case 8:
                return 'oct';
            case 10:
                return 'dec';
            case 16:
                return 'hex';
            default:
                return '???';
        }
    }

    private getAngleModeString(): string {
        switch (this.props.rpnCalculator.getOption(RpnCalculator.OPTION_ANGLE_MODE)) {
            case AngleMode.Degrees:
                return 'deg';
            case AngleMode.Radians:
                return 'rad';
            default:
                return '???';
        }
    }

    render() {
        if (this.props.error) {
            return (<div>{this.props.error.message}</div>);
        }

        return (<div>{this.getBaseString()} {this.getAngleModeString()}</div>);
    }
}
