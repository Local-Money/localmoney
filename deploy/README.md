# Local Money Deploy

These scripts allow you to upload the `.wasm` files after they have been compiled.

### Commands

To upload all wasm compiled you can run: 
```bash
yarn deploy:all

#expands

DEPLOY=all CONTRACTS=../contracts/artifacts ./node_modules/node/bin/node index.js
```

To upload only one wasm compiled you can run: 
```bash
DEPLOY={CONTRACT_NAME} CONTRACTS=../contracts/artifacts ./node_modules/node/bin/node index.js
```