async function queryOffers(client, query) {
  try {
    const result = await client.wasm.contractQuery(
      global.factoryCfg.offers_addr,
      query
    );
    return result;
  } catch (e) {
    if (e.response && e.response.data) console.error(e.response.data);
    else if (e.response) console.error(e.response);
    else console.error(e);
    throw e;
  }
}

export { queryOffers };
