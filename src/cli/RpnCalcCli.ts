import {ScreenBuffer, terminal} from "terminal-kit";
import {RpnCalc} from "../rpn/RpnCalc";
import * as fs from "fs";

export class RpnCalcCli {
    private rpnCalc = new RpnCalc();
    private stackSize = 5;
    private buffer: ScreenBuffer;
    private exit = false;
    private exitFn: undefined | (() => void);

    constructor() {
        console.log("\n".repeat(this.stackSize + 1));
        terminal.grabInput({});
        terminal.on('key', (key: string, matches: string[]) => {
            if (matches.includes('CTRL_C')) {
                this.doExit();
            } else if (matches.length === 1 && key === matches[0]) {
            } else {
                fs.promises.appendFile('rpncalc.log', `${key} ${matches}\n`);
            }
        });
        this.buffer = this.createBuffer();
    }

    private createBuffer(): ScreenBuffer {
        const height = this.stackSize + 2;
        return new ScreenBuffer({
            height,
            dst: terminal,
            x: 1,
            y: terminal.height - height
        });
    }

    private doExit(): void {
        this.exit = true;
        if (this.exitFn) {
            this.exitFn();
        }
        console.log();
    }

    run(): Promise<void> {
        this.refresh();
        this.waitForExit();
        return new Promise<void>((resolve) => {
            this.exitFn = resolve;
        });
    }

    private waitForExit(): void {
        setTimeout(() => {
            if (!this.exit) {
                this.waitForExit();
            }
        }, 100);
    }

    private refresh() {
        const options: ScreenBuffer.PutOptions = {
            x: 0,
            y: 0,
            dx: 1,
            dy: 0,
            direction: "right",
            wrap: false,
            attr: {}
        };
        this.buffer.fill({
            char: ' ',
            attr: {}
        });

        let maxItemWidth = 0;
        for (let i = 0; i < this.stackSize; i++) {
            const item = this.rpnCalc.getStackItem(i);
            maxItemWidth = Math.max(maxItemWidth, `${item || ''}`.length);
        }
        const MIN_WIDTH = 10;
        const MAX_WIDTH = 30;
        const itemWidth = Math.min(Math.max(MIN_WIDTH, maxItemWidth), MAX_WIDTH);
        for (let i = 0; i < this.stackSize; i++) {
            const item = this.rpnCalc.getStackItem(i);
            let itemStr = (item || '').padStart(itemWidth);
            if (itemStr.length > MAX_WIDTH) {
                itemStr = `…` + itemStr.substr(itemStr.length - MAX_WIDTH + 1);
            }
            this.buffer.put({...options, y: this.stackSize - i}, `${i}: ${itemStr}`);
        }
        this.buffer.put({...options, x: 0, y: this.stackSize + 1}, ">");
        this.buffer.moveTo(1, this.stackSize + 1);
        this.buffer.draw();
        this.buffer.drawCursor();
        terminal.inputField({
            default: ''
        }, (error, results) => {
            if (results) {
                this.rpnCalc.push(results);
            }
            this.refresh();
        });
    }
}
