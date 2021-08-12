export class RpnCalc {
    stack: string[] = [];

    getStackItem(index: number): string {
        return this.stack[index];
    }

    push(value: string): void {
        this.stack.unshift(value);
    }
}