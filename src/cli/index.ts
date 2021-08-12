import {RpnCalcCli} from "./RpnCalcCli";

export default function cli() {
    new RpnCalcCli().run().then(() => {
        process.exit(0);
    });
}
