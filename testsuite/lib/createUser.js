import { MnemonicKey } from "@terra-money/terra.js";

function createUser(client, mnemonic) {
  const key = new MnemonicKey({ mnemonic });

  const address = key.accAddress;

  const wallet = client.wallet(key);

  // (async () => {
  //   // const accountInfo = await client.auth.accountInfo(address);
  //   // console.log("accountInfo: ", accountInfo);
  //   const balance = await client.bank.balance(address);
  //   console.log("address :>> ", address);
  //   console.log("mnemonic :>> ", mnemonic);
  //   console.log("balance :>> ", balance[0]["_coins"]);
  //   console.log("wallet.lcd.config :>> ", wallet.lcd.config);
  // })();

  return { key, address, wallet };
}

export { createUser };
