const state = {
  walletAddress: '',
}

const getters = {
  walletAddress: (state) => state.walletAddress,
}

const actions = {
  async initWallet({ dispatch}) {
    dispatch('fakeWalletConnect')
    /*
    const ext = new Extension()
    const res = await ext.request('connect')
    const walletAddress = res.payload.address

    commit('setWalletAddress', walletAddress)
     */
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
