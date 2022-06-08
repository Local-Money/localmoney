import { executeMsg } from "./executeMsg.js";
import { MsgRevokeAllowance } from "@terra-money/terra.js";

async function revokeFeeGrant(client, granter, grantee) {
  const revokeFeeGrantMsg = new MsgRevokeAllowance(
    granter.address,
    grantee.address
  );

  const result = await executeMsg(client, revokeFeeGrantMsg, granter.wallet);

  return result;
}

export { revokeFeeGrant };
