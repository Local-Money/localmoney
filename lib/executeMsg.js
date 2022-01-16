async function executeMsg(client, msg, wallet) {
  try {
    const tx = await wallet.createAndSignTx({
      msgs: [msg],
    });

    const result = await client.tx.broadcast(tx);

    if (process.env.LOG_RESULT) console.log("result :>> ", result);

    return result;
  } catch (err) {
    if (err.response && err.response.data) {
      console.error("ERROR:", err.response.data);
    } else {
      console.error("ERROR:", err);
    }

    throw err;
  }
}

export { executeMsg };
