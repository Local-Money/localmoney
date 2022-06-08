function queryTrades(client, trade_addr, query) {
  try {
    return client.wasm.contractQuery(trade_addr, query);
  } catch (e) {
    console.error(e);
    throw e;
  }
}

export { queryTrades };
