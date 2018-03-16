import {Component} from "ink";
import {RpnCalculator} from "../rpn/RpnCalculator";

const React = require('./InkToReactBridge').default;

interface StackProps {
    rpnCalculator: RpnCalculator;
    lines: number;
}

interface StackState {
}

export class Stack extends Component<StackProps, StackState> {
    render() {
        const lines = [];
        for (let i = this.props.lines - 1; i >= 0; i--) {
            const si = this.props.rpnCalculator.peek(i);
            lines.push({
                index: i + 1,
                value: si == null ? '' : si.toString(this.props.rpnCalculator)
            });
        }

        return (<span>
            {lines.map(line => {
                return (<div>{line.index}: {line.value}</div>);
            })}
        </span>);
    }
}
