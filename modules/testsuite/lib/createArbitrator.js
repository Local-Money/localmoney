import { executeMsg } from "./executeMsg.js";
import { MsgExecuteContract } from "@terra-money/terra.js";

async function createArbitrator(client, arbitrator, user) {
  const createArbitratorMsg = new MsgExecuteContract(
    user.address,
    global.factoryCfg.offers_addr,
    {
      new_arbitrator: { ...arbitrator },
    }
  );

  const result = await executeMsg(client, createArbitratorMsg, user.wallet);

  return result;
}

export { createArbitrator };
