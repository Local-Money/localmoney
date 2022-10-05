import makerSecrets from './fixtures/maker_secrets.json'
import takerSecrets from './fixtures/taker_secrets.json'

export async function encryptDataMocked(key: string, data: string): Promise<string> {
  let encryptedData = ''
  if (key === makerSecrets.publicKey && data === 'maker001') {
    encryptedData = 'DD923A398FA24D14C9B18A3167806D9AF275B5853FEE92916327CE4165E1C286'
  }
  if (key === takerSecrets.publicKey && data === 'maker001') {
    encryptedData = 'DD923A398FA24D14C9B18A3167806D9AF275B5853FEE92916327CE4165E1C286'
  }
  if (key === takerSecrets.publicKey && data === 'taker001') {
    encryptedData = '20102E96BA164BC77C3C8E572D4AE3F37F2F9BA2A37D50380BDFC174B6F8A6EF'
  }
  if (key === makerSecrets.publicKey && data === 'taker001') {
    encryptedData = '20102E96BA164BC77C3C8E572D4AE3F37F2F9BA2A37D50380BDFC174B6F8A6EF'
  }
  return Promise.resolve(encryptedData)
}

export async function decryptDataMocked(key: string, data: string): Promise<string> {
  let decryptedData = ''
  if (key === takerSecrets.privateKey && data === 'DD923A398FA24D14C9B18A3167806D9AF275B5853FEE92916327CE4165E1C286') {
    decryptedData = 'maker001'
  }
  if (key === makerSecrets.privateKey && data === '20102E96BA164BC77C3C8E572D4AE3F37F2F9BA2A37D50380BDFC174B6F8A6EF') {
    decryptedData = 'taker001'
  }
  return Promise.resolve(decryptedData)
}
