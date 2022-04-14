import { MsgExecuteContract } from "@terra-money/terra.js";
import { executeMsg } from "./executeMsg.js";

async function acceptTradeRequest(client, tradeAddr, user) {
  const releaseMsg = new MsgExecuteContract(user.address, tradeAddr, {
    accept_request: {},
  });

  const result = await executeMsg(client, releaseMsg, user.wallet);

  return result;
}

export { acceptTradeRequest };
