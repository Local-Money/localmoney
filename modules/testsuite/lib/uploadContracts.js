import findFilesInDir from "./_findFilesInDir.js";
import { sleep } from "./sleep.js";
import { executeMsg } from "./executeMsg.js";
import { _getAttribute } from "./_getAttribute.js";

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
  const codeIds = {};

  const contractList = findFilesInDir("../contracts/artifacts/", ".wasm");

  for (const idx in contractList) {
    const path = contractList[idx];

    const storeMsg = _createStoreMsg(path, user);

    console.log(`*Storing ${path}*`);

    try {
      const result = await executeMsg(client, storeMsg, user.wallet);
      // console.log("result:>> ", result);
      codeIds[_getContractNameFromPath(path)] = _getCodeIdFromResult(result);
      await sleep(1000);
    } catch (err) {
      console.error(err.message);
      throw err;
    }
  }
  fs.writeFileSync(
    `cache/codeIds_${process.env.NETWORK_ENV}.json`,
    JSON.stringify(codeIds),
    "utf8"
  );
  console.log("Deploy Finished!", JSON.stringify(codeIds));
}

export { uploadContracts };
