import type { CosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { DirectSecp256k1HdWallet } from '@cosmjs/proto-signing'
import { Decimal } from '@cosmjs/math'
import dotenv from 'dotenv'
import { CosmosChain } from '~/network/cosmos/CosmosChain'
import type { HubConfig } from '~/types/components.interface'
import type { HubInfo } from '~/network/cosmos/config'
dotenv.config()

export class TestCosmosChain extends CosmosChain {
  async connectWallet(): Promise<void> {
    const seed = process.env.SEED ? process.env.SEED : ''
    this.signer = await DirectSecp256k1HdWallet.fromMnemonic(seed, { prefix: process.env.ADDR_PREFIX })
    // get first account
    const accounts = await this.signer.getAccounts()
    this.account = accounts[0]
    this.cwClient = await SigningCosmWasmClient.connectWithSigner(this.config.rpcUrl, this.signer, {
      gasPrice: {
        amount: Decimal.fromUserInput('0.0025', 100),
        denom: this.config.coinMinimalDenom,
      },
    })
  }

  getCwClient(): CosmWasmClient | SigningCosmWasmClient | undefined {
    return this.cwClient
  }

  async updateHub(hubAddress: string) {
    const hubConfig = await this.cwClient!.queryContractSmart(hubAddress, { config: {} }) as HubConfig
    this.hubInfo = { hubAddress, hubConfig }
  }

  getHubInfo(): HubInfo {
    return this.hubInfo
  }
}
