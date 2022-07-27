export interface SelectInfo {
  display: string
  icon?: string
}

export function getSelectInfo(dataSource: Map<string, SelectInfo>, fiatCode: string): SelectInfo {
  return dataSource.get(fiatCode)!
}
