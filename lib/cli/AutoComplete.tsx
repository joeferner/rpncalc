import {Component, Text} from "ink";
import center from "center-align";

const React = require('./InkToReactBridge').default;

interface AutoCompleteProps {
    autoCompletes?: string[],
    autoCompleteIndex?: number
}

interface AutoCompleteState {

}

export class AutoComplete extends Component<AutoCompleteProps, AutoCompleteState> {
    render() {
        if (!this.props.autoCompletes) {
            return (<div/>);
        }

        const maxWidth = this.props.autoCompletes.reduce((p, c) => {
            return Math.max(p, c.length + 2);
        }, 0);
        const autoCompletesToShow = 5;
        let autoCompleteStart = 0;
        if (this.props.autoCompleteIndex >= autoCompletesToShow) {
            autoCompleteStart = this.props.autoCompleteIndex - autoCompletesToShow + 1;
        }
        const autoCompletes = this.props.autoCompletes.slice(autoCompleteStart, autoCompleteStart + autoCompletesToShow);
        return (<div>{
            autoCompletes.map((autoComplete, i) => {
                const selected = (autoCompleteStart + i) === this.props.autoCompleteIndex;
                const fg = selected ? '#000000' : '#ffffff';
                const bg = selected ? '#ffffff' : '#000000';
                const text = center(autoComplete, maxWidth);
                return (<span><Text hex={fg} bgHex={bg}>{text}</Text>&nbsp;</span>);
            })
        }</div>);
    }
}
