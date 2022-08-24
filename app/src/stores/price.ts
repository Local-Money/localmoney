import { acceptHMRUpdate, defineStore } from 'pinia'
import { FiatCurrency } from '~/types/components.interface'

const fetchPrices = {
  [FiatCurrency.BRL]: async () => { return 4.88 },
  [FiatCurrency.ARS]: async () => { return 118.49 },
  [FiatCurrency.COP]: async () => { return 3977.33 },
}

async function fetchAllPrices() {
  return {
    BRL: 4.88,
    ARS: 118.49,
    COP: 3977.33,
  }
}

export const usePriceStore = defineStore({
  id: 'price',
  state: () => ({
    prices: {
      [FiatCurrency.BRL]: 0,
      [FiatCurrency.ARS]: 0,
      [FiatCurrency.COP]: 0,
    },
  }),

  actions: {
    async fetchPrice(fiatCurrency: FiatCurrency) {
      this.prices[fiatCurrency] = await fetchPrices[fiatCurrency]()
    },
    async fetchPrices() {
      this.prices = await fetchAllPrices()
    },
  },
  getters: { getPrice: state => (fiatCurrency: FiatCurrency) => state.prices[fiatCurrency] },
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(usePriceStore, import.meta.hot))
}

