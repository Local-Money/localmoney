/* eslint-disable @typescript-eslint/explicit-module-boundary-types */
import { Extension } from '@terra-money/terra.js'

const state = {
  walletAddress: '',
}

const getters = {
  walletAddress: (state) => state.walletAddress,
}

const actions = {
  async initWallet({ commit }) {
    const ext = new Extension()
    const res = await ext.request('connect')
    const walletAddress = res.payload.address

    commit('setWalletAddress', walletAddress)
  },
}

const mutations = {
  setWalletAddress: (state, walletAddress) => (state.walletAddress = walletAddress),
}

export default {
  state,
  getters,
  actions,
  mutations,
}
