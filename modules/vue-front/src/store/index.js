import { createStore } from 'vuex'
// import axios from 'axios'

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
      // const URL = 'http://www.apilayer.net/api/live?access_key=78bd873aa48cdf5de03ce0b650cda9eb&format=1&currencies=COP,BRL,ARS'
      // const rates = await axios.get(URL)
      const responseMock = { "USDBRL": 5.557904, "USDCOP": 3935, "USDARS": 100.360503 };
      commit('setUsdRates', responseMock)
    },
  },
  mutations: {
    setUsdRates: (state, rates) => {
      state.usdRates = rates
    },
  },
  modules: {
    transactions,
  },
})
