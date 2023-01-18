import type { ChainError } from '~/network/chain-error'

export class ListResult<T> {
  readonly data: T[]
  readonly error: ChainError | null
  readonly loading: boolean
  private loadingMore: boolean
  private enableLoadMore: boolean

  private constructor(loading: boolean, data: T[], error: ChainError | null, enableLoadMore = false) {
    this.data = data
    this.error = error
    this.loading = loading
    this.loadingMore = false
    this.enableLoadMore = enableLoadMore
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
    return this.loading && this.data.length === 0
  }

  isLoadingMore(): boolean {
    return this.loadingMore
  }

  showLoadMore(): boolean {
    return this.enableLoadMore
  }

  setLoadingMore() {
    this.loadingMore = true
  }

  addMoreItems(items: T[], pageLimit = 30) {
    this.data.push(...items)
    this.enableLoadMore = items.length === pageLimit
    this.loadingMore = false
  }

  static success<T>(data: T[], pageLimit = 30) {
    return new ListResult<T>(false, data, null, data.length === pageLimit)
  }

  static error<T>(error: ChainError) {
    return new ListResult<T>(false, [], error)
  }

  static loading<T>() {
    return new ListResult<T>(true, [], null)
  }
}
