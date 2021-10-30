import {LCDClient, MnemonicKey, MsgExecuteContract, StdSignature, StdSignMsg, StdTx,} from '@terra-money/terra.js'
import { FACTORY_CONTRACT } from '@/constants'
import router from '@/router'

// create a key out of a mnemonic
let maker_seed = 'uncle simple tide bundle apart absurd tenant fluid slam actor caught month hip tornado cattle regular nerve brand tower boy alert crash good neck'
// eslint-disable-next-line no-unused-vars
let taker_seed = 'paddle prefer true embody scissors romance train replace flush rather until clap intact hello used cricket limb cake nut permit toss stove cute easily'

const mk = new MnemonicKey({
  mnemonic: maker_seed,
})

const terra = new LCDClient({
  URL: 'https://bombay-lcd.terra.dev',
  chainID: 'bombay-12',
})

const wallet = terra.wallet(mk)
//const ext = new Extension()

const state = {
  offers: [],
  trades: [],
  fiatCurrency: 'BRL',
  factoryConfig: {
    trade_code_id: 0,
    token_addr: "",
    local_ust_pool_addr: "",
    gov_addr: "",
    offers_addr: "",
    fee_collector_addr: "",
    trading_incentives_addr: ""
  }
}

// eslint-disable-next-line no-unused-vars
function prepareTransaction(signedMsg) {
  const { public_key, signature, stdSignMsgData } = signedMsg
  const sig = StdSignature.fromData({
    signature,
    pub_key: {
      type: 'tendermint/PubKeySecp256k1',
      value: public_key,
    },
  })

  const stdSignMsg = StdSignMsg.fromData(stdSignMsgData)
  return new StdTx(stdSignMsg.msgs, stdSignMsg.fee, [sig], stdSignMsg.memo)
}

const getters = {
  offers: (state) => state.offers,
  getOfferById: (state) => (id) => {
    return state.offers.find((offer) => offer.id === id)
  },
  trades: (state) => state.trades,
  getTradeById: (state) => (tradeAddress) => {
    return state.trades.find((trade) => trade.address === tradeAddress)
  },
}

const actions = {
  /**
   * Fetch Factory Contract config
   */
  async fetchFactoryConfig({ commit, dispatch }) {
    const cfgQuery = { config: {} }
    const factoryConfig = await terra.wasm.contractQuery(FACTORY_CONTRACT, cfgQuery)
    commit('setFactoryConfig', factoryConfig)
    dispatch('fetchOffers')
    dispatch('fetchTrades')
  },
  /**
   * Fetch Offer by Id
   */
  async fetchOffer({ commit }, { id }) {
    const offerQuery = { load_offer: { id } }
    const offer = await terra.wasm.contractQuery(state.factoryConfig.offers_addr, offerQuery)
    commit('addOffer', offer)
  },
  /**
   * Fetch Offers.
   */
  async fetchOffers({ commit }) {
    const offersQuery = { offers: { fiat_currency: state.fiatCurrency } }
    const offers = await terra.wasm.contractQuery(state.factoryConfig.offers_addr, offersQuery)
    commit('setOffers', offers)
  },
  /**
   * Create Offer
   */
  async newOffer({ getters }, { offer }) {
    const offerMsg = new MsgExecuteContract(getters.walletAddress, state.factoryConfig.offers_addr, offer)
    console.log('offerMsg', offerMsg)
    const result = await executeMsg(offerMsg)
    console.log('Offer created', result)
  },
  /**
   * Fetch a specific Trade
   */
  async fetchTrade({ commit, getters, dispatch }, { tradeAddress, redirect = false }) {
    const trade = await terra.wasm.contractQuery(tradeAddress, { config: {} })
    trade.offer = getters.getOfferById(trade.offer_id)
    if (!trade.offer) {
      await dispatch('fetchOffer', { id: trade.offer_id })
    }
    trade.address = tradeAddress
    trade.offer = getters.getOfferById(trade.offer_id)

    commit('addTrade', trade)
    if (redirect) {
      router.push(`/trade/${tradeAddress}`)
    }
    return trade
  },
  /**
   * Fetches all trades for given Trader (maker or taker) address.
   */
  async fetchTrades({commit}) {
    const trades = await terra.wasm.contractQuery(state.factoryConfig.offers_addr, { trades: { maker: "terra1rz4mcfwmqkgv7ss2tygpy79ffd33gh32as49j0" } } )
    commit('setTrades', trades)
  },
  /**
   * Sends a transaction to instantiate a Trade contract.
   * @param {*} offerId Id of the Offer provided by the Offers Smart Contract.
   * @param {*} amount Amount of UST to be traded.
   */
  // eslint-disable-next-line no-unused-vars
  async openTrade({ getters, dispatch }, { offerId, ustAmount }) {
    console.log('open trade', offerId, ustAmount)

    //let sender = getters.walletAddress
    let sender = wallet.key.accAddress
    const amount = parseInt(ustAmount) * 1000000
    const newTradeMsg = {
      new_trade: {
        offer_id: offerId,
        ust_amount: amount + '',
        counterparty: sender
      },
    }
    const createTradeMsg = new MsgExecuteContract(
      sender,
      state.factoryConfig.offers_addr,
      newTradeMsg
    )

    //Coins
    /*
    const offer = getters.getOfferById(parseInt(offerId))
    if (offer.offer_type == 'buy') {
      const coin = Coin.fromData({ denom: 'uusd', amount: ustAmount })
      const coins = new Coins([coin])
      createTradeMsg.coins = coins
    }
    */

    //TODO: Error handling.
    await executeMsg(createTradeMsg)
    dispatch('fetchTrades')

    /*
    //Transaction Signing using the Terra Station Extension and brodcasting.
    ext.once('onSign', async (res) => {
      const signedTx = prepareTransaction(res.result)
      //Broadcast Transaction
      const tx = await terra.tx.broadcast(signedTx)
      const tradeAddress = tx.logs[0].events
        .find((event) => event.type == 'instantiate_contract')
        .attributes.find((attr) => attr.key == 'contract_address').value

      //Fetch Created Trade
      dispatch('fetchTrade', { tradeAddress, redirect: true })
    })

    ext.sign({ msgs: [createTradeMsg] })
    */
  },
  async releaseEscrow({ getters, dispatch }, { tradeAddress }) {
    const releaseMsg = new MsgExecuteContract(getters.walletAddress, tradeAddress, { release: {} })
    const result = await executeMsg(releaseMsg)
    console.log('Released', result)
    dispatch('fetchTrade', { tradeAddress })
  },
}

async function executeMsg(msg) {
  wallet
    .createAndSignTx({
      msgs: [msg],
    })
    .then((tx) => terra.tx.broadcast(tx))
    .then((result) => {
      return result
    })

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
  setFactoryConfig: (state, factoryConfig) => (state.factoryConfig = factoryConfig),
  addOffer: (state, offer) => state.offers.push(offer),
  setOffers: (state, offers) => (state.offers = offers),
  addTrade: (state, trade) => {
    const addedTrade = state.trades.find((t) => t.address == trade.address)
    if (addedTrade) {
      state.trades[state.trades.indexOf(addedTrade)] = trade
    } else {
      state.trades.push(trade)
    }
  },
  setTrades: (state, trades) => {
    state.trades = trades
  }
}

export default {
  state,
  getters,
  actions,
  mutations,
}
