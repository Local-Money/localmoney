import { executeMsg } from "./executeMsg.js";
import { MsgExecuteContract } from "@terra-money/terra.js";

async function enterStaking(client, amount, user) {
  const enterStakingMsg = new MsgExecuteContract(
    user.address,
    process.env.LOCAL_TOKEN_ADDR,
    {
      send: {
        // contract: "terra1l383gt6rt0mpnz2uv5axtvcunlj2096vw043sd", // Staking addr
        contract: global.factoryCfg.staking_addr,
        amount,
        msg: "ewogICJlbnRlciI6IHt9Cn0=", // { Enter: {}}
      },
    }
  );

  console.log("enterStakingMsg :>> ", enterStakingMsg);
  const result = await executeMsg(client, enterStakingMsg, user.wallet);

  return result;
}

export { enterStaking };
