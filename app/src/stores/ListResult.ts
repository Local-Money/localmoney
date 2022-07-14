import type { ChainError } from '~/network/chain-error';

export class ListResult<T> {

  readonly loading: boolean
  readonly data: T[]
  readonly error: ChainError | null

  private constructor(loading: boolean, data: T[], error: ChainError | null) {
    this.data = data
    this.error = error
    this.loading = loading
  }

  isSuccess(): boolean {
    return !this.loading && this.error === null && this.data.length > 0
  }

  isEmpty(): boolean {
    return !this.loading && this.error === null && this.data.length === 0
  }

  isError(): boolean {
    return !this.loading && this.error !== null
  }

  isLoading(): boolean {
    return this.loading
  }

  static success<T>(data: T[]) {
    return new ListResult<T>(false, data, null)
  }

  static error<T>(error: ChainError) {
    return new ListResult<T>(false, [], error)
  }

  static loading<T>() {
    return new ListResult<T>(true, [], null)
  }
}
