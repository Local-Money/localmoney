import findFilesInDir from "./_findFilesInDir.js";
import { sleep } from "./sleep.js";
import { executeMsg } from "./executeMsg.js";
import { _getAttribute } from "./_getAttribute.js";
import { findNewContractFiles } from "./findNewContractFiles.js";
import { _readCachedContractChecksums } from "./_readCachedContractChecksums.js";
import { _readCachedCodeIds } from "./_readCachedCodeIds.js";
import { _writeCachedContractChecksums } from "./_writeCachedContractChecksums.js";

import { MsgStoreCode } from "@terra-money/terra.js";
import * as fs from "fs";

function _createStoreMsg(contract, user) {
  const wasm = fs.readFileSync(contract, {
    highWaterMark: 16,
    encoding: "base64",
  });
  return new MsgStoreCode(user.address, wasm);
}

function _getContractNameFromPath(path) {
  let regex = RegExp(/artifacts\/(.*?)\.wasm/, "i");
  return path.match(regex)[1];
}

function _getCodeIdFromResult(result) {
  return parseInt(_getAttribute(result, "store_code", "code_id"));
}

async function uploadContracts(client, user) {
  const contractPath = "../contracts/artifacts/";

  const codeIds = _readCachedCodeIds();

  const cachedContractChecksums = _readCachedContractChecksums();

  const newContractList = findNewContractFiles(contractPath); // findFilesInDir("../contracts/artifacts/", ".wasm");

  if (newContractList.length > 0)
    console.log("** Found new contracts:", newContractList);

  for (const idx in newContractList) {
    const [contract, checksum] = newContractList[idx];

    const path = contractPath + contract;

    const storeMsg = _createStoreMsg(path, user);

    console.log(`*Storing ${path}*`);
    try {
      const result = await executeMsg(client, storeMsg, user.wallet);

      // update contract checksum cache so we don't need to upload it again if another contract upload times out
      cachedContractChecksums[contract] = checksum;
      _writeCachedContractChecksums(cachedContractChecksums);

      // update codeIds cached so we can instantiate the contract
      codeIds[_getContractNameFromPath(path)] = _getCodeIdFromResult(result);
      fs.writeFileSync(
        `cache/codeIds_${process.env.NETWORK_ENV}.json`,
        JSON.stringify(codeIds),
        "utf8"
      );

      await sleep(5000); // Wait for blockchain propagation to avoid exiting with error
    } catch (err) {
      console.error(err.message);
      throw err;
    }
  }

  console.log(
    "Uploading contract wasm files finished!",
    JSON.stringify(codeIds)
  );
}

export { uploadContracts };
