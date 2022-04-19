import {
  Coin,
  Coins, Extension,
  LCDClient,
  MsgExecuteContract,
  StdSignature,
  StdSignMsg,
  StdTx,
} from "@terra-money/terra.js";
import {FACTORY_CONTRACT} from "@/constants";
import router from "@/router";
import {updateTrade} from "@/store/firebase"
import {newTrade} from "../firebase";

const lcdOptions = {
  URL: 'http://143.244.190.1:3060',
  chainID: 'localterra',
}
let terra = new LCDClient(lcdOptions);
const ext = new Extension()

const state = {
  loading: {
    isLoading: false,
    label: "Processing...",
    transaction: "Follow the transaction"
  },
  walletAddress: '',
  offers: [],
  tradeInfos: [],
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
  const {public_key, signature, stdSignMsgData} = signedMsg;
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
  trades: (state) => state.tradeInfos,
  getTradeInfo: (state) => (tradeAddr) => {
    return state.tradeInfos.find((tradeInfo) => tradeInfo.trade.addr === tradeAddr);
  },
  lunaUstPrice: (state) => state.lunaUstPrice,
  ustUsdPrice: (state) => state.ustUsdPrice,
  loading: (state) => state.loading,
};

const actions = {
  async initWallet({commit, dispatch}) {
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
  },
  /**
   * Fetch Factory Contract config
   */
  async fetchFactoryConfig({commit, dispatch}) {
    const cfgQuery = {config: {}};
    const factoryConfig = await terra.wasm.contractQuery(
      FACTORY_CONTRACT,
      cfgQuery
    );
    commit("setFactoryConfig", factoryConfig);
    dispatch("fetchTradeInfos")
  },
  /**
   * Fetch Offer by Id
   */
  async fetchOffer({commit}, {id}) {
    const offerQuery = {offer: {id}};
    const offer = await terra.wasm.contractQuery(
      state.factoryConfig.offers_addr,
      offerQuery
    );
    commit("addOffer", offer);
  },
  /**
   * Fetch Offers.
   */
  async fetchOffers({commit, getters}, {fiatCurrency, offerType, paginated = false}) {
    const offers = paginated ? getters.offers : []
    const last_offer_id = offers.length > 0 && paginated ? offers[offers.length -1].id : 0

    const offersQuery = {
      offers_by_type_fiat: {
        fiat_currency: fiatCurrency,
        offer_type: offerType,
        last_value: last_offer_id,
        limit: 10
      }
    };
    const loadedOffers = await terra.wasm.contractQuery(
      state.factoryConfig.offers_addr,
      offersQuery
    );
    commit("setOffers", offers.concat(loadedOffers));
  },
  /**
   * Create Offer
   */
  async newOffer({commit, getters, dispatch}, {offer}) {
    const offerMsg = new MsgExecuteContract(
      getters.walletAddress,
      state.factoryConfig.offers_addr,
      offer
    );

    await executeMsg(commit, getters, dispatch, offerMsg);
    router.push(`/`);
  },
  /**
   * Fetch a specific Trade
   */
  async fetchTradeInfo({commit, getters, dispatch}, {addr, tradeData}) {
    const tradeInfo = {}
    const trade = await terra.wasm.contractQuery(addr, {state: {}});
    tradeInfo.trade = trade

    tradeInfo.offer = getters.getOfferById(trade.offer_id);
    if (!tradeInfo.offer) {
      await dispatch("fetchOffer", {id: trade.offer_id});
      tradeInfo.offer = getters.getOfferById(trade.offer_id);
    }

    if (tradeData) {
      Object.assign(tradeInfo.trade, tradeData)
    }

    commit("addTradeInfo", tradeInfo);

    return tradeInfo;
  },
  /**
   * Fetches all trades for given Trader (maker or taker) address.
   */
  async fetchTradeInfos({commit, getters}, redirect = false) {
    const wallet = getters.walletAddress;
    const trades_as_seller = await terra.wasm.contractQuery(
      state.factoryConfig.offers_addr, {trades_query: {user: wallet, index: "seller", limit: 10}}
    );
    const trades_as_buyer = await terra.wasm.contractQuery(
      state.factoryConfig.offers_addr, {trades_query: {user: wallet, index: "buyer", limit: 10}}
    );

    const trades = trades_as_buyer.concat(trades_as_seller);

    commit("setTradeInfos", trades);
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
  async openTrade({commit, getters, dispatch}, {offer, ustAmount}) {
    let sender = getters.walletAddress
    const amount = ustAmount * 1000000;
    const newTradeMsg = {
      new_trade: {
        offer_id: offer.id,
        ust_amount: amount + "",
        counterparty: sender,
        taker: sender, //TODO
        taker_contact: "@TODO",
        arbitrator: sender, //TODO
      },
    };
    const createTradeMsg = new MsgExecuteContract(
      sender,
      state.factoryConfig.offers_addr,
      newTradeMsg
    );

    //TODO: Error handling.
    await executeMsg(commit, getters, dispatch, createTradeMsg);
    dispatch("fetchTradeInfos", true);
    newTrade(offer.owner, newTradeMsg)
  },
  async fundEscrow({commit, getters, dispatch}, tradeAddr) {
    let tradeInfo = getters.getTradeInfo(tradeAddr)
    const ustAmount = tradeInfo.trade.ust_amount
    const ust = Coin.fromData({denom: 'uusd', amount: ustAmount})

    const localTerraFee = Coin.fromData({denom: 'uusd', amount: ustAmount * 0.01})
    let ltFeeTax = await terra.utils.calculateTax(localTerraFee)
    let releaseTax = await terra.utils.calculateTax(ust)
    ltFeeTax = parseInt(ltFeeTax.toData().amount)
    releaseTax = parseInt(releaseTax.toData().amount)

    //TODO: issue with diveregence between cosmwasm and terrajs posted on tg channel, awaiting response. Adding 1UST
    let oneUST = 1000000;
    let fundEscrowAmount = parseInt(ustAmount) + parseInt(localTerraFee.amount) + ltFeeTax + releaseTax + oneUST;
    fundEscrowAmount = Coin.fromData({denom: 'uusd', amount: fundEscrowAmount})
    const coins = new Coins([fundEscrowAmount])
    const fundMsg = {"fund_escrow": {}}
    const fundEscrowMsg = new MsgExecuteContract(getters.walletAddress, tradeAddr, fundMsg, coins)
    await executeMsg(commit, getters, dispatch, fundEscrowMsg)

    tradeInfo = await dispatch("fetchTradeInfo", {addr: tradeAddr})
    await updateTrade(tradeInfo.trade)
  },
  async releaseEscrow({commit, getters, dispatch}, tradeAddr) {
    const releaseMsg = new MsgExecuteContract(
      getters.walletAddress,
      tradeAddr,
      {release_escrow: {}}
    );
    await executeMsg(commit, getters, dispatch, releaseMsg);
    //TODO: Error handling
    let tradeInfo = await dispatch("fetchTradeInfo", {addr: tradeAddr});
    await updateTrade(tradeInfo.trade)
  },
  async acceptTradeRequest({commit, getters, dispatch}, tradeAddr) {
    const fiatDeposited = new MsgExecuteContract(getters.walletAddress, tradeAddr, {
      accept_request: {},
    });
    await executeMsg(commit, getters, dispatch, fiatDeposited);
    let tradeInfo = await dispatch("fetchTradeInfo", {addr: tradeAddr});
    await updateTrade(tradeInfo.trade)
  },
  async setFiatDeposited({commit, getters, dispatch}, tradeAddr) {
    const fiatDeposited = new MsgExecuteContract(getters.walletAddress, tradeAddr, {
      fiat_deposited: {},
    });
    await executeMsg(commit, getters, dispatch, fiatDeposited);
    let tradeInfo = await dispatch("fetchTradeInfo", {addr: tradeAddr});
    await updateTrade(tradeInfo.trade)
  },
  async refundEscrow({commit, getters, dispatch}, tradeAddr) {
    const refundMsg = new MsgExecuteContract(getters.walletAddress, tradeAddr, {
      refund: {},
    });
    await executeMsg(commit, getters, dispatch, refundMsg);
    let tradeInfo = await dispatch("fetchTradeInfo", {addr: tradeAddr});
    await updateTrade(tradeInfo.trade)
  },
  async fetchLunaPrice({commit}) {
    const res = await fetch(`${lcdOptions.URL}/v1/market/swaprate/uluna`)
    const priceData = await res.json()
    const lunaUstPrice = priceData.find(p => p.denom === "uusd").swaprate
    commit('setLunaUstPrice', parseFloat(lunaUstPrice).toFixed(2))
  },
  async fetchUstUsdPrice({commit}) {
    const res = await fetch("https://api.coinpaprika.com/v1/tickers/ust-terrausd?quotes=USD")
    const ustPriceData = await res.json()
    const ustUsdPrice = ustPriceData.quotes["USD"].price
    commit('setUstUsdPrice', ustUsdPrice.toFixed(2))
  },

  // @TODO delete this method
  async setTradeAsPaid({commit}, {tradeAddr, paid}) {
    const tradeInfoIdx = state.tradeInfos.findIndex((t) => t.trade.addr === tradeAddr);
    state.tradeInfos[tradeInfoIdx].trade.paid = paid
    //TODO: create method to update a single tradeInfo
    await commit("setTradeInfos", state.tradeInfos)
  },
};

async function executeMsg(commit, getters, dispatch, msg) {
  if (getters.walletAddress === "") {
    dispatch('initWallet')
    return
  }
  return new Promise((resolve) => {
    ext.once('onPost', async (res) => {
      if (res.success) {
        commit("setLoadingTransaction", res.result.txhash)
        commit("setIsLoading", true)
      }
      let interval = setInterval(async () => {
        console.log('res', res);
        let txInfo = await terra.tx.txInfo(res.result.txhash)
        if (txInfo) {
          resolve(txInfo)
          clearInterval(interval)
          commit("setIsLoading", false)
        }
      }, 1000)
    })
    ext.post({
      msgs: [msg]
    })
    /*
    //Suddenly stopped working (at least on Terrarium, needs to be tested on TestNet, MainNet.
    //Or hopefully we'll be able to use the "auto" fee option.
    terra.tx.estimateFee(getters.walletAddress, [msg]).then((stdFee) => {
      ext.post({
        fee: stdFee,
        msgs: [msg]
      })
    })
     */
  })
}

const mutations = {
  setWalletAddress: (state, walletAddress) => (state.walletAddress = walletAddress),
  setFactoryConfig: (state, factoryConfig) =>
    (state.factoryConfig = factoryConfig),
  addOffer: (state, offer) => state.offers.push(offer),
  setOffers: (state, offers) => (state.offers = offers),
  addTradeInfo: (state, tradeInfo) => {
    const addedTradeInfo = state.tradeInfos.find((t) => t.trade.addr === tradeInfo.trade.addr);
    if (addedTradeInfo) {
      Object.assign(addedTradeInfo, tradeInfo)
    } else {
      state.tradeInfos.push(tradeInfo);
    }
    state.tradeInfos = [...state.tradeInfos]
  },
  setIsLoading: (state, isLoading) => {
    state.loading.isLoading = isLoading;
  },
  setLoadingLabel: (state, label) => {
    state.loading.label = label;
  },
  setLoadingTransaction: (state, transaction) => {
    state.loading.transaction = transaction;
  },
  setTradeInfos: (state, tradeInfos) => {
    state.tradeInfos = [...tradeInfos];
  },
  setLunaUstPrice: (state, price) => state.lunaUstPrice = price,
  setUstUsdPrice: (state, price) => state.ustUsdPrice = price,
};

export default {
  state,
  getters,
  actions,
  mutations,
};
