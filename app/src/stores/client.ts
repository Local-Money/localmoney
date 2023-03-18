import { acceptHMRUpdate, defineStore } from 'pinia'
import { useLocalStorage } from '@vueuse/core'
import type { Coin } from '@cosmjs/stargate'
import axios from 'axios'
import { ListResult } from './ListResult'
import { ChainClient, chainFactory } from '~/network/Chain'
import type { ChainError } from '~/network/chain-error'
import { WalletNotConnected } from '~/network/chain-error'
import type {
  Addr,
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
import { LoadingState, OfferState, TradeState } from '~/types/components.interface'
import type { Secrets } from '~/utils/crypto'
import { encryptData, generateKeys } from '~/utils/crypto'
import { denomToValue } from '~/utils/denom'
import { CRYPTO_DECIMAL_PLACES } from '~/utils/constants'
import { OfferEvents, TradeEvents, toOfferData, toTradeData, trackOffer, trackTrade } from '~/analytics/analytics'

const LIMIT_ITEMS_PER_PAGE = 10

export const useClientStore = defineStore({
  id: 'client',
  state: () => {
    return {
      chainClient: <ChainClient>ChainClient.kujiraTestnet, // TODO call setClient in the App.vue setup function to properly init a chain adapter
      client: chainFactory(ChainClient.kujiraTestnet),
      applicationConnected: useLocalStorage('walletAlreadyConnected', false),
      userWallet: <UserWallet>{ isConnected: false, address: 'undefined' },
      secrets: useLocalStorage('secrets', new Map<string, Secrets>()),
      profile: <Profile>{},
      localBalance: <Coin>{},
      fiatPrices: new Map<String, Map<String, number>>(),
      offers: <ListResult<OfferResponse>>ListResult.loading(),
      makerOffers: <ListResult<OfferResponse>>ListResult.loading(),
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
      if (this.applicationConnected) {
        await this.connectWallet()
      }
    },
    async connectWallet() {
      try {
        await this.client.connectWallet()
        const address = this.client.getWalletAddress()
        await this.syncSecrets(address)
        this.userWallet = { isConnected: true, address }
        await this.fetchBalances()
        this.applicationConnected = true
        await this.fetchArbitrators()
      } catch (e) {
        this.userWallet = { isConnected: false, address: 'undefined' }
        this.handle.error(e)
      }
    },
    async fetchBalances() {
      await this.fetchLocalBalance()
    },
    async fetchLocalBalance() {
      // Todo we should change this to get the LOCAL denom from some config
      let localDenom: Denom
      if (this.chainClient === ChainClient.kujiraMainnet) {
        localDenom = { native: 'factory/kujira1swkuyt08z74n5jl7zr6hx0ru5sa2yev5v896p6/local' }
      } else {
        localDenom = { native: 'factory/kujira12w0ua4eqnkk0aahtnjlt6h3dhxael6x25s507w/local' }
      }

      this.localBalance = await this.client.fetchTokenBalance(localDenom)
    },
    async disconnectWallet() {
      await this.client.disconnectWallet()
      this.userWallet = { isConnected: false, address: 'undefined' }
      this.applicationConnected = false
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
      const address = this.userWallet.address
      const userSecrets = this.secrets.get(address)
      if (userSecrets === undefined) {
        throw new WalletNotConnected()
      }
      return userSecrets!
    },
    async fetchMakerProfile(maker: Addr) {
      return await this.client.fetchProfile(maker)
    },
    async fetchMakerOffers(maker: Addr) {
      this.makerOffers = ListResult.loading()
      try {
        let offers = await this.client.fetchMakerOffers(maker)
        offers = offers.filter(({ offer }) => offer.state === OfferState.active)
        for (const { offer } of offers) {
          await this.updateFiatPrice(offer.fiat_currency, offer.denom)
        }
        this.makerOffers = ListResult.success(offers)
      } catch (e) {
        this.makerOffers = ListResult.error(e as ChainError)
      }
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
        const postOffer = {
          ...param,
          min_amount: `${param.min_amount * CRYPTO_DECIMAL_PLACES}`,
          max_amount: `${param.max_amount * CRYPTO_DECIMAL_PLACES}`,
          owner_contact,
          owner_encryption_key,
        } as PostOffer
        const offerId = await this.client.createOffer(postOffer)
        trackOffer(OfferEvents.created, toOfferData(offerId, postOffer, this.chainClient))
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
        trackOffer(OfferEvents.updated, toOfferData(updateOffer.id, updateOffer, this.chainClient))
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
        trackOffer(OfferEvents.unarchived, toOfferData(updateOffer.id, updateOffer, this.chainClient))
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
        const tradeInfo = await this.fetchTradeDetail(trade_id)
        trackTrade(TradeEvents.created, toTradeData(tradeInfo.trade, tradeInfo.offer.offer, this.chainClient))
        this.notifyOnBot({ ...tradeInfo.trade, state: TradeState.request_created })
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
        const tradeInfo = await this.fetchTradeDetail(tradeId)
        trackTrade(TradeEvents.accepted, toTradeData(tradeInfo.trade, tradeInfo.offer.offer, this.chainClient))
        this.notifyOnBot({ ...tradeInfo.trade, state: TradeState.request_accepted })
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
        const tradeInfo = await this.fetchTradeDetail(tradeId)
        trackTrade(TradeEvents.canceled, toTradeData(tradeInfo.trade, tradeInfo.offer.offer, this.chainClient))
        this.notifyOnBot({ ...tradeInfo.trade, state: TradeState.request_canceled })
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
        const trade = await this.fetchTradeDetail(tradeInfo.trade.id)
        trackTrade(TradeEvents.funded, toTradeData(trade.trade, trade.offer.offer, this.chainClient))
        this.notifyOnBot({ ...tradeInfo.trade, state: TradeState.escrow_funded })
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
        const tradeInfo = await this.fetchTradeDetail(tradeId)
        trackTrade(TradeEvents.paid, toTradeData(tradeInfo.trade, tradeInfo.offer.offer, this.chainClient))
        this.notifyOnBot({ ...tradeInfo.trade, state: TradeState.fiat_deposited })
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
        const tradeInfo = await this.fetchTradeDetail(tradeId)
        trackTrade(TradeEvents.released, toTradeData(tradeInfo.trade, tradeInfo.offer.offer, this.chainClient))
        this.notifyOnBot({ ...tradeInfo.trade, state: TradeState.escrow_released })
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
        const tradeInfo = await this.fetchTradeDetail(tradeId)
        trackTrade(TradeEvents.refunded, toTradeData(tradeInfo.trade, tradeInfo.offer.offer, this.chainClient))
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
        const tradeInfo = await this.fetchTradeDetail(tradeId)
        trackTrade(TradeEvents.disputed, toTradeData(tradeInfo.trade, tradeInfo.offer.offer, this.chainClient))
        this.notifyOnBot({ ...tradeInfo.trade, state: TradeState.escrow_disputed })
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
        const tradeInfo = await this.fetchTradeDetail(tradeId)
        trackTrade(TradeEvents.dispute_settled, toTradeData(tradeInfo.trade, tradeInfo.offer.offer, this.chainClient))
        this.notifyOnBot({ ...tradeInfo.trade, state: TradeState.settled_for_maker })
      } catch (e) {
        this.handle.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    notifyOnBot(trade: { id: number; state: TradeState; buyer: string; seller: string }) {
      // only on mainnet it will trigger the bot
      if (this.chainClient === ChainClient.kujiraMainnet) {
        const address = this.userWallet.address === trade.seller ? trade.buyer : trade.seller
        const notification = JSON.stringify({ data: [{ trade_id: trade.id, trade_state: trade.state, address }] })
        axios
          .post('/notify', notification, {
            headers: {
              'Content-Type': 'application/json',
            },
          })
          .then((result) => console.log('result: ', result.data))
          .catch((e) => console.error(e))
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
