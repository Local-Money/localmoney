import { JUNO_TESTNET_CONFIG, JUNO_TESTNET_HUB_INFO } from './cosmos/config/juno'
import {
  KUJIRA_MAINNET_CONFIG,
  KUJIRA_MAINNET_HUB_INFO,
  KUJIRA_TESTNET_CONFIG,
  KUJIRA_TESTNET_HUB_INFO,
} from './cosmos/config/kujira'
import { DEV_CONFIG, DEV_HUB_INFO } from './cosmos/config/dev'
import type {
  Arbitrator,
  Denom,
  DenomFiatPrice,
  FetchOffersArgs,
  FiatCurrency,
  HubConfig,
  NewTrade,
  OfferResponse,
  PatchOffer,
  PostOffer,
  Profile,
  TradeInfo,
} from '~/types/components.interface'
import { CosmosChain } from '~/network/cosmos/CosmosChain'

export interface Chain {
  init(): void

  connectWallet(): Promise<void>

  getHubConfig(): HubConfig

  getWalletAddress(): string

  fetchProfile(): Promise<Profile>

  fetchOffer(offerId: string): Promise<OfferResponse>

  fetchOffers(args: FetchOffersArgs, limit: number, last?: number): Promise<OfferResponse[]>

  fetchMyOffers(limit: number, last?: number): Promise<OfferResponse[]>

  createOffer(postOffer: PostOffer): Promise<void>

  updateOffer(updateOffer: PatchOffer): Promise<void>

  openTrade(trade: NewTrade): Promise<number>

  fetchTrades(limit: number, last?: number): Promise<TradeInfo[]>

  fetchDisputedTrades(limit: number, last?: number): Promise<{ openDisputes: TradeInfo[]; closedDisputes: TradeInfo[] }>

  fetchTradeDetail(tradeId: number): Promise<TradeInfo>

  fetchArbitrators(): Promise<Arbitrator[]>

  fetchFiatPriceForDenom(fiat: FiatCurrency, denom: Denom): Promise<DenomFiatPrice>

  acceptTradeRequest(tradeId: number, makerContact: string): Promise<void>

  cancelTradeRequest(tradeId: number): Promise<void>

  fundEscrow(tradeInfo: TradeInfo, maker_contact?: string): Promise<void>

  setFiatDeposited(tradeId: number): Promise<void>

  releaseEscrow(tradeId: number): Promise<void>

  refundEscrow(tradeId: number): Promise<void>

  openDispute(tradeId: number, buyerContact: string, sellerContact: string): Promise<void>

  settleDispute(tradeId: number, winner: string): Promise<void>

  newArbitrator(arbitrator: Arbitrator): Promise<void>
}

export enum ChainClient {
  kujiraTestnet = 'KUJIRA_TESTNET',
  kujiraMainnet = 'KUJIRA_MAINNET',
  juno = 'JUNO',
  dev = 'DEV',
}

// Centralized place to instantiate chain client and inject dependencies if needed
export function chainFactory(client: ChainClient): Chain {
  switch (client) {
    case ChainClient.kujiraTestnet:
      return new CosmosChain(KUJIRA_TESTNET_CONFIG, KUJIRA_TESTNET_HUB_INFO)
    case ChainClient.kujiraMainnet:
      return new CosmosChain(KUJIRA_MAINNET_CONFIG, KUJIRA_MAINNET_HUB_INFO)
    case ChainClient.juno:
      return new CosmosChain(JUNO_TESTNET_CONFIG, JUNO_TESTNET_HUB_INFO)
    case ChainClient.dev:
      return new CosmosChain(DEV_CONFIG, DEV_HUB_INFO)
  }
}
