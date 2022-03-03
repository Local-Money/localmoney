export async function queryFactoryCfg(client, factoryAddr) {
  console.log("* Querying Factory Config");
  const factoryCfg = await client.wasm.contractQuery(factoryAddr, {
    config: {},
  });

  // console.log("query factoryCfg :>> ", factoryCfg);
  return factoryCfg;
}
