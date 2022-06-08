import { Fee, Coin } from "@terra-money/terra.js";

async function executeMsg(client, msg, wallet, feePayer = undefined) {
  try {
    const txOptions = {
      msgs: [msg],
    };

    if (feePayer) {
      const feeCoin = [
        Coin.fromData({
          denom: "uusd",
          amount: 1500000,
        }),
      ];

      txOptions.fee = new Fee(10000000, feeCoin, undefined, feePayer);
    }

    const tx = await wallet.createAndSignTx(txOptions);

    const result = await client.tx.broadcast(tx);

    if (process.env.LOG_RESULT) console.log("result :>> ", result);

    return result;
  } catch (err) {
    if (err.response && err.response.data) {
      console.error("ERROR:", err.response.data);
    } else {
      console.error("ERROR:", err);
    }

    throw err;
  }
}

export { executeMsg };
