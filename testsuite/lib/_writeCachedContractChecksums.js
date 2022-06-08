import * as fs from "fs";

function _writeCachedContractChecksums(cachedContractChecksums) {
  fs.writeFileSync(
    `cache/contractChecksums_${process.env.NETWORK_ENV}.json`,
    JSON.stringify(cachedContractChecksums),
    "utf8"
  );
}

export { _writeCachedContractChecksums };
