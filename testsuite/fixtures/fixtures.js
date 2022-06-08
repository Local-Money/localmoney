import dotenv from "dotenv";
dotenv.config({ path: `./.${process.env.NETWORK_ENV}.env` });

import { createLCDClient } from "../lib/createLCDClient.js";
import { createUser } from "../lib/createUser.js";
import { uploadContracts } from "../lib/uploadContracts.js";
import { instantiateFactory } from "../lib/instantiateFactory.js";

export const mochaGlobalSetup = async () => {
  if (process.env.FACTORY_ADDR) {
    // run against existing live factory, don't bother uploading or initiating
    console.log(
      "*** RUNNING TEST AGAINST EXISTING FACTORY_ADDR: ",
      process.env.FACTORY_ADDR
    );
    return;
  }

  if (process.env.DIRTY_RUN)
    console.log(
      "*** THIS IS A DIRTY RUN, FACTORY WILL NOT BE INSTANTIATED FOR EACH TEST !!"
    );

  // Don't do anything if there is nothing to do
  if (!process.env.UPLOAD) return;

  const terra = await createLCDClient();

  const arbitrator = createUser(terra, process.env.ARBITRATOR_MNEMONIC);

  if (process.env.UPLOAD) await uploadContracts(terra, arbitrator);
  if (process.env.INSTANTIATE)
    await instantiateFactory(terra, arbitrator, { cache: true });
};
