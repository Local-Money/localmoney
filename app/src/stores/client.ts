import { acceptHMRUpdate, defineStore } from 'pinia'
import { useLocalStorage } from '@vueuse/core'
import { ListResult } from './ListResult'
import { ChainClient, chainFactory } from '~/network/Chain'
import type { ChainError } from '~/network/chain-error'
import type {
  Arbitrator,
  Denom,
  FetchOffersArgs,
  FiatCurrency,
  HubConfig,
  NewTrade,
  OfferResponse,
  OfferType,
  PatchOffer,
  PostOffer,
  Profile,
  TradeInfo,
  UserWallet,
} from '~/types/components.interface'
import { LoadingState, OfferState } from '~/types/components.interface'
import type { Secrets } from '~/utils/crypto'
import { encryptData, generateKeys } from '~/utils/crypto'
import { denomToValue } from '~/utils/denom'
import { WalletNotConnected } from '~/network/chain-error'
import { CRYPTO_DECIMAL_PLACES } from '~/utils/constants'

const LIMIT_ITEMS_PER_PAGE = 10

export const useClientStore = defineStore({
  id: 'client',
  state: () => {
    return {
      chainClient: <ChainClient>ChainClient.kujiraTestnet, // TODO call setClient in the App.vue setup function to properly init a chain adapter
      client: chainFactory(ChainClient.kujiraTestnet),
      userWallet: <UserWallet>{ isConnected: false, address: 'undefined' },
      secrets: useLocalStorage('secrets', new Map<string, Secrets>()),
      profile: <Profile>{},
      fiatPrices: new Map<String, Map<String, number>>(),
      offers: <ListResult<OfferResponse>>ListResult.loading(),
      myOffers: <ListResult<OfferResponse>>ListResult.loading(),
      trades: <ListResult<TradeInfo>>ListResult.loading(),
      arbitrators: <ListResult<Arbitrator>>ListResult.loading(),
      openDisputes: <ListResult<TradeInfo>>ListResult.loading(),
      closedDisputes: <ListResult<TradeInfo>>ListResult.loading(),
      loadingState: <LoadingState>LoadingState.dismiss(),
    }
  },
  actions: {
    /**
     * Set the blockchain
     * @param {ChainClient} chainClient - The Blockchain backend to connect to
     */
    async setClient(chainClient: ChainClient) {
      this.$reset()
      // TODO disconnect old chain adapter
      this.chainClient = chainClient
      this.client = chainFactory(this.chainClient)
      await this.client.init()
    },
    async connectWallet() {
      try {
        await this.client.connectWallet()
        const address = this.client.getWalletAddress()
        await this.syncSecrets(address)
        this.userWallet = { isConnected: true, address }
        await this.fetchArbitrators()
      } catch (e) {
        this.userWallet = { isConnected: false, address: 'undefined' }
        this.handle.error(e)
      }
    },
    getHubConfig(): HubConfig {
      return this.client.getHubConfig()
    },
    async fetchProfile() {
      this.profile = await this.client.fetchProfile()
    },
    async syncSecrets(address: string) {
      await this.fetchProfile()
      const secrets = this.secrets.get(address) ?? (await generateKeys())
      console.log(`${address} - secrets: `, secrets)
      if (!this.secrets.has(address)) {
        this.secrets.set(address, secrets)
      }
    },
    getSecrets() {
      const address = this.client.getWalletAddress()
      const userSecrets = this.secrets.get(address)
      if (userSecrets === undefined) {
        throw new WalletNotConnected()
      }
      return userSecrets!
    },
    async fetchOffers(offersArgs: FetchOffersArgs) {
      this.offers = ListResult.loading()
      try {
        const offers = await this.client.fetchOffers(offersArgs, LIMIT_ITEMS_PER_PAGE)
        this.offers = ListResult.success(offers, LIMIT_ITEMS_PER_PAGE)
      } catch (e) {
        this.offers = ListResult.error(e as ChainError)
      }
    },

    // We can improve this code if we return the total amount of offers from the protocol
    async fetchMoreOffers(offersArgs: FetchOffersArgs, last?: number) {
      this.offers.setLoadingMore()
      try {
        const offers = await this.client.fetchOffers(offersArgs, LIMIT_ITEMS_PER_PAGE, last)
        this.offers.addMoreItems(offers, LIMIT_ITEMS_PER_PAGE)
      } catch (e) {
        this.handle.error(e)
      }
    },
    async fetchMyOffers() {
      this.myOffers = ListResult.loading()
      try {
        const myOffers = await this.client.fetchMyOffers(LIMIT_ITEMS_PER_PAGE)
        this.myOffers = ListResult.success(myOffers, LIMIT_ITEMS_PER_PAGE)
      } catch (e) {
        this.myOffers = ListResult.error(e as ChainError)
      }
    },
    // We can improve this code if we return the total amount of my offers from the protocol
    async fetchMoreMyOffers(last: number) {
      this.myOffers.setLoadingMore()
      try {
        const myOffers = await this.client.fetchMyOffers(LIMIT_ITEMS_PER_PAGE, last)
        this.myOffers.addMoreItems(myOffers, LIMIT_ITEMS_PER_PAGE)
      } catch (e) {
        this.handle.error(e)
      }
    },
    async createOffer(param: {
      telegram_handle: string
      offer_type: OfferType
      fiat_currency: FiatCurrency
      rate: string
      denom: Denom
      min_amount: number
      max_amount: number
      description: string
    }) {
      this.loadingState = LoadingState.show('Creating Offer...')
      try {
        // Encrypt contact to save on the profile when an offer is created
        const owner_encryption_key = this.getSecrets().publicKey
        const owner_contact = await encryptData(owner_encryption_key, param.telegram_handle)
        await this.client.createOffer({
          ...param,
          min_amount: `${param.min_amount * CRYPTO_DECIMAL_PLACES}`,
          max_amount: `${param.max_amount * CRYPTO_DECIMAL_PLACES}`,
          owner_contact,
          owner_encryption_key,
        } as PostOffer)
        await this.fetchProfile()
        await this.fetchMyOffers()
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async updateOffer(updateOffer: PatchOffer) {
      this.loadingState = LoadingState.show('Updating Offer...')
      try {
        await this.client.updateOffer(updateOffer)
        await this.fetchMyOffers()
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async unarchiveOffer(updateOffer: PatchOffer) {
      this.loadingState = LoadingState.show('Archiving Offer...')
      try {
        updateOffer.state = OfferState.paused
        await this.client.updateOffer(updateOffer)
        await this.fetchMyOffers()
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async openTrade(offerResponse: OfferResponse, telegramHandle: string, amount: number) {
      this.loadingState = LoadingState.show('Opening trade...')
      try {
        const profile_taker_encryption_key = this.getSecrets().publicKey
        const taker_contact = await encryptData(offerResponse.profile.encryption_key!, telegramHandle)
        const profile_taker_contact = await encryptData(profile_taker_encryption_key, telegramHandle)
        const newTrade: NewTrade = {
          offer_id: offerResponse.offer.id,
          amount: `${Number(amount * CRYPTO_DECIMAL_PLACES).toFixed(0)}`,
          taker: `${this.userWallet.address}`,
          profile_taker_contact,
          taker_contact,
          profile_taker_encryption_key,
        }
        const trade_id = await this.client.openTrade(newTrade)
        await this.fetchProfile()
        const route = isNaN(trade_id) ? { name: 'Trades' } : { name: 'TradeDetail', params: { id: trade_id } }
        await this.router.push(route)
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async fetchTrades() {
      this.trades = ListResult.loading()
      try {
        const tradesList = await this.client.fetchTrades(LIMIT_ITEMS_PER_PAGE)
        this.trades = ListResult.success(tradesList, LIMIT_ITEMS_PER_PAGE)
      } catch (e) {
        this.trades = ListResult.error(e as ChainError)
      }
    },
    async fetchMoreTrades(last: number) {
      this.trades.setLoadingMore()
      try {
        const trades = await this.client.fetchTrades(LIMIT_ITEMS_PER_PAGE, last)
        this.trades.addMoreItems(trades, LIMIT_ITEMS_PER_PAGE)
      } catch (e) {
        this.handle.error(e)
      }
    },
    async fetchTradeDetail(tradeId: number) {
      return await this.client.fetchTradeDetail(tradeId)
    },
    async fetchArbitrators() {
      this.arbitrators = ListResult.loading()
      try {
        const arbitratorsList = await this.client.fetchArbitrators()
        this.arbitrators = ListResult.success(arbitratorsList)
      } catch (e) {
        this.arbitrators = ListResult.error(e as ChainError)
      }
    },
    async fetchDisputedTrades(limit = 30, last?: number) {
      this.openDisputes = ListResult.loading()
      this.closedDisputes = ListResult.loading()
      try {
        const disputedTrades = await this.client.fetchDisputedTrades(limit, last)
        this.openDisputes = ListResult.success(disputedTrades.openDisputes)
        this.closedDisputes = ListResult.success(disputedTrades.closedDisputes)
      } catch (e) {
        console.error(e)
      }
    },
    async updateFiatPrice(fiat: FiatCurrency, denom: Denom) {
      try {
        const price = await this.client.updateFiatPrice(fiat, denom)
        if (this.fiatPrices.has(fiat)) {
          this.fiatPrices.get(fiat)?.set(denomToValue(denom), price.price)
        } else {
          const priceForDenom = new Map([[denomToValue(denom), price.price]])
          this.fiatPrices.set(fiat, priceForDenom)
        }
      } catch (e) {
        console.error(e)
      }
    },
    async fetchFiatPriceForDenom(fiat: FiatCurrency, denom: Denom) {
      return await this.client.updateFiatPrice(fiat, denom)
    },
    async acceptTradeRequest(tradeId: number, makerContact: string) {
      this.loadingState = LoadingState.show('Accepting trade...')
      try {
        await this.client.acceptTradeRequest(tradeId, makerContact)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async cancelTradeRequest(tradeId: number) {
      this.loadingState = LoadingState.show('Canceling trade...')
      try {
        await this.client.cancelTradeRequest(tradeId)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async fundEscrow(tradeInfo: TradeInfo, makerContact?: string) {
      this.loadingState = LoadingState.show('Funding trade...')
      try {
        await this.client.fundEscrow(tradeInfo, makerContact)
        await this.fetchTradeDetail(tradeInfo.trade.id)
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async setFiatDeposited(tradeId: number) {
      this.loadingState = LoadingState.show('Marking trade as paid...')
      try {
        await this.client.setFiatDeposited(tradeId)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async releaseEscrow(tradeId: number) {
      this.loadingState = LoadingState.show('Funding trade...')
      try {
        await this.client.releaseEscrow(tradeId)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async refundEscrow(tradeId: number) {
      this.loadingState = LoadingState.show('Refunding trade...')
      try {
        await this.client.refundEscrow(tradeId)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async openDispute(tradeId: number, buyerContact: string, sellerContact: string) {
      this.loadingState = LoadingState.show('Opening dispute...')
      try {
        await this.client.openDispute(tradeId, buyerContact, sellerContact)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async settleDispute(tradeId: number, winner: string) {
      this.loadingState = LoadingState.show('Settling dispute...')
      try {
        await this.client.settleDispute(tradeId, winner)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    getFiatPrice(fiatCurrency: FiatCurrency, denom: Denom): number {
      const fiatPrice = this.fiatPrices.get(fiatCurrency)?.get(denomToValue(denom)) ?? 0
      try {
        return fiatPrice / 100
      } catch (e) {
        return 0
      }
    },
  },
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useClientStore, import.meta.hot))
}
