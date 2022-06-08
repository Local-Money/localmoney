async function queryStakingTotals(client, query) {
  try {
    const queryTotalShares = {
      total_shares: {},
    };
    const resultTotalShares = await client.wasm.contractQuery(
      global.factoryCfg.staking_addr,
      queryTotalShares
    );

    console.log("resultTotalShares :>> ", resultTotalShares);

    const queryTotalDeposit = {
      total_deposit: {},
    };
    const resultTotalDeposit = await client.wasm.contractQuery(
      global.factoryCfg.staking_addr,
      queryTotalDeposit
    );

    console.log("resultTotalDeposit :>> ", resultTotalDeposit);
    const queryWarming = {
      total_warming: {},
    };
    const resultTotalWarming = await client.wasm.contractQuery(
      global.factoryCfg.staking_addr,
      queryWarming
    );
    console.log("resultTotalWarming :>> ", resultTotalWarming);

    return {
      resultTotalShares,
      resultTotalDeposit,
      resultTotalWarming,
    };
  } catch (e) {
    console.error(e);
    throw e;
  }
}

export { queryStakingTotals };
