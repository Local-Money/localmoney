/* eslint-disable eslint-comments/no-unlimited-disable */
const Sequencer = require('@jest/test-sequencer').default

class CustomSequencer extends Sequencer {
  sort(tests) {
    const sorted = []
    tests.forEach((test) => {
      if (test.path.includes('hub.spec')) {
        sorted[0] = test
      } else if (test.path.includes('price.spec')) {
        sorted[1] = test
      } else if (test.path.includes('arbitration.spec')) {
        sorted[2] = test
      } else if (test.path.includes('trade.spec.ts')) {
        sorted[3] = test
      }
    })
    return sorted
  }
}

module.exports = CustomSequencer
/* eslint-enable */
