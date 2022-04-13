import { MsgExecuteContract } from "@terra-money/terra.js";
import { executeMsg } from "./executeMsg.js";

async function cancelTradeRequest(client, tradeAddr, user) {
  const refundMsg = new MsgExecuteContract(user.address, tradeAddr, {
    cancel_request: {},
  });

  const result = await executeMsg(client, refundMsg, user.wallet);

  return result;
}

export { cancelTradeRequest };
