import * as fs from "fs";

function _readCachedCodeIds() {
  let cachedCodeIds = {};

  try {
    cachedCodeIds = JSON.parse(
      fs.readFileSync(`cache/codeIds_${process.env.NETWORK_ENV}.json`, "utf8")
    );
  } catch (e) {
    console.log(
      "** Couldn't read cached codeIds, starting fresh with empty object."
    );
  }
  return cachedCodeIds;
}

export { _readCachedCodeIds };
