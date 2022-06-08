import { executeMsg } from "./executeMsg.js";
import { MsgExecuteContract } from "@terra-money/terra.js";

/***
 * @param {any} client
 * @param {OfferUpdateMsg} offer_update
 * @param {Object} user
 */

async function updateOffer(client, offer_update, user) {
  const msg = new MsgExecuteContract(
    user.address,
    global.factoryCfg.offers_addr,
    {
      update_offer: { offer_update },
    }
  );
  console.log("msg :>> ", msg);

  const result = await executeMsg(client, msg, user.wallet);

  return result;
}

export { updateOffer };
