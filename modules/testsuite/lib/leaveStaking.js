import { executeMsg } from "./executeMsg.js";
import { MsgExecuteContract } from "@terra-money/terra.js";

async function leaveStaking(client, amount, user) {
  const leaveStakingMsg = new MsgExecuteContract(
    user.address,
    global.factoryCfg.xlocal_addr,
    {
      send: {
        contract: global.factoryCfg.staking_addr,
        amount,
        msg: "ewogICJsZWF2ZSI6IHt9Cn0=", // {Leave:{}}
      },
    }
  );

  console.log("leaveStakingMsg :>> ", leaveStakingMsg);
  const result = await executeMsg(client, leaveStakingMsg, user.wallet);

  return result;
}

export { leaveStaking };
