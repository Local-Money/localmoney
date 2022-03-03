function queryOffers(client, query) {
  try {
    return client.wasm.contractQuery(global.factoryCfg.offers_addr, query);
  } catch (e) {
    console.error(e);
    throw e;
  }
}

export { queryOffers };
