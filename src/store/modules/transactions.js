import {
  Coin,
  Coins,
  LCDClient,
  MnemonicKey,
  MsgExecuteContract,
  StdSignature,
  StdSignMsg,
  StdTx,
} from "@terra-money/terra.js";
import { FACTORY_CONTRACT } from "@/constants";
import router from "@/router";

// create a key out of a mnemonic
let maker_seed, taker_seed;
let network = prompt("1 for terrarium, 2 for bombay")

if (network === "2") {
  maker_seed = 'uncle simple tide bundle apart absurd tenant fluid slam actor caught month hip tornado cattle regular nerve brand tower boy alert crash good neck'
  taker_seed = 'paddle prefer true embody scissors romance train replace flush rather until clap intact hello used cricket limb cake nut permit toss stove cute easily'
} else {
  maker_seed =
    "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius";
  taker_seed =
    "quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty";
}

let takerOrMaker = prompt("1 for maker, 2 for taker").toLowerCase().trim()

let seed = (takerOrMaker === "1") ? maker_seed : taker_seed
const mk = new MnemonicKey({
  mnemonic: seed,
})

const lcdOptions = network === "2" ? {
  URL: 'https://bombay-lcd.terra.dev',
  chainID: 'bombay-12',
} : {
  URL: 'http://143.244.190.1:3060',
  chainID: 'localterra',
}

const terra = new LCDClient(lcdOptions);
const wallet = terra.wallet(mk);
//const ext = new Extension()

const state = {
  offers: [],
  trades: [],
  fiatCurrency: "BRL",
  factoryConfig: {
    trade_code_id: 0,
    token_addr: "",
    local_ust_pool_addr: "",
    gov_addr: "",
    offers_addr: "",
    fee_collector_addr: "",
    trading_incentives_addr: "",
  },
};

// eslint-disable-next-line no-unused-vars
function prepareTransaction(signedMsg) {
  const { public_key, signature, stdSignMsgData } = signedMsg;
  const sig = StdSignature.fromData({
    signature,
    pub_key: {
      type: "tendermint/PubKeySecp256k1",
      value: public_key,
    },
  });

  const stdSignMsg = StdSignMsg.fromData(stdSignMsgData);
  return new StdTx(stdSignMsg.msgs, stdSignMsg.fee, [sig], stdSignMsg.memo);
}

const getters = {
  offers: (state) => state.offers,
  getOfferById: (state) => (id) => {
    return state.offers.find((offer) => offer.id === id);
  },
  trades: (state) => state.trades,
  getTradeInfo: (state) => (tradeAddr) => {
    console.log("getTradeInfo", tradeAddr, state.trades);
    return state.trades.find((tradeInfo) => tradeInfo.trade.addr === tradeAddr);
  },
};

