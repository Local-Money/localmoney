import { MsgExecuteContract } from "@terra-money/terra.js";
import { executeMsg } from "./executeMsg.js";

async function createTrade(client, trade, user) {
  const createTradeMsg = new MsgExecuteContract(
    user.address,
    global.factoryCfg.offers_addr,
    {
      new_trade: trade,
    }
  );

  const result = await executeMsg(client, createTradeMsg, user.wallet);

  const tradeAddr = result.logs[0].events
    .find((e) => e.type === "instantiate_contract")
    .attributes.find((a) => a.key === "contract_address").value;

  return tradeAddr;
}

export { createTrade };
