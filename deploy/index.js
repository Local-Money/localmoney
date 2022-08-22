import dotenv from 'dotenv';
dotenv.config()

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";
import * as fs from "fs";
import findFilesInDir from "./findFilesInDir.js";

let rpcEndpoint = "http://localhost:26657";
let maker_seed =
  "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

if (process.env.SEED) {
  maker_seed = process.env.SEED;
}
if (process.env.RPC) {
  rpcEndpoint = process.env.RPC;
}

const gasPrice = GasPrice.fromString(process.env.GAS_PRICE);
const makerWallet = await DirectSecp256k1HdWallet.fromMnemonic(maker_seed, { prefix: process.env.ADDR_PREFIX });
const makerAccounts = await makerWallet.getAccounts();
const makerAddr = makerAccounts[0].address;
console.log('makerAddr', makerAddr);

const makerClient = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, makerWallet, {
  broadcastTimeoutMs: 30 * 1000,
  gasPrice: gasPrice
});

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function getContractNameFromPath(path) {
  let regex = RegExp(/artifacts\/(.*?)\.wasm/, "i");
  return path.match(regex)[1];
}

function getAttribute(result, event, attribute) {
  return result.logs[0].events
    .find((e) => e.type === event)
    .attributes.find((e) => e.key === attribute).value;
}

async function deploy(contract) {
  let codeIds = {};
  let contracts = findFilesInDir(process.env.CONTRACTS, ".wasm");

  if (contract.toLowerCase() === "all") {
    for (const i in contracts) {
      let c = contracts[i];
      codeIds[getContractNameFromPath(c)] = await uploadContract(c, makerAddr);
    }
    fs.writeFileSync("codeIds.json", JSON.stringify(codeIds), "utf8");
  } else {
    //Filter by name
    let codeIds = JSON.parse(fs.readFileSync("codeIds.json", "utf8"));
    console.log(codeIds);
    let names;
    if (contract.indexOf(",")) {
      names = contract.split(",");
    } else {
      names = [contract];
    }
    for (const i in names) {
      let name = names[i];
      for (const i in contracts) {
        let c = contracts[i];
        if (c.indexOf(name) >= 0) {
          codeIds[getContractNameFromPath(c)] = await uploadContract(c, makerAddr);
        }
      }
    }
    fs.writeFileSync("codeIds.json", JSON.stringify(codeIds), "utf8");
  }
  console.log("Deploy Finished!", JSON.stringify(codeIds));
}

async function uploadContract(filePath, addr) {
  const wasm = fs.readFileSync(filePath);
  const uploadResult = await makerClient.upload(addr, wasm, "auto");
  console.log('upload result:', uploadResult);
  await sleep(333);
  return uploadResult.codeId;
}


if (process.env.DEPLOY) {
  await deploy(process.env.DEPLOY);
} else {
  console.log('DEPLOY env var is missing.')
  console.log('Please specific which contract to deploy or "all" to deploy all contracts.')
}
