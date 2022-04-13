import { MsgExecuteContract } from "@terra-money/terra.js";
import { executeMsg } from "./executeMsg.js";

async function fiatDeposited(client, { tradeAddr }, user) {
  const fundEscrowMsg = new MsgExecuteContract(user.address, tradeAddr, {
    fiat_deposited: {},
  });

  const result = await executeMsg(client, fundEscrowMsg, user.wallet);

  if (!result.txhash) {
    console.error(result);
    console.error(`%Error marking fiat_deposited for trade ${tradeAdd}%`);
    throw new Error();
  }
  console.log("result :>> ", result);
  return result;
}

export { fiatDeposited };
