function queryClaims(client, query) {
  try {
    const query = {
      claims: {
        recipient: "terra1vpy55n57punft83gsa9gte7rhruzm9sehav2wp", // TODO make dynamic
      },
    };
    return client.wasm.contractQuery(global.factoryCfg.staking_addr, query);
  } catch (e) {
    console.error(e);
    throw e;
  }
}

export { queryClaims };
