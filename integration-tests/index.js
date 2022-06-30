import dotenv from 'dotenv';
dotenv.config()

import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { DirectSecp256k1HdWallet } from "@cosmjs/proto-signing";
import { GasPrice } from "@cosmjs/stargate";
import * as fs from "fs";
import findFilesInDir from "./findFilesInDir.js";
import { coin, coins } from '@cosmjs/amino';

let rpcEndpoint = "http://localhost:26657";
let maker_seed =
  "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose";

if (process.env.SEED) {
  maker_seed = process.env.SEED;
}
if (process.env.RPC) {
  rpcEndpoint = process.env.RPC;
}

const min_amount = "1";
const max_amount = "10";
const offer_type = "buy";

const gasPrice = GasPrice.fromString(process.env.GAS_PRICE);
const makerWallet = await DirectSecp256k1HdWallet.fromMnemonic(maker_seed, { prefix: process.env.ADDR_PREFIX });
const makerAccounts = await makerWallet.getAccounts();
const makerAddr = makerAccounts[0].address;
console.log('makerAddr', makerAddr);
//const local_denom = { "native": `factory/${makerAddr}/local` }
const local_denom = { "native": process.env.DENOM }

const makerClient = await SigningCosmWasmClient.connectWithSigner(rpcEndpoint, makerWallet, {
  broadcastTimeoutMs: 30 * 1000,
  gasPrice: gasPrice
});

function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function setupProtocol(codeIds) {
  // Instantiate Factory
  const factoryInstantiateMsg = {
    admin_addr: makerAddr,
  };
  console.log('Factory Instantiate - Msg = ', JSON.stringify(factoryInstantiateMsg));
  const result = await makerClient.instantiate(makerAddr, codeIds.factory, factoryInstantiateMsg, "factory", "auto");
  console.log("instantiate factory result = ", result);
  // Instantiate Offer
  console.log('Offer Instantiate');
  const offerInstantiateResult = await makerClient.instantiate(makerAddr, codeIds.offer, {}, "offer", "auto");
  console.log("Instantiate Offer result = ", offerInstantiateResult);
  // Instantiate Trade
  console.log('Trade Instantiate');
  const tradeInstantiateResult = await makerClient.instantiate(makerAddr, codeIds.trade, {}, "trade", "auto");
  console.log("Instantiate Trade result = ", tradeInstantiateResult);
  // Instantiate Trade Incentives
  console.log('Trade Incentives Instantiate');
  const tradeIncentivesInstantiateResult = await makerClient.instantiate(makerAddr, codeIds.trade, {}, "trade", "auto");
  console.log("Instantiate Trade Incentives result = ", tradeIncentivesInstantiateResult);
  // Update Factory config
  const factoryAddress = result.contractAddress
  const updatedConfigMsg = {
    update_config: {
      offer_addr: offerInstantiateResult.contractAddress,
      trade_addr: tradeInstantiateResult.contractAddress,
      trading_incentives_addr: tradeIncentivesInstantiateResult.contractAddress,
      local_market_addr: makerAddr, //TODO: use actual address
      local_denom
    }
  }
  console.log('Factory Update Config - Msg = ', JSON.stringify(updatedConfigMsg));
  const updateFactoryConfigResult = await makerClient.execute(makerAddr, factoryAddress, updatedConfigMsg, "auto")
  console.log("Updated Factory Config result = ", updateFactoryConfigResult);
  console.log("\n");
  return result;
}

async function create_offers(offer_addr) {
  const offers = [
    {
      create: {
        offer: {
          offer_type: "buy",
          fiat_currency: "COP",
          min_amount,
          max_amount,
          rate: "1",
          denom: local_denom,
        },
      },
    },
  ];

  let finalResult;

  for (let idx = 0; idx < offers.length; idx++) {
    const offer = offers[idx];
    console.log(`*Creating Offer ${idx}*`);
    const createOfferResult = await makerClient.execute(makerAddr, offer_addr, offer, "auto");
    console.log(`Created Offer ${idx}:`, createOfferResult);
    finalResult = createOfferResult;
  }
  return finalResult;
}

async function query_offers(offer_addr) {
  const queries = [
    {
      offers_query: {
        limit: 5,
        order: 'desc'
      },
    },
  ];

  const offers = [];
  for (let idx = 0; idx < queries.length; idx++) {
    const query = queries[idx];
    const queryResult = await makerClient.queryContractSmart(offer_addr, query);
    offers.push(queryResult)
  }

  return offers;
}

