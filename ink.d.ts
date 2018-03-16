declare module 'ink' {
    export type exitFunction = () => void;

    export class h {
    }

    export class Component<TProps, TState> {
        props: TProps;
        state: TState;

        constructor(props?: TProps, context?: any);

        setState(newState: TState): void;
    }

    export interface Text {
    }

    export function render(component: any): exitFunction;
}
