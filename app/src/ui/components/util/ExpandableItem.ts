export class ExpandableItem<T> {
  data: T
  isExpanded: boolean

  constructor(item: T) {
    this.data = item
    this.isExpanded = false
  }
}
