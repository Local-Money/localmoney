import dotenv from "dotenv";
dotenv.config({ path: `./.${process.env.NETWORK_ENV}.env` });

import { createLCDClient } from "../lib/createLCDClient.js";
import { createUser } from "../lib/createUser.js";
import { uploadContracts } from "../lib/uploadContracts.js";
import { instantiateFactory } from "../lib/instantiateFactory.js";

export const deploy = async () => {
  const terra = await createLCDClient();

  const arbitrator = createUser(terra, process.env.ARBITRATOR_MNEMONIC);

  await uploadContracts(terra, arbitrator);
  await instantiateFactory(terra, arbitrator, { cache: true });
};

await deploy();
