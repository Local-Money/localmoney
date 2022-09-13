import { acceptHMRUpdate, defineStore } from 'pinia'
import { ListResult } from './ListResult'
import { ChainClient, chainFactory } from '~/network/Chain'
import type { ChainError } from '~/network/chain-error'
import type {
  Arbitrator,
  Denom,
  FetchOffersArgs,
  GetOffer,
  NewTrade,
  PatchOffer,
  PostOffer,
  TradeInfo,
  UserWallet,
} from '~/types/components.interface'
import { LoadingState, OfferState } from '~/types/components.interface'

export const useClientStore = defineStore({
  id: 'client',
  state: () => {
    return {
      chainClient: <ChainClient>ChainClient.mock, // TODO call setClient in the App.vue setup function to properly init a chain adapter
      client: chainFactory(ChainClient.kujira),
      userWallet: <UserWallet>{ isConnected: false, address: 'undefined' },
      offers: <ListResult<GetOffer>>ListResult.loading(),
      myOffers: <ListResult<GetOffer>>ListResult.loading(),
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
        this.userWallet = { isConnected: true, address }
        await this.fetchArbitrators()
      } catch (e) {
        this.userWallet = { isConnected: false, address: 'undefined' }
        alert((e as ChainError).message)
      }
    },
    async fetchOffers(offersArgs: FetchOffersArgs) {
      this.offers = ListResult.loading()
      try {
        const listOffers = await this.client.fetchOffers(offersArgs)
        this.offers = ListResult.success(listOffers)
      } catch (e) {
        this.offers = ListResult.error(e as ChainError)
      }
    },
    async fetchMyOffers() {
      this.myOffers = ListResult.loading()
      try {
        const listMyOffers = await this.client.fetchMyOffers()
        this.myOffers = ListResult.success(listMyOffers)
      } catch (e) {
        this.myOffers = ListResult.error(e as ChainError)
      }
    },
    async createOffer(postOffer: PostOffer) {
      this.loadingState = LoadingState.show('Creating Offer...')
      try {
        await this.client.createOffer(postOffer)
        await this.fetchMyOffers()
      } catch (e) {
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
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
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
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
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async openTrade(trade: NewTrade) {
      this.loadingState = LoadingState.show('Opening trade...')
      try {
        const trade_id = await this.client.openTrade(trade)
        const route = trade_id === '' ? { name: 'Trades' } : { name: 'TradeDetail', params: { id: trade_id } }
        await this.router.push(route)
      } catch (e) {
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async fetchMyTrades() {
      this.trades = ListResult.loading()
      try {
        const tradesList = await this.client.fetchTrades()
        this.trades = ListResult.success(tradesList)
      } catch (e) {
        this.trades = ListResult.error(e as ChainError)
      }
    },
    async fetchTradeDetail(tradeId: string): Promise<TradeInfo> {
      const trade = await this.client.fetchTradeDetail(tradeId)
      const offer = await this.client.fetchOffer(trade.offer_id)
      return {
        trade,
        offer,
        expired: false,
      } as TradeInfo
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
    async fetchDisputedTrades() {
      this.openDisputes = ListResult.loading()
      this.closedDisputes = ListResult.loading()
      try {
        const disputedTrades = await this.client.fetchDisputedTrades()
        this.openDisputes = ListResult.success(disputedTrades.openDisputes)
        this.closedDisputes = ListResult.success(disputedTrades.closedDisputes)
      } catch (e) {
        console.error(e)
      }
    },
    async acceptTradeRequest(tradeId: string, makerContact: string) {
      this.loadingState = LoadingState.show('Accepting trade...')
      try {
        await this.client.acceptTradeRequest(tradeId, makerContact)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async cancelTradeRequest(tradeId: string) {
      this.loadingState = LoadingState.show('Canceling trade...')
      try {
        await this.client.cancelTradeRequest(tradeId)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async fundEscrow(tradeId: string, amount: string, denom: Denom, makerContact?: string) {
      this.loadingState = LoadingState.show('Funding trade...')
      try {
        await this.client.fundEscrow(tradeId, amount, denom, makerContact)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async setFiatDeposited(tradeId: string) {
      this.loadingState = LoadingState.show('Marking trade as paid...')
      try {
        await this.client.setFiatDeposited(tradeId)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async releaseEscrow(tradeId: string) {
      this.loadingState = LoadingState.show('Funding trade...')
      try {
        await this.client.releaseEscrow(tradeId)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async refundEscrow(tradeId: string) {
      this.loadingState = LoadingState.show('Refunding trade...')
      try {
        await this.client.refundEscrow(tradeId)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async openDispute(tradeId: string) {
      this.loadingState = LoadingState.show('Opening dispute...')
      try {
        await this.client.openDispute(tradeId)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
    async settleDispute(tradeId: string, winner: string) {
      this.loadingState = LoadingState.show('Settling dispute...')
      try {
        await this.client.settleDispute(tradeId, winner)
        await this.fetchTradeDetail(tradeId)
      } catch (e) {
        // TODO handle error
        alert((e as ChainError).message)
        console.error(e)
      } finally {
        this.loadingState = LoadingState.dismiss()
      }
    },
  },
})

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useClientStore, import.meta.hot))
}
