import { readFileSync } from "fs";

function loadCachedFactory() {
  console.log("* Loading cached factory..");
  try {
    const factory = JSON.parse(
      readFileSync(`cache/factory_${process.env.NETWORK_ENV}.json`, "utf8")
    );
    // console.log("cached factory :>> ", factory);

    return factory;
  } catch ({ message }) {
    console.error(message);
    console.log(
      "-> First instantiate the contracts with: npm run instantiate or npm run upload!"
    );
    throw new Error(message);
  }
}

export { loadCachedFactory };
