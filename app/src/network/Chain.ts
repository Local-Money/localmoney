import { JUNO_TESTNET_CONFIG, JUNO_TESTNET_HUB_INFO } from './cosmos/config/juno'
import { KUJIRA_TESTNET_CONFIG, KUJIRA_TESTNET_HUB_INFO } from './cosmos/config/kujira'
import { DEV_CONFIG, DEV_HUB_INFO } from './cosmos/config/dev'
import type {
  Arbitrator,
  Denom,
  FetchOffersArgs,
  GetOffer,
  NewTrade,
  PatchOffer,
  PostOffer,
  Profile,
  Trade,
  TradeInfo,
} from '~/types/components.interface'
import { CosmosChain } from '~/network/cosmos/CosmosChain'

export interface Chain {
  init(): void

  connectWallet(): Promise<void>

  getWalletAddress(): string

  fetchProfile(): Promise<Profile>

  fetchOffer(offerId: string): Promise<GetOffer>

  fetchOffers(args: FetchOffersArgs): Promise<GetOffer[]>

  fetchMyOffers(): Promise<GetOffer[]>

  createOffer(postOffer: PostOffer): Promise<void>

  updateOffer(updateOffer: PatchOffer): Promise<void>

  openTrade(trade: NewTrade): Promise<string>

  fetchTrades(): Promise<TradeInfo[]>

  fetchDisputedTrades(): Promise<{ openDisputes: TradeInfo[]; closedDisputes: TradeInfo[] }>

  fetchTradeDetail(tradeId: string): Promise<Trade>

  fetchArbitrators(): Promise<Arbitrator[]>

  acceptTradeRequest(tradeId: string, makerContact: string): Promise<void>

  cancelTradeRequest(tradeId: string): Promise<void>

  fundEscrow(tradeId: string, amount: string, denom: Denom, maker_contact?: string): Promise<void>

  setFiatDeposited(tradeId: string): Promise<void>

  releaseEscrow(tradeId: string): Promise<void>

  refundEscrow(tradeId: string): Promise<void>

  openDispute(tradeId: string, buyerContact: string, sellerContact: string): Promise<void>

  settleDispute(tradeId: string, winner: string): Promise<void>

  newArbitrator(arbitrator: Arbitrator): Promise<void>
}

export enum ChainClient {
  kujira = 'KUJIRA',
  juno = 'JUNO',
  dev = 'DEV',
}

// Centralized place to instantiate chain client and inject dependencies if needed
export function chainFactory(client: ChainClient): Chain {
  switch (client) {
    case ChainClient.kujira:
      return new CosmosChain(KUJIRA_TESTNET_CONFIG, KUJIRA_TESTNET_HUB_INFO)
    case ChainClient.juno:
      return new CosmosChain(JUNO_TESTNET_CONFIG, JUNO_TESTNET_HUB_INFO)
    case ChainClient.dev:
      return new CosmosChain(DEV_CONFIG, DEV_HUB_INFO)
  }
}
