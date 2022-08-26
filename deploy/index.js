import dotenv from 'dotenv';
dotenv.config({ path: '../app/.env' })

import {SigningCosmWasmClient} from "@cosmjs/cosmwasm-stargate";
import {DirectSecp256k1HdWallet} from "@cosmjs/proto-signing";
import {GasPrice} from "@cosmjs/stargate";
import * as fs from "fs";
import findFilesInDir from "./findFilesInDir.js";

let rpcEndpoint = "http://localhost:26657";
let seed =
  "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

if (process.env.ADMIN_SEED) {
  seed = process.env.ADMIN_SEED;
}
if (process.env.RPC) {
  rpcEndpoint = process.env.RPC;
}

const gasPrice = GasPrice.fromString(process.env.GAS_PRICE);
const wallet = await DirectSecp256k1HdWallet.fromMnemonic(seed, {prefix: process.env.ADDR_PREFIX});
const accounts = await wallet.getAccounts();
const walletAddr = accounts[0].address;
const codeIdsPath = '../app/tests/fixtures/codeIds.json'
console.log('Wallet Address:', walletAddr);

const cwClient = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, wallet, {
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

async function deploy(contract) {
  let codeIds = {};
  let contracts = findFilesInDir(process.env.CONTRACTS, ".wasm");

  if (contract.toLowerCase() === "all") {
    for (const i in contracts) {
      let c = contracts[i];
      codeIds[getContractNameFromPath(c)] = await uploadContract(c, walletAddr);
    }
    fs.writeFileSync(codeIdsPath, JSON.stringify(codeIds), "utf8");
  } else {
    //Filter by name
    let codeIds = JSON.parse(fs.readFileSync(codeIdsPath, "utf8"));
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
          codeIds[getContractNameFromPath(c)] = await uploadContract(c, walletAddr);
        }
      }
    }
    fs.writeFileSync(codeIdsPath, JSON.stringify(codeIds), "utf8");
  }
  console.log("Deploy Finished!", JSON.stringify(codeIds));
}

async function uploadContract(filePath, addr) {
  const wasm = fs.readFileSync(filePath);
  const uploadResult = await cwClient.upload(addr, wasm, "auto");
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
