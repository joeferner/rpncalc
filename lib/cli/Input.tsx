import {Component} from "ink";

const React = require('./InkToReactBridge').default;

interface InputProps {
    value: string;
    onSubmit: (input: string) => void;
    onChange: (input: string) => void;
    onBackspace: () => void;
    onTab: () => void;
    focus: boolean;
}

interface InputState {
}

interface Key {
    name: string;
    ctrl: boolean;
    meta: boolean;
    shift: boolean;
    sequence: string;
}

export class Input extends Component<InputProps, InputState> {
    constructor(props: InputProps, context: any) {
        super(props, context);
        this.handleKeyPress = this.handleKeyPress.bind(this);
    }

    render() {
        return (<div>&gt; {this.props.value}</div>);
    }

    componentDidMount() {
        process.stdin.on('keypress', this.handleKeyPress);
    }

    componentWillUnmount() {
        process.stdin.removeListener('keypress', this.handleKeyPress);
    }

    handleKeyPress(ch: string, key: Key) {
        if (!this.props.focus) {
            return;
        }

        const {value, onChange, onSubmit} = this.props;

        if (key.name === 'tab') {
            this.props.onTab();
            return;
        }

        if (key.name === 'return') {
            onSubmit(value);
            return;
        }

        if (key.name === 'backspace') {
            if (this.props.onBackspace()) {
                return;
            }
            onChange(value.slice(0, -1));
            return;
        }

        if (key.name === 'space' || key.sequence === ch && /^.*$/.test(ch) && !key.ctrl) {
            onChange(value + ch);
        }
    }
}
