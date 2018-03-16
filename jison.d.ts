declare module 'jison' {
    export class Parser {
        yy: any;

        constructor(str: string);

        parse(str: string): any;
    }
}
