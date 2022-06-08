import { Fee, Coin } from "@terra-money/terra.js";

async function executeMsgPaidByFeeGrant(client, msg, wallet) {
  try {
    const feeCoin = [
      Coin.fromData({
        denom: "uusd",
        amount: 1500000,
      }),
    ];

    const tx = await wallet.createAndSignTx({
      msgs: [msg],
      fee: new Fee(
        10000000,
        feeCoin,
        undefined,
        "terra10ms2n6uqzgrz4gtkcyslqx0gysfvwlg6n2tusk"
      ),
    });

    const result = await client.tx.broadcast(tx);
    console.log("result :>> ", result);
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

export { executeMsgPaidByFeeGrant };
