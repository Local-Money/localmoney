import * as fs from "fs";

function _readCachedContractChecksums() {
  let cachedContractChecksums = {};

  try {
    cachedContractChecksums = JSON.parse(
      fs.readFileSync(
        `cache/contractChecksums_${process.env.NETWORK_ENV}.json`,
        "utf8"
      )
    );
  } catch (e) {
    console.log(
      "** Couldn't read cached contract checksums, starting fresh with empty object."
    );
  }
  return cachedContractChecksums;
}

export { _readCachedContractChecksums };
