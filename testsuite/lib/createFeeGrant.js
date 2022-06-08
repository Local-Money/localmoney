import { executeMsg } from "./executeMsg.js";
import {
  MsgGrantAllowance,
  BasicAllowance,
  Coin,
  Coins,
} from "@terra-money/terra.js";

async function createFeeGrant(client, granter, grantee) {
  const expiration = new Date(Date.now() + 12096e5); // 14 days in the future

  const spend_limit = [
    Coin.fromData({
      denom: "uusd",
      amount: 10000000,
    }),
  ];

  const allowance = new BasicAllowance(spend_limit, expiration);

  const createFeeGrantMsg = new MsgGrantAllowance(
    granter.address,
    grantee.address,
    allowance
  );

  const result = await executeMsg(client, createFeeGrantMsg, granter.wallet);

  return result;
}

export { createFeeGrant };
