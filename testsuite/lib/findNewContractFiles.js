import * as fs from "fs";
import { _readNewContractChecksums } from "./_readNewContractChecksums.js";
import { _readCachedContractChecksums } from "./_readCachedContractChecksums.js";
import { _writeCachedContractChecksums } from "./_writeCachedContractChecksums.js";

function findNewContractFiles(path) {
  const cachedContractChecksums = _readCachedContractChecksums();

  const newContractChecksums = _readNewContractChecksums(path);

  const newContractFiles = [];

  for (const contract in newContractChecksums) {
    if (Object.hasOwnProperty.call(newContractChecksums, contract)) {
      const cachedChecksum = cachedContractChecksums[contract];
      const newChecksum = newContractChecksums[contract];

      if (cachedChecksum !== newChecksum)
        newContractFiles.push([contract, newChecksum]);
    }
  }

  return newContractFiles;
}

export { findNewContractFiles };