async function test(codeIds) {
  let factoryCfg;
  let factoryAddr = process.env.FACTORY;
  let tradeAddr;

  let setup = new Promise(async (resolve, reject) => {
    if (factoryAddr) {
      console.log("*Querying Factory Config*");
      const queryResult = await makerClient.queryContractSmart(factoryAddr, {config:{}});
      resolve(queryResult);
    } else {
      console.log("*Setup protocol*");
      console.log('codeIds', codeIds);
      setupProtocol(codeIds).then((r) => {
        const factoryAddr = getAttribute(
          r,
          "instantiate",
          "_contract_address"
        );
        console.log("**Factory Addr:", factoryAddr);

        console.log("*Querying Factory Config*");
        const queryResult = makerClient.queryContractSmart(factoryAddr, {"config":{}});
        resolve(queryResult)
      });
    }
  });
  setup
    .then(async (r) => {
      factoryCfg = r;
      console.log("Factory Config result", r);
      if (process.env.CREATE_OFFERS) {
        await create_offers(factoryCfg.offer_addr);
      }
      return query_offers(factoryCfg.offer_addr);
    }).then(async (r) => {
      //Create Trade
      const newTradeMsg = {
        new_trade: {
          offer_id: r[0][0].id + "",
          amount: min_amount + "",
          taker: makerAddr,
        },
      };
      console.log('new trade msg', JSON.stringify(newTradeMsg));
      console.log("*Creating Trade*");
      return makerClient.execute(makerAddr, factoryCfg.offer_addr, newTradeMsg, "auto");
    }).then((result) => {
      //Accept Trade Request
      console.log("Trade Result:", JSON.stringify(result));
      tradeAddr = result.logs[0].events
        .find((e) => e.type === "instantiate")
        .attributes.find((a) => a.key === "_contract_address").value;
      console.log("**Trade created with address:", tradeAddr);
      console.log("*Accepting Trade Request");
      return makerClient.execute(makerAddr, tradeAddr, {"accept_request":{}}, "auto", "fund_escrow");
    }).then((r) => {
      //Fund Escrow
      console.log("Accept Trade Request Result:", r);
      console.log("*Funding Escrow*");
      console.log('makerAddr:',makerAddr);
      const funds = coins(min_amount, process.env.DENOM);
      return makerClient.execute(makerAddr, tradeAddr, {"fund_escrow":{}}, "auto", "fund_escrow", funds);
    }).then((r) => {
      //Query State
      console.log("Fund escrow result: ", r);
      if (r.transactionHash) {
        console.log("**Escrow Funded**");
      } else {
        console.log("%Error%");
      }
      return makerClient.queryContractSmart(tradeAddr, {"state":{}});
    }).then((r) => {
      //Mark as Paid
      console.log("Trade State:", r);
      console.log("*Marking Trade as Paid*");
      return makerClient.execute(makerAddr, tradeAddr, {"fiat_deposited":{}}, "auto", "release_escrow")
    }).then((r) => {
      //Query State
      console.log("Mark as Paid Result:", r);
      return makerClient.queryContractSmart(tradeAddr, {"state":{}});
    }).then((r) => {
      //Release Escrow
      console.log("Trade State:", r);
      console.log("*Releasing Trade*");
      return makerClient.execute(makerAddr, tradeAddr, {"release_escrow":{}}, "auto", "release_escrow")
    }).then((r) => {
      //Query State
      console.log("Trade Release Result:", r);
      return makerClient.queryContractSmart(tradeAddr, {"state":{}});
    }).then((r) => {
      console.log("Trade State", r);
  });
}

function getContractNameFromPath(path) {
  let regex = RegExp(/artifacts\/(.*?)\.wasm/, "i");
  return path.match(regex)[1];
}

function getCodeIdFromResult(result) {
  return parseInt(getAttribute(result, "store_code", "code_id"));
}

function getAttribute(result, event, attribute) {
  return result.logs[0].events
    .find((e) => e.type === event)
    .attributes.find((e) => e.key === attribute).value;
}

function timeout(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
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
    console.log("Deploy Finished!", JSON.stringify(codeIds));
    await test(codeIds);
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
    console.log("Deploy Finished!", JSON.stringify(codeIds));
    await test(codeIds);
  }
}

async function uploadContract(filePath, addr) {
  const wasm = fs.readFileSync(filePath);
  const uploadResult = await makerClient.upload(addr, wasm, "auto");
  console.log('upload result:', uploadResult);
  await sleep(333);
  return uploadResult.codeId;
}

function fundEscrow(tradeAddr) {
  const coin = Coin.fromData({
    denom: "uusd",
    amount: min_amount + "",
  });
  const coins = new Coins([coin]);
  let fundEscrowMsg = new MsgExecuteContract(
    taker,
    tradeAddr,
    { fund_escrow: {} },
    coins
  );
  console.log("*Funding Escrow*");
  executeMsg(fundEscrowMsg, taker_wallet).then((r) => {
    console.log("Result", r);
    if (r.txhash) {
      release(tradeAddr, taker_wallet);
    }
  });
}

function release(tradeAddr, wallet) {
  console.log("Sending release msg");
  const releaseMsg = new MsgExecuteContract(taker, tradeAddr, {
    release: {},
  });
  console.log("Release Msg:", releaseMsg);
  executeMsg(releaseMsg, wallet).then((r) => {
    r.toJSON().then((r) => console.log(r));
  });
}

if (process.env.DEPLOY) {
  await deploy(process.env.DEPLOY);
} else if (process.env.FUND) {
  fundEscrow(process.env.FUND);
} else if (process.env.RELEASE) {
  release(process.env.RELEASE, taker_wallet);
} else {
  let codeIds = JSON.parse(fs.readFileSync("codeIds.json", "utf8"));
  await test(codeIds);
}
