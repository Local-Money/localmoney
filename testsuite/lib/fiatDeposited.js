import { MsgExecuteContract } from "@terra-money/terra.js";
import { executeMsg } from "./executeMsg.js";

async function fiatDeposited(
  client,
  { tradeAddr },
  user,
  feePayer = undefined
) {
  const fundEscrowMsg = new MsgExecuteContract(user.address, tradeAddr, {
    fiat_deposited: {},
  });

  const result = await executeMsg(client, fundEscrowMsg, user.wallet, feePayer);
  console.log("fiatdeposited result :>> ", result);

  if (!result.txhash) {
    console.error(result);
    console.error(`%Error marking fiat_deposited for trade ${tradeAddr}%`);
    throw new Error();
  }
  return result;
}

export { fiatDeposited };
