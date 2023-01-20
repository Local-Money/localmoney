import type { ToastInterface } from 'vue-toastification'
import type { ChainError } from '~/network/chain-error'

export class FeedbackHandler {
  private toast: ToastInterface
  public constructor(toast: ToastInterface) {
    this.toast = toast
  }

  public error(e: any) {
    // We can validate each type of error here
    const message = (e as ChainError).message
    this.toast.error(message)
  }

  public success(message: string) {
    this.toast.success(message)
  }
}

export default function useFeedbackHandle(toast: ToastInterface): FeedbackHandler {
  return new FeedbackHandler(toast)
}
