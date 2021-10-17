import {
  Extension,
  LCDClient,
  MnemonicKey,
  MsgExecuteContract,
  MsgInstantiateContract,
  StdSignature,
  StdSignMsg,
  StdTx,
} from '@terra-money/terra.js'
import { OFFERS_CONTRACT } from '@/constants.js'
import router from '@/router'

// create a key out of a mnemonic
const mk = new MnemonicKey({
  mnemonic:
    'uncle simple tide bundle apart absurd tenant fluid slam actor caught month hip tornado cattle regular nerve brand tower boy alert crash good neck',
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
}

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
  const signedTx = new StdTx(stdSignMsg.msgs, stdSignMsg.fee, [sig], stdSignMsg.memo)

  return signedTx
}

const getters = {
  allOffers: (state) => state.offers,
  getOfferById: (state) => (id) => {
    return state.offers.find((offer) => offer.id == id)
  },
  getTrades: (state) => state.trades,
  getTradeById: (state) => (tradeAddress) => {
    return state.trades.find((trade) => trade.address == tradeAddress)
  },
}

const actions = {
  /**
   * Fetch Offer by Id
   */
  async fetchOffer({ commit }, { id }) {
    const offerQuery = { load_offer: { id } }
    const offer = await terra.wasm.contractQuery(OFFERS_CONTRACT, offerQuery)
    commit('addOffer', offer)
  },
  /**
   * Fetch Offers.
   */
  async fetchOffers({ commit }) {
    const offersQuery = { offers: { fiat_currency: 'BRL' } }
    const offers = await terra.wasm.contractQuery(OFFERS_CONTRACT, offersQuery)
    commit('setOffers', offers)
  },
  /**
   * Create Offer
   */
  async newOffer({ getters }, { offer }) {
    const offerMsg = new MsgExecuteContract(getters.walletAddress, OFFERS_CONTRACT, offer)
    console.log('offerMsg', offerMsg)
    await executeMsg(offerMsg, () => {
      alert('Offer created')
    })
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
   * Sends a transaction to instantiate a Trade contract.
   * @param {*} offerId Id of the Offer provided by the Offers Smart Contract.
   * @param {*} amount Amount of UST to be traded.
   */
  async openTrade({ getters, dispatch }, { offerId, ustAmount }) {
    console.log('open trade', offerId, ustAmount)

    const amount = parseInt(ustAmount) * 1000000
    const newTradeMsg = {
      new_trade: {
        offer_id: offerId,
        ust_amount: amount + '',
        counterparty: getters.walletAddress,
      },
    }
    const createTradeMsg = new MsgExecuteContract(
      getters.walletAddress,
      OFFERS_CONTRACT,
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

    executeMsg(createTradeMsg, () => {
      console.log('trade created')
    })
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
    await executeMsg(releaseMsg, () => {
      dispatch('fetchTrade', { tradeAddress })
    })
  },
}

async function executeMsg(msg, cb) {
  wallet
    .createAndSignTx({
      msgs: [msg],
    })
    .then((tx) => terra.tx.broadcast(tx))
    .then((result) => {
      console.log(result)
      cb()
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
}

export default {
  state,
  getters,
  actions,
  mutations,
}
