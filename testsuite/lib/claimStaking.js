import { executeMsg } from "./executeMsg.js";
import { MsgExecuteContract } from "@terra-money/terra.js";

async function claimStaking(client, claim_id, user) {
  const claimStakingMsg = new MsgExecuteContract(
    user.address,
    global.factoryCfg.staking_addr,
    {
      claim: {
        claim_id,
      },
    }
  );

  console.log("claimStakingMsg :>> ", claimStakingMsg);
  const result = await executeMsg(client, claimStakingMsg, user.wallet);

  return result;
}

export { claimStaking };
