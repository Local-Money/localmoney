import { LCDClient, Coins } from "@terra-money/terra.js";
import axios from "axios";

async function createLCDClient() {
  const LCDOptions = {
    URL: process.env.LCDURL,
    chainID: process.env.NETWORK,
  };

  // Fetch gas prices and convert to `Coin` format.
  if (process.env.FCDURL) {
    const gasPriceQuery = await axios.get(
      `${process.env.FCDURL}/v1/txs/gas_prices`
    );

    const gasPrices = new Coins(gasPriceQuery.data);

    LCDOptions.gasPrices = gasPrices;
    LCDOptions.gasAdjustment = "2";
    LCDOptions.gas = 10000000;
  }
  LCDOptions.gasAdjustment = "2";
  LCDOptions.gas = 10000000;

  console.log("LCDClient", {
    URL: process.env.LCDURL,
    chainID: process.env.NETWORK,
    hasGasPrices: !!LCDOptions.gasPrices,
  });

  const client = new LCDClient(LCDOptions);

  return client;
}

export { createLCDClient };
