import { executeMsg } from "./executeMsg.js";
import { MsgExecuteContract } from "@terra-money/terra.js";

async function createOffer(client, offer, user) {
  const createOfferMsg = new MsgExecuteContract(
    user.address,
    global.factoryCfg.offers_addr,
    {
      create: { offer },
    }
  );

  const result = await executeMsg(client, createOfferMsg, user.wallet);

  return result;
}

export { createOffer };
