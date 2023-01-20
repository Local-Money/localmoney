# Local Money Testsuite
The Testsuite is a set of tests that are used to verify the proper functioning of the protocol. It involves instantiating the protocol and running integration tests.
### Commands

To run all tests:
```bash
yarn test
```

To only instantiate the protocol:
```bash
yarn test -t 'setup protocol'
```

To setup fees configurations:
```bash
yarn test -t 'fees'
```

To pre-populate the price contract, you can run this code:
```bash
yarn test -t 'price test'
```

You can also combine all this tests using `|` between each code, example:
```bash
yarn test -t 'setup protocol|fees|price test'
```