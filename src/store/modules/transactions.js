import {
  Coin,
  Coins, Extension,
  LCDClient,
  MsgExecuteContract,
  StdSignature,
  StdSignMsg,
  StdTx,
} from "@terra-money/terra.js";
import { FACTORY_CONTRACT } from "@/constants";
import router from "@/router";
import { updateTrade } from "@/store/firebase"
import { newTrade } from "../firebase";

const lcdOptions = {
  URL: 'http://143.244.190.1:3060',
  chainID: 'localterra',
}
let terra = new LCDClient(lcdOptions);
const ext = new Extension()

const state = {
  walletAddress: '',
  offers: [],
  trades: [],
  fiatCurrency: "BRL",
  lunaUstPrice: 0,
  ustUsdPrice: 0,
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
  console.log('signedMsg', signedMsg)
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
  walletAddress: (state) => state.walletAddress,
  offers: (state) => state.offers,
  getOfferById: (state) => (id) => {
    return state.offers.find((offer) => offer.id === id);
  },
  trades: (state) => state.trades,
  getTradeInfo: (state) => (tradeAddr) => {
    return state.trades.find((tradeInfo) => tradeInfo.trade.addr === tradeAddr);
  },
  lunaUstPrice: (state) => state.lunaUstPrice,
  ustUsdPrice: (state) => state.ustUsdPrice
};

const actions = {
  async initWallet({ commit, dispatch }) {
    const ext = new Extension()
    const res = await ext.request('connect')
    const info = await ext.request('info')
    terra = new LCDClient({
      URL: info.payload.lcd,
      chainID: info.payload.chainID
    })
    const walletAddress = res.payload.address
    commit('setWalletAddress', walletAddress)
    dispatch('fetchFactoryConfig')
    dispatch('fetchTrades')
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
    await executeMsg(getters, dispatch, offerMsg);
    dispatch("fetchOffers");
  },
  /**
   * Fetch a specific Trade
   */
  async fetchTrade(
    { commit, getters, dispatch },
    tradeAddress, redirect = false
  ) {
    const trade = await terra.wasm.contractQuery(tradeAddress, { state: {} });
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
  async fetchTrades({ commit, getters }, redirect = false) {
    const wallet = getters.walletAddress;
    const trades = await terra.wasm.contractQuery(
      state.factoryConfig.offers_addr, { trades: { trader: wallet } }
    );
    commit("setTrades", trades);
    if (redirect) {
      router.push('/trades')
    }
  },
  /**
   * Sends a transaction to instantiate a Trade contract.
   * @param {*} offerId Id of the Offer provided by the Offers Smart Contract.
   * @param {*} amount Amount of UST to be traded.
   */
  // eslint-disable-next-line no-unused-vars
  async openTrade({ getters, dispatch }, { offer, ustAmount }) {
    let sender = getters.walletAddress
    const amount = ustAmount * 1000000;
    const newTradeMsg = {
      new_trade: {
        offer_id: offer.id,
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
    await executeMsg(getters, dispatch, createTradeMsg);
    dispatch("fetchTrades", true);
    newTrade(offer.owner, newTradeMsg)
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
    fundEscrowAmount = Coin.fromData({ denom: 'uusd', amount: fundEscrowAmount })
    const coins = new Coins([fundEscrowAmount])
    const fundMsg = {"fund_escrow":{}}
    const fundEscrowMsg = new MsgExecuteContract(getters.walletAddress, tradeAddr, fundMsg, coins)
    await executeMsg(getters, dispatch, fundEscrowMsg)
    let trade = await dispatch('fetchTrade', tradeAddr)
    updateTrade(trade)
  },
  async releaseEscrow({ getters, dispatch }, tradeAddr) {
    const releaseMsg = new MsgExecuteContract(
      getters.walletAddress,
      tradeAddr,
      { release: {} }
    );
    await executeMsg(getters, dispatch, releaseMsg);
    //TODO: Error handling
    let trade = await dispatch("fetchTrade", tradeAddr);
    updateTrade(trade)
  },
  async refundEscrow({ getters, dispatch }, tradeAddr) {
    const refundMsg = new MsgExecuteContract(getters.walletAddress, tradeAddr, {
      refund: {},
    });
    await executeMsg(getters, dispatch, refundMsg);
    dispatch("fetchTrade", tradeAddr);
  },
  async fetchLunaPrice({ commit }) {
    const res = await fetch(`${lcdOptions.URL}/v1/market/swaprate/uluna`)
    const priceData = await res.json()
    const lunaUstPrice = priceData.find(p => p.denom === "uusd").swaprate
    commit('setLunaUstPrice', parseFloat(lunaUstPrice).toFixed(2))
  },
  async fetchUstUsdPrice({ commit }) {
    const res = await fetch("https://api.coinpaprika.com/v1/tickers/ust-terrausd?quotes=USD")
    const ustPriceData = await res.json()
    const ustUsdPrice = ustPriceData.quotes["USD"].price
    commit('setUstUsdPrice', ustUsdPrice.toFixed(2))
  }
};

async function executeMsg(getters, dispatch, msg) {
  if (getters.walletAddress === "") {
    dispatch('initWallet')
    return
  }
  return new Promise((resolve) => {
    ext.once('onPost', async (res) => {
      console.log('post tx result', res)
      let interval = setInterval(async () => {
        let txInfo = await terra.tx.txInfo(res.result.txhash)
        if (txInfo) {
          resolve(txInfo)
          clearInterval(interval)
        }
      }, 1000)
    })
    terra.tx.estimateFee(getters.walletAddress, [msg]).then((stdFee) => {
      ext.post({
        fee: stdFee,
        msgs: [msg]
      })
    })
  })
}

const mutations = {
  setWalletAddress: (state, walletAddress) => (state.walletAddress = walletAddress),
  setFactoryConfig: (state, factoryConfig) =>
    (state.factoryConfig = factoryConfig),
  addOffer: (state, offer) => state.offers.push(offer),
  setOffers: (state, offers) => (state.offers = offers),
  addTrade: (state, trade) => {
    const addedTrade = state.trades.find((t) => t.trade.addr === trade.addr);
    if (addedTrade) {
      state.trades[state.trades.indexOf(addedTrade)].trade = trade;
      state.trades = [...state.trades]
    } else {
      state.trades.push(trade);
      state.trades = [...state.trades]
    }
  },
  setTrades: (state, trades) => {
    state.trades = [...trades];
  },
  setLunaUstPrice: (state, price) => state.lunaUstPrice = price,
  setUstUsdPrice: (state, price) => state.ustUsdPrice= price,
};

export default {
  state,
  getters,
  actions,
  mutations,
};
