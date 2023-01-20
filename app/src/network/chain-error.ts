export abstract class ChainError extends Error {}

export class DefaultError extends ChainError {
  constructor(message = 'Something wrong happened.') {
    super(message)
    // Set the prototype explicitly.
    Object.setPrototypeOf(this, DefaultError.prototype)
  }

  static fromError(e: any) {
    if (Object.hasOwn(e, 'message')) {
      const regex = /(; message index: 0: dispatch: submessages:|; message index: 0:) (.*): execute/g.exec(e.message)
      const message = regex?.at(regex?.length - 1) ?? e.message
      return new DefaultError(message)
    }
  }
}

export class WalletNotInstalled extends ChainError {
  constructor(message = 'No wallet detected. Please install and try again.') {
    super(message)
    // Set the prototype explicitly.
    Object.setPrototypeOf(this, WalletNotInstalled.prototype)
  }
}

export class WalletNotConnected extends ChainError {
  constructor(message = 'You need to connect your wallet.') {
    super(message)
    // Set the prototype explicitly.
    Object.setPrototypeOf(this, WalletNotConnected.prototype)
  }
}
