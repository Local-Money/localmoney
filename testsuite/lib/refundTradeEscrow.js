import { MsgExecuteContract } from "@terra-money/terra.js";
import { executeMsg } from "./executeMsg.js";

async function refundTradeEscrow(client, tradeAddr, user) {
  const refundMsg = new MsgExecuteContract(user.address, tradeAddr, {
    refund_escrow: {},
  });

  const result = await executeMsg(client, refundMsg, user.wallet);

  return result;
}

export { refundTradeEscrow };
