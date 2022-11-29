export function formatTimeLimit(time: Date): string {
  const hours = time.getUTCHours()
  const mins = time.getUTCMinutes()
  let timeLimit = ''
  if (hours > 0) {
    const suffix = hours === 1 ? 'hour' : 'hours'
    timeLimit += `${hours} ${suffix} `
  }
  if (mins > 0) {
    const suffix = mins === 1 ? 'minute' : 'minutes'
    timeLimit += `${mins} ${suffix}`
  }
  return timeLimit
}
export function formatTimer(timer: Date, finalState: string): string {
  const hours = zeroPrefix(timer.getUTCHours(), 2)
  const mins = zeroPrefix(timer.getUTCMinutes(), 2)
  const secs = zeroPrefix(timer.getUTCSeconds(), 2)
  const timerRunning = timer.getUTCHours() !== 0 ? `${hours}h ${mins}m ${secs}s` : `${mins}m ${secs}s`
  return timer.getTime() > 0 ? timerRunning : finalState
}

function zeroPrefix(num: number, digit: number): string {
  let zero = ''
  for (let i = 0; i < digit; i++) {
    zero += '0'
  }
  return (zero + num).slice(-digit)
}
