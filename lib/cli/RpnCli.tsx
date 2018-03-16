import {Component, exitFunction, render} from "ink";
import {RpnCalculator} from "../rpn/RpnCalculator";
import {Stack} from "./Stack";
import {Input} from "./Input";
import {StatusLine} from "./StatusLine";
import {Key} from "readline";
import * as fs from "fs-extra";
import * as path from "path";
import AppDirectory from "appdirectory";

const React = require('./InkToReactBridge').default;

interface RpnCliComponentProps {
    rpnCalculator: RpnCalculator;
}

interface RpnCliComponentState {
    error?: Error;
    value?: string;
}

class RpnCliComponent extends Component<RpnCliComponentProps, RpnCliComponentState> {
    constructor(props: RpnCliComponentProps, context: any) {
        super(props, context);
        this.handleInputSubmit = this.handleInputSubmit.bind(this);
        this.handleInputChange = this.handleInputChange.bind(this);
        this.handleInputBackspace = this.handleInputBackspace.bind(this);
        this.state = {
            error: null,
            value: ''
        };
    }

    setBase(base: number) {
        this.props.rpnCalculator.setOption(RpnCalculator.OPTION_BASE, base)
            .then(() => {
                this.setState({
                    value: ''
                });
            })
            .catch((error) => {
                this.setState({
                    error
                });
            });
    }

    handleInputChange(value: string) {
        const lastChar = value.length > 0 ? value.substr(value.length - 1) : '';
        switch (lastChar) {
            case '+':
            case '-':
            case '*':
            case '/':
            case '^':
            case '%':
                return this.doImmediateOperator(value.substr(0, value.length - 1), lastChar);
        }

        this.setState({
            value: value,
            error: null
        });
    }

    // private autoComplete(inputString: string, callback: AutoCompleteCallback) {
    //     let arr = this.rpnCalculator.getAutoCompletes();
    //     arr = arr.concat([
    //         'hex', 'hexadecimal',
    //         'dec', 'decimal',
    //         'oct', 'octal',
    //         'bin', 'binary'
    //     ]);
    //     callback(undefined, termKitAutoComplete(arr, inputString, true));
    // }

    private doImmediateOperator(input: string, op: string) {
        if (input.length > 0) {
            this.props.rpnCalculator.push(input)
                .then(() => {
                    this.push(op);
                })
                .catch((err) => {
                    // do nothing this could be an expression
                });
        } else {
            this.push(op);
        }
    }

    private handleInputBackspace() {
        if (this.state.value.length == 0) {
            this.props.rpnCalculator.pop()
                .then(() => {
                    this.setState({});
                })
                .catch((error) => {
                    this.setState({error});
                });
            return true;
        }
        return false;
    }

    handleInputSubmit(value: string) {
        switch (value) {
            case 'hex':
            case 'hexadecimal':
                this.setBase(16);
                return;
            case 'dec':
            case 'decimal':
                this.setBase(10);
                return;
            case 'oct':
            case 'octal':
                this.setBase(8);
                return;
            case 'bin':
            case 'binary':
                this.setBase(2);
                return;
        }

        this.push(value);
    }

    private push(value: string) {
        this.props.rpnCalculator.push(value)
            .then(() => {
                this.setState({
                    value: '',
                    error: null
                });
            })
            .catch((error) => {
                this.setState({
                    error
                });
            });
    }

    render() {
        return (<span>
            <StatusLine error={this.state.error} rpnCalculator={this.props.rpnCalculator}/>
            <Stack lines={5} rpnCalculator={this.props.rpnCalculator}/>
            <Input value={this.state.value}
                   focus={true}
                   onChange={this.handleInputChange}
                   onSubmit={this.handleInputSubmit}
                   onBackspace={this.handleInputBackspace}/>
        </span>);
    }
}

export class RpnCli {
    private rpnCalculator: RpnCalculator;
    private exit: exitFunction;

    constructor(rpnCalculator: RpnCalculator) {
        this.rpnCalculator = rpnCalculator;
    }

    async start() {
        const configFile = RpnCli.getConfigFile();
        await fs.mkdirs(path.dirname(configFile));
        const config = await RpnCli.loadConfig(configFile);
        this.rpnCalculator.loadConfig(config.rpnCalculator);

        this.exit = render(<RpnCliComponent rpnCalculator={this.rpnCalculator}/>);
        RpnCli.removeInkDefaultKeyPressListener();
        process.stdin.on('keypress', (ch: string, key: Key) => {
            if (key.ctrl && key.name === 'c') {
                this.handleExit();
            }
        });
    }

    private handleExit() {
        const config = {
            rpnCalculator: this.rpnCalculator.getConfig()
        };
        RpnCli.saveConfig(RpnCli.getConfigFile(), config);
        this.exit();
    }

    private static saveConfig(configFile: string, config: any) {
        fs.writeFileSync(configFile, JSON.stringify(config, null, 2));
    }

    private static getConfigFile() {
        return path.join(new AppDirectory('rpn').userConfig(), 'config.json');
    }

    private static async loadConfig(configFile: string) {
        if (await fs.pathExists(configFile)) {
            return await fs.readJson(configFile)
        }
        return {
            rpnCalculator: {}
        };
    }

    private static removeInkDefaultKeyPressListener() {
        const keyPressListeners = process.stdin.listeners('keypress');
        for (let keyPressListener of keyPressListeners) {
            let str = keyPressListener.toString();
            if (str.includes('escape') && str.includes('exit')) {
                process.stdin.removeListener('keypress', keyPressListener as any);
                break;
            }
        }
    }
}
