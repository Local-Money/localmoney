import makerSecrets from './fixtures/maker_secrets.json'
import takerSecrets from './fixtures/taker_secrets.json'
import adminSecrets from './fixtures/admin_secrets.json'

export async function encryptDataMocked(key: string, data: string): Promise<string> {
  let encryptedData = ''
  if (key === makerSecrets.publicKey && data === 'maker001') {
    encryptedData =
      'X0+XfKDZWng3NLJbGb5klwebCT/kGguuN7QTtQJhrCSkLpyAH6xjynZei1lLOXQll336pZbLkoxguWNZ4fnBAaYnkcE0GAPEIQlh00AGJl7w3++af5UTSaJ1YT2g71uu0eDAHU91FOb5Zr2djf0KbZ4D4l/5k/DOnY8YEFqyiIUXfI9oE0m9MIZDzaKsTh9TS06J+IhNlFvAi+TlrHWoMlE/JpBbsSr+A2gLHzte1cg8XiPJfQAU5U6300SwybXpG/3AHpl4RKLjwWb09GIUu9bOs7ERezSrRxrrN5ia+DBufAfi9l0pIm7lixll1wOuwS1Ww+7chpeShlFxNAhbdA=='
  }
  if (key === takerSecrets.publicKey && data === 'maker001') {
    encryptedData =
      'X0+XfKDZWng3NLJbGb5klwebCT/kGguuN7QTtQJhrCSkLpyAH6xjynZei1lLOXQll336pZbLkoxguWNZ4fnBAaYnkcE0GAPEIQlh00AGJl7w3++af5UTSaJ1YT2g71uu0eDAHU91FOb5Zr2djf0KbZ4D4l/5k/DOnY8YEFqyiIUXfI9oE0m9MIZDzaKsTh9TS06J+IhNlFvAi+TlrHWoMlE/JpBbsSr+A2gLHzte1cg8XiPJfQAU5U6300SwybXpG/3AHpl4RKLjwWb09GIUu9bOs7ERezSrRxrrN5ia+DBufAfi9l0pIm7lixll1wOuwS1Ww+7chpeShlFxNAhbdA=='
  }
  if (key === adminSecrets.publicKey && data === 'maker001') {
    encryptedData =
      'x70c8iGEPr7zzR4DdWurgIAMnfoQLKYIzZ4d0j64ArNniumD2oxDGb1etMqEjZM758HTkh0jt75GOZ6Q1PCpBYbOBO8fXGlsrMqxr860ZCgMuoN98tZuG4N2LCY/+6LPxzv7MT2kNld8V+EwPxihztPvCaZ6POogcPovhLHR9wFBipiTUxQqBAPUL9odKOpebyyg0fkJN0EOnqf/4ssJJW8xnZFFz4JvHcC8tQT4ztQUlOMJURpUkVIc9jzY1PRgNV3mq/z252KPk3cNvbgWzQ8Y5UEjc/a6dpN6HEu3g8Cty1d4cPTgKm5PTZ97H9BpNnB0nkWi6A0hdxQMXgYESw=='
  }
  if (key === takerSecrets.publicKey && data === 'taker001') {
    encryptedData =
      'IGHwpmkcb5TsWQmn25XTjwh9rfHSryvhLpKPWNq/T+xq56548Qii081kvVxjSsmkCjEp/TDTOz0g2KQyF5TsAS8kr077qj4SavqMad3rAqlHlXddQrUnPFcDhJ3JeO8YyGQ7HnifHE8Anm56SA78I7juWnsG9g/7xu00Gkeza0O5HE8S7f2vQc5wKjbraX2uEdDHWi9Mri7d3DT5u4+pdhAMYBdlwmCNeJLCdbV2GduRBEH4uJzSqy87RhRDGd5U91apl8viM9ZbMZMhYu005VgXY9VyZvxv7WvptYKzahDFFHecWhOur9FyvEkg5ho33diujqmKS3F+pdyk+ScagQ=='
  }
  if (key === makerSecrets.publicKey && data === 'taker001') {
    encryptedData =
      'IGHwpmkcb5TsWQmn25XTjwh9rfHSryvhLpKPWNq/T+xq56548Qii081kvVxjSsmkCjEp/TDTOz0g2KQyF5TsAS8kr077qj4SavqMad3rAqlHlXddQrUnPFcDhJ3JeO8YyGQ7HnifHE8Anm56SA78I7juWnsG9g/7xu00Gkeza0O5HE8S7f2vQc5wKjbraX2uEdDHWi9Mri7d3DT5u4+pdhAMYBdlwmCNeJLCdbV2GduRBEH4uJzSqy87RhRDGd5U91apl8viM9ZbMZMhYu005VgXY9VyZvxv7WvptYKzahDFFHecWhOur9FyvEkg5ho33diujqmKS3F+pdyk+ScagQ=='
  }
  if (key === adminSecrets.publicKey && data === 'taker001') {
    encryptedData =
      'Iyb+yejSrRAaQyddH1oC7Z2MXMAgH6WvUzIuZ1bqefjqVPNvngglJMAjbGdRCGh6kFd4MiiaWg7KM71IDWkhohaVWhJ7i+t8sTvWYjosRlpZPE76wbYls8svsZGo0p0YCj3vKTK5J43gsYiMi3XTPYb9ooMSos7t4I/5QQwmpnFopi6PhQwI3Kk+bZLK02CLEeinegN1tS1THYLdSdd9hQG08jf6WGGtHLBlzGXJlQwlYOE8Cv9q8u9QkldD04ehiFwmtIInnYomZ4VRUu962jh6uy63zIjpA+tmtsAG1qEquJEw7/f2fUWJ64SDvTvDCommUPVg3RliDbweDOefag=='
  }
  return Promise.resolve(encryptedData)
}