const actions = {
  async fakeWalletConnect({ commit, dispatch }) {
    await commit("setWalletAddress", wallet.key.accAddress);
    dispatch("fetchTrades");
  },
  /**
   * Fetch Factory Contract config
   */
  async fetchFactoryConfig({ commit, dispatch }) {
    const cfgQuery = { config: {} };
    const factoryConfig = await terra.wasm.contractQuery(
      FACTORY_CONTRACT,
      cfgQuery
    );
    commit("setFactoryConfig", factoryConfig);
    dispatch("fetchOffers");
  },
  /**
   * Fetch Offer by Id
   */
  async fetchOffer({ commit }, { id }) {
    const offerQuery = { load_offer: { id } };
    const offer = await terra.wasm.contractQuery(
      state.factoryConfig.offers_addr,
      offerQuery
    );
    commit("addOffer", offer);
  },
  /**
   * Fetch Offers.
   */
  async fetchOffers({ commit }) {
    const offersQuery = { offers: { fiat_currency: state.fiatCurrency } };
    const offers = await terra.wasm.contractQuery(
      state.factoryConfig.offers_addr,
      offersQuery
    );
    commit("setOffers", offers);
  },
  /**
   * Create Offer
   */
  async newOffer({ getters, dispatch }, { offer }) {
    const offerMsg = new MsgExecuteContract(
      getters.walletAddress,
      state.factoryConfig.offers_addr,
      offer
    );
    await executeMsg(offerMsg);
    console.log("fetchOffers");
    dispatch("fetchOffers");
  },
  /**
   * Fetch a specific Trade
   */
  async fetchTrade(
    { commit, getters, dispatch },
    { tradeAddress, redirect = false }
  ) {
    const trade = await terra.wasm.contractQuery(tradeAddress, { config: {} });
    trade.offer = getters.getOfferById(trade.offer_id);
    if (!trade.offer) {
      await dispatch("fetchOffer", { id: trade.offer_id });
    }
    trade.address = tradeAddress;
    trade.offer = getters.getOfferById(trade.offer_id);

    commit("addTrade", trade);
    if (redirect) {
      router.push(`/trade/${tradeAddress}`);
    }
    return trade;
  },
  /**
   * Fetches all trades for given Trader (maker or taker) address.
   */
  async fetchTrades({ commit, getters }) {
    const wallet = getters.walletAddress;
    console.log("wallet: ", wallet);
    const trades = await terra.wasm.contractQuery(
      state.factoryConfig.offers_addr,
      { trades: { maker: wallet } }
    );
    commit("setTrades", trades);
  },
  /**
   * Sends a transaction to instantiate a Trade contract.
   * @param {*} offerId Id of the Offer provided by the Offers Smart Contract.
   * @param {*} amount Amount of UST to be traded.
   */
  // eslint-disable-next-line no-unused-vars
  async openTrade({ getters, dispatch }, { offerId, ustAmount }) {
    console.log("open trade", offerId, ustAmount);

    //let sender = getters.walletAddressess
    let sender = wallet.key.accAddress;
    const amount = parseInt(ustAmount) * 1000000;
    const newTradeMsg = {
      new_trade: {
        offer_id: offerId,
        ust_amount: amount + "",
        counterparty: sender,
      },
    };
    const createTradeMsg = new MsgExecuteContract(
      sender,
      state.factoryConfig.offers_addr,
      newTradeMsg
    );

    //TODO: Error handling.
    await executeMsg(createTradeMsg);
    dispatch("fetchTrades");
  },
  async fundEscrow({ getters, dispatch }, tradeAddr) {
    const tradeInfo = getters.getTradeInfo(tradeAddr)
    const ustAmount = tradeInfo.trade.ust_amount
    const ust = Coin.fromData({ denom: 'uusd', amount: ustAmount })

    const localTerraFee = Coin.fromData({ denom: 'uusd', amount: ustAmount * 0.01 })
    let ltFeeTax = await terra.utils.calculateTax(localTerraFee)
    let releaseTax = await terra.utils.calculateTax(ust)
    ltFeeTax = parseInt(ltFeeTax.toData().amount)
    releaseTax = parseInt(releaseTax.toData().amount)

    //TODO: issue with diveregence between cosmwasm and terrajs posted on tg channel, awaiting response. Adding 1UST
    let oneUST = 1000000;
    let fundEscrowAmount = parseInt(ustAmount) + parseInt(localTerraFee.amount) + ltFeeTax + releaseTax + oneUST;
    console.log('fundEscrowAmount', fundEscrowAmount)
    fundEscrowAmount = Coin.fromData({ denom: 'uusd', amount: fundEscrowAmount })
    const coins = new Coins([fundEscrowAmount])
    const fundMsg = {"fund_escrow":{}}
    const fundEscrowMsg = new MsgExecuteContract(getters.walletAddress, tradeAddr, fundMsg, coins)
    await executeMsg(fundEscrowMsg)
    dispatch('fetchTrade', { tradeAddress: tradeAddr })
  },
  async releaseEscrow({ getters, dispatch }, tradeAddr) {
    const releaseMsg = new MsgExecuteContract(
      getters.walletAddress,
      tradeAddr,
      { release: {} }
    );
    const result = await executeMsg(releaseMsg);
    //TODO: Error handling
    console.log("Released", result);
    dispatch("fetchTrade", { tradeAddress: tradeAddr });
  },
  async refundEscrow({ getters, dispatch }, tradeAddr) {
    const refundMsg = new MsgExecuteContract(getters.walletAddress, tradeAddr, {
      refund: {},
    });
    const result = await executeMsg(refundMsg);
    //TODO: Error handling
    console.log("Refunded", result);
    dispatch("fetchTrade", { tradeAddress: tradeAddr });
  },
};

async function executeMsg(msg) {
  return wallet
    .createAndSignTx({
      msgs: [msg],
    })
    .then((tx) => terra.tx.broadcast(tx))
    .then((result) => {
      return result;
    });

  /*
  ext.once('onSign', async (res) => {
    if (res.result) {
      const signedTx = prepareTransaction(res.result)
      await terra.tx.broadcast(signedTx)
      cb()
    } else {
      alert('Error')
    }
  })
  ext.sign({ msgs: [msg] })
   */
}

const mutations = {
  setFactoryConfig: (state, factoryConfig) =>
    (state.factoryConfig = factoryConfig),
  addOffer: (state, offer) => state.offers.push(offer),
  setOffers: (state, offers) => (state.offers = offers),
  addTrade: (state, trade) => {
    const addedTrade = state.trades.find((t) => t.address == trade.address);
    if (addedTrade) {
      state.trades[state.trades.indexOf(addedTrade)] = trade;
    } else {
      state.trades.push(trade);
    }
  },
  setTrades: (state, trades) => {
    state.trades = trades;
  },
};

export default {
  state,
  getters,
  actions,
  mutations,
};
