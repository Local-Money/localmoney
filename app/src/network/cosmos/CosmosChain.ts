/* eslint-disable no-console */
import { CosmWasmClient, SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import type { AccountData, OfflineSigner } from '@cosmjs/launchpad'
import { Decimal } from '@cosmjs/math'
import type { OfflineDirectSigner } from '@cosmjs/proto-signing'
import type { Coin } from '@cosmjs/stargate'
import type { Chain } from '~/network/Chain'
import { DefaultError, WalletNotConnected, WalletNotInstalled } from '~/network/chain-error'
import type { CosmosConfig, HubInfo } from '~/network/cosmos/config'
import type {
  Addr,
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
import { denomToValue } from '~/utils/denom'

export class CosmosChain implements Chain {
  protected config: CosmosConfig
  protected hubInfo: HubInfo

  protected signer?: OfflineSigner | OfflineDirectSigner
  protected account?: AccountData
  protected cwClient?: CosmWasmClient | SigningCosmWasmClient

  constructor(config: CosmosConfig, hubInfo: HubInfo) {
    this.config = config
    this.hubInfo = hubInfo
  }

  async init() {
    this.cwClient = await CosmWasmClient.connect(this.config.rpcUrl)
    this.hubInfo.hubConfig = (await this.cwClient.queryContractSmart(this.hubInfo.hubAddress, {
      config: {},
    })) as HubConfig
    // console.log("Factory config >> ", this.hubInfo.hubConfig)
  }

  async connectWallet() {
    if (!window.getOfflineSigner || !window.keplr || !window.getOfflineSignerAuto) {
      throw new WalletNotInstalled()
    } else {
      await CosmosChain.suggestChain(this.config)
      await window.keplr.enable(this.config.chainId)
      this.signer = await window.getOfflineSignerAuto(this.config.chainId)
      this.cwClient = await SigningCosmWasmClient.connectWithSigner(this.config.rpcUrl, this.signer, {
        gasPrice: {
          amount: Decimal.fromUserInput('0.0025', 100),
          denom: this.config.coinMinimalDenom,
        },
      })
      // get first account
      ;[this.account] = await this.signer.getAccounts()
    }
  }

  async disconnectWallet() {
    this.cwClient?.disconnect()
    this.account = undefined
    this.signer = undefined
  }

  getHubConfig(): HubConfig {
    return this.hubInfo.hubConfig
  }

  getWalletAddress(): string {
    return this.account ? this.account.address : 'undefined'
  }

  async fetchProfile(profile_addr?: Addr) {
    if (!this.cwClient) {
      await this.init()
    }
    try {
      const addr = profile_addr === undefined ? this.getWalletAddress() : profile_addr
      const result = (await this.cwClient!.queryContractSmart(this.hubInfo.hubConfig.profile_addr, {
        profile: { addr },
      })) as Profile
      console.log('Profile result >> ', result)
      return result
    } catch (e) {
      throw DefaultError.fromError(e)
    }
  }

  async fetchTokenBalance(denom: Denom) {
    if (this.cwClient instanceof SigningCosmWasmClient && this.signer) {
      try {
        const balance = await this.cwClient.getBalance(this.getWalletAddress(), denomToValue(denom))
        console.log(`balance: `, balance)
        return balance
      } catch (e) {
        throw DefaultError.fromError(e)
      }
    } else {
      throw new WalletNotConnected()
    }
  }

  // TODO encrypt the postOffer.owner_contact field
  async createOffer(postOffer: PostOffer) {
    const msg = { create: { offer: postOffer } }
    console.log('Create offer msg >> ', msg)
    if (this.cwClient instanceof SigningCosmWasmClient && this.signer) {
      try {
        const result = await this.cwClient.execute(
          this.getWalletAddress(),
          this.hubInfo.hubConfig.offer_addr,
          msg,
          'auto'
        )
        console.log('Create offer result >> ', result)
        const offer_id = result.logs[0].events
          .find((e) => e.type === 'wasm')
          ?.attributes.find((a) => a.key === 'id')?.value
        return Number(offer_id)
      } catch (e) {
        throw DefaultError.fromError(e)
      }
    } else {
      throw new WalletNotConnected()
    }
  }

  // TODO encrypt the postOffer.owner_contact field
  async updateOffer(updateOffer: PatchOffer) {
    const msg = { update_offer: { offer_update: updateOffer } }
    console.log('Update offer msg >> ', msg)
    if (this.cwClient instanceof SigningCosmWasmClient && this.signer) {
      try {
        const result = await this.cwClient.execute(
          this.getWalletAddress(),
          this.hubInfo.hubConfig.offer_addr,
          msg,
          'auto'
        )
        console.log('Update offer result >> ', result)
      } catch (e) {
        throw DefaultError.fromError(e)
      }
    } else {
      throw new WalletNotConnected()
    }
  }

  async fetchMyOffers(limit = 100, last?: number) {
    if (this.cwClient instanceof SigningCosmWasmClient) {
      try {
        return (await this.cwClient.queryContractSmart(this.hubInfo.hubConfig.offer_addr, {
          offers_by_owner: {
            owner: this.getWalletAddress(),
            limit,
            last,
          },
        })) as OfferResponse[]
      } catch (e) {
        throw DefaultError.fromError(e)
      }
    } else {
      throw new WalletNotConnected()
    }
  }

  async fetchMakerOffers(maker: Addr): Promise<OfferResponse[]> {
    if (!this.cwClient) {
      await this.init()
    }
    try {
      return (await this.cwClient!.queryContractSmart(this.hubInfo.hubConfig.offer_addr, {
        offers_by_owner: {
          owner: maker,
          limit: 1000,
        },
      })) as OfferResponse[]
    } catch (e) {
      throw DefaultError.fromError(e)
    }
  }

  async fetchOffer(offerId: string): Promise<OfferResponse> {
    // TODO: fix init
    if (!this.cwClient) {
      await this.init()
    }
    try {
      const queryMsg = { offer: { id: offerId } }
      const response = (await this.cwClient!.queryContractSmart(
        this.hubInfo.hubConfig.offer_addr,
        queryMsg
      )) as OfferResponse
      console.log('response >>> ', response)
      return response
    } catch (e) {
      throw DefaultError.fromError(e)
    }
  }

  async fetchOffers(args: FetchOffersArgs, limit = 100, last?: number) {
    // TODO: fix init
    if (!this.cwClient) {
      await this.init()
    }
    try {
      const queryMsg = {
        offers_by: {
          fiat_currency: args.fiatCurrency,
          offer_type: args.offerType,
          denom: args.denom,
          order: args.order,
          limit,
          last,
        },
      }
      const response = (await this.cwClient!.queryContractSmart(
        this.hubInfo.hubConfig.offer_addr,
        queryMsg
      )) as OfferResponse[]
      console.log('response >>> ', response)
      return response
    } catch (e) {
      throw DefaultError.fromError(e)
    }
  }

  async openTrade(trade: NewTrade) {
    const msg = { create: trade }
    console.log('Open Trade msg >> ', msg)
    if (this.cwClient instanceof SigningCosmWasmClient && this.signer) {
      try {
        const result = await this.cwClient.execute(
          this.getWalletAddress(),
          this.hubInfo.hubConfig.trade_addr,
          msg,
          'auto'
        )
        console.log('Open Trade result >> ', result)
        const trade_id = result.logs[0].events
          .find((e) => e.type === 'wasm')
          ?.attributes.find((a) => a.key === 'trade_id')?.value
        return Number(trade_id)
      } catch (e) {
        throw DefaultError.fromError(e)
      }
    } else {
      throw new WalletNotConnected()
    }
  }

  // TODO maybe we can do a single trades_query
  async fetchTrades(limit = 100, last?: number) {
    if (this.cwClient instanceof SigningCosmWasmClient) {
      const userAddr = this.getWalletAddress()
      // TODO fix init
      if (!this.cwClient) {
        await this.init()
      }
      try {
        // Query of trades as buyer
        const response = (await this.cwClient!.queryContractSmart(this.hubInfo.hubConfig.trade_addr, {
          trades: { user: userAddr, role: 'trader', limit, last },
        })) as TradeInfo[]
        console.log('response >>> ', response)
        return response
      } catch (e) {
        throw DefaultError.fromError(e)
      }
    } else {
      throw new WalletNotConnected()
    }
  }

  async fetchDisputedTrades(
    limit = 100,
    last?: number
  ): Promise<{ openDisputes: TradeInfo[]; closedDisputes: TradeInfo[] }> {
    if (this.cwClient instanceof SigningCosmWasmClient) {
      const userAddr = this.getWalletAddress()
      // TODO fix init
      if (!this.cwClient) {
        await this.init()
      }
      try {
        // Query of trades as buyer
        const queryMsg = { trades: { user: userAddr, role: 'arbitrator', limit, last } }
        const disputedTrades = (await this.cwClient!.queryContractSmart(
          this.hubInfo.hubConfig.trade_addr,
          queryMsg
        )) as TradeInfo[]
        const openDisputes = disputedTrades.filter((t) => t.trade.state === 'escrow_disputed')
        const closedDisputes = disputedTrades.filter((t) => t.trade.state !== 'escrow_disputed')
        const response: { openDisputes: TradeInfo[]; closedDisputes: TradeInfo[] } = { openDisputes, closedDisputes }
        console.log('response >>> ', response)
        return response
      } catch (e) {
        throw DefaultError.fromError(e)
      }
    } else {
      throw new WalletNotConnected()
    }
  }

  async fetchTradeDetail(tradeId: number) {
    // TODO fix init
    if (!this.cwClient) {
      await this.init()
    }
    try {
      const response = (await this.cwClient!.queryContractSmart(this.hubInfo.hubConfig.trade_addr, {
        trade: { id: tradeId },
      })) as TradeInfo
      return response
    } catch (e) {
      throw DefaultError.fromError(e)
    }
  }

  async fetchArbitrators() {
    // TODO: fix init
    if (!this.cwClient) {
      await this.init()
    }
    try {
      const queryMsg = {
        arbitrators: {
          limit: 100,
        },
      }
      const response = (await this.cwClient!.queryContractSmart(
        this.hubInfo.hubConfig.trade_addr,
        queryMsg
      )) as Arbitrator[]
      console.log('response >>> ', response)
      return response
    } catch (e) {
      throw DefaultError.fromError(e)
    }
  }

  async fetchOpenDisputes() {
    // TODO: fix init
    if (!this.cwClient) {
      await this.init()
    }
    try {
      const queryMsg = {
        trades: {
          user: this.getWalletAddress(),
          role: 'arbitrator',
          limit: 100,
        },
      }
      const response = (await this.cwClient!.queryContractSmart(
        this.hubInfo.hubConfig.trade_addr,
        queryMsg
      )) as TradeInfo[]
      console.log('response >>> ', response)
      return response
    } catch (e) {
      throw DefaultError.fromError(e)
    }
  }

  async updateFiatPrice(fiat: FiatCurrency, denom: Denom): Promise<DenomFiatPrice> {
    // TODO: fix init
    if (!this.cwClient) {
      await this.init()
    }
    try {
      const queryMsg = { price: { fiat, denom } }
      const response = (await this.cwClient!.queryContractSmart(
        this.hubInfo.hubConfig.price_addr,
        queryMsg
      )) as DenomFiatPrice
      console.log('response >>> ', response)
      return response
    } catch (e) {
      throw DefaultError.fromError(e)
    }
  }

  // TODO encrypt maker_contact field
  async acceptTradeRequest(tradeId: number, makerContact: string) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, {
      accept_request: { trade_id: tradeId, maker_contact: makerContact },
    })
  }

  async cancelTradeRequest(tradeId: number) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, {
      cancel_request: { trade_id: tradeId },
    })
  }

  async fundEscrow(tradeInfo: TradeInfo, makerContact?: string) {
    const hubConfig = this.hubInfo.hubConfig
    let fundAmount = Number(tradeInfo.trade.amount)
    console.log('amount: ', fundAmount)

    // If current user is the maker, add the fee to the amount to fund
    if (tradeInfo.offer.offer.owner === this.getWalletAddress()) {
      const burnAmount = Math.floor(hubConfig.burn_fee_pct * fundAmount)
      const chainAmount = Math.floor(hubConfig.chain_fee_pct * fundAmount)
      const warchestAmount = Math.floor(hubConfig.warchest_fee_pct * fundAmount)
      const totalFee = burnAmount + chainAmount + warchestAmount
      console.log('total fee:', totalFee)
      fundAmount += totalFee
      console.log('amount + fees: ', fundAmount)
    }

    const funds: Coin[] = [
      {
        amount: Math.floor(fundAmount).toFixed(0),
        denom: denomToValue(tradeInfo.trade.denom),
      },
    ]
    console.log('funds', funds)
    await this.changeTradeState(
      this.hubInfo.hubConfig.trade_addr,
      { fund_escrow: { trade_id: tradeInfo.trade.id, maker_contact: makerContact } },
      funds
    )
  }

  async setFiatDeposited(tradeId: number) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, {
      fiat_deposited: { trade_id: tradeId },
    })
  }

  async releaseEscrow(tradeId: number) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, {
      release_escrow: { trade_id: tradeId },
    })
  }

  async refundEscrow(tradeId: number) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, {
      refund_escrow: { trade_id: tradeId },
    })
  }

  async openDispute(tradeId: number, buyerContact: string, sellerContact: string) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, {
      dispute_escrow: {
        trade_id: tradeId,
        buyer_contact: buyerContact,
        seller_contact: sellerContact,
      },
    })
  }

  private async changeTradeState(addr: string, msg: Record<string, unknown>, funds?: Coin[]) {
    console.log('Trade State >> ', msg)
    if (this.cwClient instanceof SigningCosmWasmClient && this.signer) {
      try {
        const result = await this.cwClient.execute(this.getWalletAddress(), addr, msg, 'auto', undefined, funds)
        console.log('Trade State result >> ', result)
      } catch (e) {
        throw DefaultError.fromError(e)
      }
    } else {
      throw new WalletNotConnected()
    }
  }

  async newArbitrator(arbitrator: Arbitrator) {
    const msg = { new_arbitrator: arbitrator }
    console.log('New Arbitrator msg >> ', msg)
    if (this.cwClient instanceof SigningCosmWasmClient && this.signer) {
      try {
        const result = await this.cwClient.execute(
          this.getWalletAddress(),
          this.hubInfo.hubConfig.trade_addr,
          msg,
          'auto'
        )
        console.log('New arbitrator result >> ', result)
      } catch (e) {
        throw DefaultError.fromError(e)
      }
    } else {
      throw new WalletNotConnected()
    }
  }

  async settleDispute(tradeId: number, winner: string) {
    const msg = { settle_dispute: { trade_id: tradeId, winner } }
    console.log('msg >> ', msg)
    if (this.cwClient instanceof SigningCosmWasmClient && this.signer) {
      try {
        const result = await this.cwClient.execute(
          this.getWalletAddress(),
          this.hubInfo.hubConfig.trade_addr,
          msg,
          'auto'
        )
        console.log('result >> ', result)
      } catch (e) {
        throw DefaultError.fromError(e)
      }
    } else {
      throw new WalletNotConnected()
    }
  }

  // TODO extract this method
  private static async suggestChain(config: CosmosConfig) {
    try {
      await window.keplr?.experimentalSuggestChain({
        // Chain-id of the Osmosis chain.
        chainId: config.chainId,
        // The name of the chain to be displayed to the user.
        chainName: config.chainName,
        // RPC endpoint of the chain. In this case we are using blockapsis, as it's accepts connections from any host currently. No Cors limitations.
        rpc: config.rpcUrl,
        // REST endpoint of the chain.
        rest: config.lcdUrl,
        // Staking coin information
        stakeCurrency: {
          // Coin denomination to be displayed to the user.
          coinDenom: config.coinDenom,
          // Actual denom (i.e. uatom, uscrt) used by the blockchain.
          coinMinimalDenom: config.coinMinimalDenom,
          // # of decimal points to convert minimal denomination to user-facing denomination.
          coinDecimals: config.coinDecimals,
        },
        bip44: {
          // You can only set the coin type of BIP44.
          // 'Purpose' is fixed to 44.
          coinType: 118,
        },
        bech32Config: {
          bech32PrefixAccAddr: `${config.addressPrefix}`,
          bech32PrefixAccPub: `${config.addressPrefix}pub`,
          bech32PrefixValAddr: `${config.addressPrefix}valoper`,
          bech32PrefixValPub: `${config.addressPrefix}valoperpub`,
          bech32PrefixConsAddr: `${config.addressPrefix}valcons`,
          bech32PrefixConsPub: `${config.addressPrefix}valconspub`,
        },
        // List of all coin/tokens used in this chain.
        currencies: [
          {
            coinDenom: config.coinDenom,
            coinMinimalDenom: config.coinMinimalDenom,
            coinDecimals: config.coinDecimals,
          },
        ],
        // List of coin/tokens used as a fee token in this chain.
        feeCurrencies: [
          {
            coinDenom: config.coinDenom,
            coinMinimalDenom: config.coinMinimalDenom,
            coinDecimals: config.coinDecimals,
          },
        ],
      })
    } catch (e) {
      console.log(e)
    }
  }
}