export async function decryptDataMocked(key: string, data: string): Promise<string> {
  let decryptedData = ''
  if (
    key === takerSecrets.privateKey &&
    data ===
      'X0+XfKDZWng3NLJbGb5klwebCT/kGguuN7QTtQJhrCSkLpyAH6xjynZei1lLOXQll336pZbLkoxguWNZ4fnBAaYnkcE0GAPEIQlh00AGJl7w3++af5UTSaJ1YT2g71uu0eDAHU91FOb5Zr2djf0KbZ4D4l/5k/DOnY8YEFqyiIUXfI9oE0m9MIZDzaKsTh9TS06J+IhNlFvAi+TlrHWoMlE/JpBbsSr+A2gLHzte1cg8XiPJfQAU5U6300SwybXpG/3AHpl4RKLjwWb09GIUu9bOs7ERezSrRxrrN5ia+DBufAfi9l0pIm7lixll1wOuwS1Ww+7chpeShlFxNAhbdA=='
  ) {
    decryptedData = 'maker001'
  }
  if (
    key === makerSecrets.privateKey &&
    data ===
      'IGHwpmkcb5TsWQmn25XTjwh9rfHSryvhLpKPWNq/T+xq56548Qii081kvVxjSsmkCjEp/TDTOz0g2KQyF5TsAS8kr077qj4SavqMad3rAqlHlXddQrUnPFcDhJ3JeO8YyGQ7HnifHE8Anm56SA78I7juWnsG9g/7xu00Gkeza0O5HE8S7f2vQc5wKjbraX2uEdDHWi9Mri7d3DT5u4+pdhAMYBdlwmCNeJLCdbV2GduRBEH4uJzSqy87RhRDGd5U91apl8viM9ZbMZMhYu005VgXY9VyZvxv7WvptYKzahDFFHecWhOur9FyvEkg5ho33diujqmKS3F+pdyk+ScagQ=='
  ) {
    decryptedData = 'taker001'
  }
  return Promise.resolve(decryptedData)
}
