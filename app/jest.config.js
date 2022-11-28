/** @type {import('@ts-jest/dist/types').InitialOptionsTsJest} */
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'jsdom',
  testSequencer: '<rootDir>/tests/sequencer.js',
  globals: {
    'ts-jest': {
      isolatedModules: true,
    },
  },
  // Test are actually running in node env so we need to add custom export conditons
  // https://github.com/vuejs/test-utils/issues/234#issuecomment-1133672109
  testEnvironmentOptions: {
    customExportConditions: ['node', 'node-addons'],
  },
  moduleFileExtensions: ['json', 'js', 'jsx', 'ts', 'tsx', 'vue'],
  moduleNameMapper: {
    '^~/(.*)$': '<rootDir>/src/$1',
  },
  collectCoverage: false,
}
