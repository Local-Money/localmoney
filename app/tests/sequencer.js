/* eslint-disable eslint-comments/no-unlimited-disable */
const Sequencer = require('@jest/test-sequencer').default

class CustomSequencer extends Sequencer {
  sort(tests) {
    const sorted = []
    tests.forEach((test) => {
      if (test.path.includes('setup.spec')) {
        sorted[0] = test
      } else if (test.path.includes('price.spec')) {
        sorted[1] = test
      } else if (test.path.includes('fees.spec')) {
        sorted[2] = test
      } else if (test.path.includes('pagination.spec')) {
        sorted[3] = test
      } else if (test.path.includes('arbitration.spec')) {
        sorted[4] = test
      } else if (test.path.includes('trade.spec.ts')) {
        sorted[5] = test
      }
    })
    return sorted
  }
}

module.exports = CustomSequencer
/* eslint-enable */
