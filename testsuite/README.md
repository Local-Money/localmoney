# LocalTerra Test Suite

Automated, parallel testing of real user behaviors against terrarium and bombay.

![localterra-testsuit](https://user-images.githubusercontent.com/92374012/153931405-697eccac-9ea6-418c-ad14-d3cea25af074.png)


## Run the entire test suite in parallel

```
npm run upload:terrarium
```

1.  Uploads the wasm files to terrarium
2.  Instantiates the contracts before EACH test for a clean fixture
3.  Runs the tests in parralel

## Run an individual suit

Allows `it.only()` and `describe.only()`.

```
npm run test:terrarium:disputeResolution
```

1.  Instantiates the contracts before the test for a clean fixture
2.  Runs the test

## Run a dirty single test

This is extremely fast, especially useful with `it.only()`for queries.

```
DIRTY_RUN=true npm run test:terrarium:disputeResolution
```

1.  Reuses the last cached contracts for a dirty fixture
2.  Runs the test

## Run a test against an existing factoryAddr

Test a live deploy, make liberal use of `describe.only()` and `it.only()`. Great for testing queries against live data.

```
FACTORY_ADDR="terra1um2pufv3zkfzqmeksp3xekecwnu6hrl66g3ukq" npm run test:terrarium:disputeResolution
```

1.  Fetches the `factoryCfg` of the live deployed instance
2.  Runs the test
