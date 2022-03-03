import { MsgExecuteContract } from "@terra-money/terra.js";
import { executeMsg } from "./executeMsg.js";

async function disputeTrade(client, tradeAddr, user) {
  const disputeMsg = new MsgExecuteContract(user.address, tradeAddr, {
    dispute: {},
  });

  const result = await executeMsg(client, disputeMsg, user.wallet);
  if (!result.txhash) {
    console.error("ERROR:", result);
    throw Error(`%Error disputing Trade ${tradeAddr}%`);
  }
}

export { disputeTrade };
