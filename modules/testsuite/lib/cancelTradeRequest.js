import { MsgExecuteContract } from "@terra-money/terra.js";
import { executeMsg } from "./executeMsg.js";

async function cancelTradeRequest(client, tradeAddr, user) {
  const disputeMsg = new MsgExecuteContract(user.address, tradeAddr, {
    cancel_request: {},
  });

  const result = await executeMsg(client, disputeMsg, user.wallet);
  if (!result.txhash) {
    console.error("ERROR:", result);
    throw Error(`%Error cancelling Trade Request ${tradeAddr}%`);
  }
}

export { cancelTradeRequest };
