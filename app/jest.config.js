/** @type {import('ts-jest/dist/types').InitialOptionsTsJest} */
/** "isolatedModules: true" disables Type checking on tests but increases compilation time dramatically **/
module.exports = {
  preset: 'ts-jest',
  testEnvironment: 'node',
  globals: {
    'ts-jest': {
      isolatedModules: true,
    },
  },
}
