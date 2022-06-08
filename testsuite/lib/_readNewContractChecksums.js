import * as fs from "fs";

function _readNewContractChecksums(path) {
  const data = fs.readFileSync(path + "checksums.txt", {
    encoding: "utf8",
    flag: "r",
  });

  const lines = data.split(/\r\n|\n/);

  const contractChecksums = {};

  lines.forEach((line) => {
    const entries = line.split("  ");
    if (entries[1]) contractChecksums[entries[1]] = entries[0];
  });

  return contractChecksums;
}

export { _readNewContractChecksums };
