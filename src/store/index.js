import { createStore } from 'vuex'
import axios from 'axios'

import wallet from './modules/wallet'
import transactions from './modules/transactions'

export default createStore({
  state: {
    usdRates: Object,
  },
  getters: {
    getUsdRate: (state) => (currency) => {
      return state.usdRates[('usd' + currency).toUpperCase()]
    },
  },
  actions: {
    async fetchUsdRates({ commit }) {
      const URL =
        'http://www.apilayer.net/api/live?access_key=78bd873aa48cdf5de03ce0b650cda9eb&format=1&currencies=COP,BRL,ARS'
      const rates = await axios.get(URL)
      commit('setUsdRates', rates.data.quotes)
    },
  },
  mutations: {
    setUsdRates: (state, rates) => {
      state.usdRates = rates
    },
  },
  modules: {
    transactions,
    wallet,
  },
})
