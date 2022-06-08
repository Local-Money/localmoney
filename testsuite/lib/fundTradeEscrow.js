import { Coin, Coins, MsgExecuteContract } from "@terra-money/terra.js";
import { executeMsg } from "./executeMsg.js";

async function fundTradeEscrow(client, { tradeAddr, offerId, amount }, user) {
  console.log(
    "Funding Escrow:",
    `https://terrasco.pe/testnet/address/${tradeAddr}`
  );

  const coin = Coin.fromData({
    denom: "uusd",
    amount: parseInt(amount) + "",
  });

  const coins = new Coins([coin]);

  const fundEscrowMsg = new MsgExecuteContract(
    user.address,
    tradeAddr,
    { fund_escrow: {} },
    coins
  );

  const result = await executeMsg(client, fundEscrowMsg, user.wallet);

  if (!result.txhash) {
    console.error(result);
    console.error(`%Error funding escrow for Offer #${offerId}%`);
    throw new Error();
  }

  return result;
}

export { fundTradeEscrow };
