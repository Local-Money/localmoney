import { MsgExecuteContract } from "@terra-money/terra.js";
import { executeMsg } from "./executeMsg.js";

async function releaseTradeEscrow(client, tradeAddr, user) {
  const releaseMsg = new MsgExecuteContract(user.address, tradeAddr, {
    release: {},
  });

  const result = await executeMsg(client, releaseMsg, user.wallet);

  return result;
}

export { releaseTradeEscrow };
