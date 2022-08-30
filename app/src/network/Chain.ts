import type {
  Denom,
  FetchOffersArgs,
  GetOffer,
  NewTrade,
  PatchOffer,
  PostOffer,
  Trade,
  TradeInfo,
} from '~/types/components.interface'
import MockChain from '~/network/mock/MockChain'
import { CosmosChain } from '~/network/cosmos/CosmosChain'
import {
  JUNO_CONFIG,
  JUNO_HUB_INFO,
  KUJIRA_CONFIG,
  KUJIRA_HUB_INFO,
  TEST_CONFIG,
  TEST_HUB_INFO,
} from '~/network/cosmos/config'

export interface Chain {
  init(): void

  connectWallet(): Promise<void>

  getWalletAddress(): string

  fetchOffers(args: FetchOffersArgs): Promise<GetOffer[]>

  fetchMyOffers(): Promise<GetOffer[]>

  createOffer(postOffer: PostOffer): Promise<void>

  updateOffer(updateOffer: PatchOffer): Promise<void>

  openTrade(trade: NewTrade): Promise<string>

  fetchTrades(): Promise<TradeInfo[]>

  fetchTradeDetail(tradeId: string): Promise<Trade>

  acceptTradeRequest(tradeId: string): Promise<void>

  cancelTradeRequest(tradeId: string): Promise<void>

  fundEscrow(tradeId: string, amount: string, denom: Denom): Promise<void>

  setFiatDeposited(tradeId: string): Promise<void>

  releaseEscrow(tradeId: string): Promise<void>

  refundEscrow(tradeId: string): Promise<void>

  openDispute(tradeId: string): Promise<void>
}

export enum ChainClient {
  mock = 'MOCK',
  kujira = 'KUJIRA',
  juno = 'JUNO',
  dev = 'DEV',
}

// Centralized place to instantiate chain client and inject dependencies if needed
export function chainFactory(client: ChainClient): Chain {
  switch (client) {
    case ChainClient.mock:
      return new MockChain()
    case ChainClient.kujira:
      return new CosmosChain(KUJIRA_CONFIG, KUJIRA_HUB_INFO)
    case ChainClient.juno:
      return new CosmosChain(JUNO_CONFIG, JUNO_HUB_INFO)
    case ChainClient.dev:
      return new CosmosChain(TEST_CONFIG, TEST_HUB_INFO)
  }
}
