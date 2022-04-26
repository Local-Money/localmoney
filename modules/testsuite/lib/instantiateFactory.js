import { executeMsg } from "./executeMsg.js";
import { _getAttribute } from "./_getAttribute.js";
import { queryFactoryCfg } from "./queryFactoryCfg.js";

import { MsgInstantiateContract } from "@terra-money/terra.js";

import { writeFileSync, readFileSync } from "fs";

function _storeFactory(factory) {
  writeFileSync(
    `cache/factory_${process.env.NETWORK_ENV}.json`,
    JSON.stringify(factory),
    "utf8"
  );
}

async function instantiateFactory(client, user, options = {}) {
  console.log("* Instantiate Factory as", user.address);
  let codeIds;

  try {
    codeIds = JSON.parse(
      readFileSync(`cache/codeIds_${process.env.NETWORK_ENV}.json`, "utf8")
    );
  } catch ({ message }) {
    console.error(message);
    console.log("-> First upload the contracts: npm run upload");
    throw new Error(message);
  }

  const factoryInstantiateMsg = {
    cw20_code_id: parseInt(process.env.CW20_CODE_ID),
    gov_contract_code_id: codeIds.governance,
    trading_incentives_code_id: codeIds.trading_incentives,
    offer_code_id: codeIds.offer,
    trade_code_id: codeIds.trade,
    local_ust_pool_addr: user.address,
    staking_code_id: codeIds.localterra_staking,
    local_token_addr: process.env.LOCAL_TOKEN_ADDR,
  };

  const instantiateFactoryMsg = new MsgInstantiateContract(
    user.address,
    user.address,
    codeIds.factory,
    factoryInstantiateMsg
  );

  // console.log("instantiateFactoryMsg :>> ", instantiateFactoryMsg);
  let result;
  try {
    result = await executeMsg(client, instantiateFactoryMsg, user.wallet);
    // console.log("result instantiateFactoryMsg :>> ", result, user.address); // If this fails make sure your wallet has LUNA to pay the fees
  } catch (err) {
    console.log(err);
    throw err;
  }
  const factoryAddr = _getAttribute(
    result,
    "instantiate_contract",
    "contract_address"
  );
  console.log("* Factory Addr:", factoryAddr);

  const factory = { factoryAddr, factoryCfg: {} };
  if (options.cache) _storeFactory(factory);

  factory.factoryCfg = await queryFactoryCfg(client, factoryAddr);

  if (options.cache) _storeFactory(factory);

  console.log("factory :>> ", factory);

  return factory;
}

export { instantiateFactory };
