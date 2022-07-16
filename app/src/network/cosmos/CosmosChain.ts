import type { AccountData, OfflineSigner } from '@cosmjs/launchpad'
import type { OfflineDirectSigner } from '@cosmjs/proto-signing'
import { Decimal } from '@cosmjs/math'
import { CosmWasmClient, SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import type { Coin } from '@cosmjs/stargate'
import type {
  Denom,
  FetchOffersArgs,
  GetOffer,
  HubConfig, NewTrade,
  PatchOffer,
  PostOffer,
  Trade,
  TradeInfo,
} from '~/types/components.interface'
import type { Chain } from '~/network/Chain'
import type { CosmosConfig, HubInfo } from '~/network/cosmos/config'
import { DefaultError, WalletNotConnected, WalletNotInstalled } from '~/network/chain-error'

export class CosmosChain implements Chain {
  private readonly config: CosmosConfig
  private hubInfo: HubInfo

  private signer?: OfflineSigner | OfflineDirectSigner
  private account?: AccountData
  private cwClient?: CosmWasmClient | SigningCosmWasmClient

  constructor(config: CosmosConfig, hubInfo: HubInfo) {
    this.config = config
    this.hubInfo = hubInfo
  }

  async init() {
    this.cwClient = await CosmWasmClient.connect(this.config.rpcUrl)
    this.hubInfo.hubConfig = await this.cwClient.queryContractSmart(
      this.hubInfo.hubAddress,
      { config: {} },
    ) as HubConfig
    // console.log("Factory config >> ", this.hubInfo.hubConfig)
  }

  async connectWallet() {
    if (!window.getOfflineSigner || !window.keplr || !window.getOfflineSignerAuto) {
      throw new WalletNotInstalled()
    }
    else {
      await CosmosChain.suggestChain(this.config)
      await window.keplr.enable(this.config.chainId)
      this.signer = await window.getOfflineSignerAuto(this.config.chainId);
      // get first account
      [this.account] = await this.signer.getAccounts()
      this.cwClient = await SigningCosmWasmClient.connectWithSigner(this.config.rpcUrl, this.signer, {
        gasPrice: {
          amount: Decimal.fromUserInput('0.0025', 100),
          denom: this.config.coinMinimalDenom,
        },
      })
    }
  }

  getWalletAddress(): string {
    return this.account ? this.account.address : 'undefined'
  }

  async createOffer(postOffer: PostOffer) {
    const msg = { create: { offer: postOffer } }
    console.log('Create offer msg >> ', msg)
    if (this.cwClient instanceof SigningCosmWasmClient && this.signer) {
      try {
        const result = await this.cwClient.execute(
          this.getWalletAddress(),
          this.hubInfo.hubConfig.offer_addr,
          msg,
          'auto',
        )
        console.log('Create offer result >> ', result)
      }
      catch (e) {
        throw new DefaultError()
      }
    }
    else {
      throw new WalletNotConnected()
    }
  }

  async updateOffer(updateOffer: PatchOffer) {
    const msg = { update_offer: { offer_update: updateOffer } }
    console.log('Update offer msg >> ', msg)
    if (this.cwClient instanceof SigningCosmWasmClient && this.signer) {
      try {
        const result = await this.cwClient.execute(
          this.getWalletAddress(),
          this.hubInfo.hubConfig.offer_addr,
          msg,
          'auto',
        )
        console.log('Update offer result >> ', result)
      }
      catch (e) {
        throw new DefaultError()
      }
    }
    else {
      throw new WalletNotConnected()
    }
  }

  async fetchMyOffers() {
    if (this.cwClient instanceof SigningCosmWasmClient) {
      try {
        const response = await this.cwClient.queryContractSmart(
          this.hubInfo.hubConfig.offer_addr,
          {
            offers_query: {
              owner: this.getWalletAddress(),
              limit: 10,
              order: 'asc',
            },
          }) as GetOffer[]
        console.log('response >> ', response)
        return response
      }
      catch (e) {
        throw new DefaultError()
      }
    }
    else {
      throw new WalletNotConnected()
    }
  }

  async fetchOffers(args: FetchOffersArgs) {
    console.log('args >>> ', args)
    // TODO fix init
    if (!this.cwClient)
      await this.init()
    try {
      const response = await this.cwClient!.queryContractSmart(
        this.hubInfo.hubConfig.offer_addr,
        {
          offers_by_type_fiat: {
            fiat_currency: args.fiatCurrency,
            offer_type: args.offerType,
            // min: "",
            // max: "",
            limit: 10,
            order: 'asc',
          },
        }) as GetOffer[]
      console.log('response >>> ', response)
      return response
    }
    catch (e) {
      throw new DefaultError()
    }
  }

  async openTrade(trade: NewTrade) {
    let response = ''
    const msg = { create: trade }
    console.log('Open Trade msg >> ', msg)
    if (this.cwClient instanceof SigningCosmWasmClient && this.signer) {
      try {
        const result = await this.cwClient.execute(
          this.getWalletAddress(),
          this.hubInfo.hubConfig.trade_addr,
          msg,
          'auto',
        )
        console.log('Open Trade result >> ', result)
        // TODO should we try to get this info this way?
        response = result.logs[0].events[2].attributes[1].value
      }
      catch (e) {
        throw new DefaultError()
      }
    }
    else {
      throw new WalletNotConnected()
    }
    return response
  }

  // TODO maybe we can do a single trades_query
  async fetchTrades() {
    if (this.cwClient instanceof SigningCosmWasmClient) {
      const userAddr = this.getWalletAddress()
      // TODO fix init
      if (!this.cwClient)
        await this.init()
      try {
        // Query of trades as buyer
        const queryAsBuyerMsg = { trades: { user: userAddr, index: 'buyer', limit: 100 } }
        const tradesAsBuyer = await this.cwClient!.queryContractSmart(
          this.hubInfo.hubConfig.trade_addr,
          queryAsBuyerMsg,
        ) as TradeInfo[]

        // Query of trades as seller
        const queryAsSellerMsg = { trades: { user: userAddr, index: 'seller', limit: 100 } }
        const tradesAsSeller = await this.cwClient!.queryContractSmart(
          this.hubInfo.hubConfig.trade_addr,
          queryAsSellerMsg,
        ) as TradeInfo[]

        // Join all trades
        const response: TradeInfo[] = tradesAsBuyer.concat(tradesAsSeller)
        console.log('response >>> ', response)
        return response
      }
      catch (e) {
        throw new DefaultError()
      }
    }
    else {
      throw new WalletNotConnected()
    }
  }

  async fetchTradeDetail(tradeId: string) {
    // TODO fix init
    if (!this.cwClient)
      await this.init()
    try {
      const response = await this.cwClient!.queryContractSmart(
        this.hubInfo.hubConfig.trade_addr,
        { trade: { id: tradeId } },
      ) as Trade
      console.log('response >>> ', response)
      return response
    }
    catch (e) {
      // TODO error state
      throw new DefaultError()
    }
  }

  async acceptTradeRequest(tradeId: string) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, { accept_request: { trade_id: tradeId } })
  }

  async cancelTradeRequest(tradeId: string) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, { cancel_request: { trade_id: tradeId } })
  }

  async fundEscrow(tradeId: string, amount: string, denom: Denom) {
    let fundAmount = Number(amount)
    const localFee = fundAmount * 0.01
    fundAmount += localFee
    const funds: Coin[] = [{
      amount: `${fundAmount}`,
      denom: denom.native,
    }]
    console.log(funds)
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, { fund_escrow: { trade_id: tradeId } }, funds)
  }

  async setFiatDeposited(tradeId: string) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, { fiat_deposited: { trade_id: tradeId } })
  }

  async releaseEscrow(tradeId: string) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, { release_escrow: { trade_id: tradeId } })
  }

  async refundEscrow(tradeId: string) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, { refund_escrow: { trade_id: tradeId } })
  }

  async openDispute(tradeId: string) {
    await this.changeTradeState(this.hubInfo.hubConfig.trade_addr, { dispute_escrow: { trade_id: tradeId } })
  }

  private async changeTradeState(
    tradeId: string,
    msg: Record<string, unknown>,
    funds?: Coin[],
  ) {
    console.log('Trade State >> ', msg)
    if (this.cwClient instanceof SigningCosmWasmClient && this.signer) {
      try {
        const result = await this.cwClient.execute(
          this.getWalletAddress(),
          tradeId,
          msg,
          'auto',
          undefined,
          funds,
        )
        console.log('Trade State result >> ', result)
      }
      catch (e) {
        // TODO manage error
        throw new DefaultError()
      }
    }
    else {
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
        currencies: [{
          coinDenom: config.coinDenom,
          coinMinimalDenom: config.coinMinimalDenom,
          coinDecimals: config.coinDecimals,
        }],
        // List of coin/tokens used as a fee token in this chain.
        feeCurrencies: [{
          coinDenom: config.coinDenom,
          coinMinimalDenom: config.coinMinimalDenom,
          coinDecimals: config.coinDecimals,
        }],
      })
    }
    catch (e) {
      console.log(e)
    }
  }
}
