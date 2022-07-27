import type { GetOffer } from '~/types/components.interface'
import { checkMicroDenomAvailable } from '~/utils/denom'
import { checkFiatAvailable } from '~/utils/fiat'

export function checkValidOffer(offer: GetOffer): boolean {
  return checkMicroDenomAvailable(offer.denom.native) && checkFiatAvailable(offer.fiat_currency)
}
