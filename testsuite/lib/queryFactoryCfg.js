export async function queryFactoryCfg(client, factoryAddr) {
  console.log("* Querying Factory Config");
  const factoryCfg = await client.wasm.contractQuery(factoryAddr, {
    config: {},
  });
  const stakingCfg = await client.wasm.contractQuery(factoryCfg.staking_addr, {
    config: {},
  });

  factoryCfg.xlocal_addr = stakingCfg.share_token_addr;

  console.log("query factoryCfg :>> ", factoryCfg);
  return factoryCfg;
}
