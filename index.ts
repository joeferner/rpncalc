#!/usr/bin/env node

import {RpnCalculator} from "./lib/rpn/RpnCalculator";
import {RpnCli} from "./lib/cli/RpnCli";

async function run() {
    const rpnCalculator = new RpnCalculator();
    const rpnCli = new RpnCli(rpnCalculator);
    await rpnCli.start();
}

run();
