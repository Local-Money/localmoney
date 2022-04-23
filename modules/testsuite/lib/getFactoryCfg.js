import { instantiateFactory } from "./instantiateFactory.js";
import { loadCachedFactory } from "./loadCachedFactory.js";
import { queryFactoryCfg } from "./queryFactoryCfg.js";

async function getFactoryCfg(client, user) {
  if (process.env.FACTORY_ADDR)
    // run against existing live factory, don't bother uploading or initiating
    return await queryFactoryCfg(client, process.env.FACTORY_ADDR);

  const factory =
    process.env.DIRTY_RUN === "true"
      ? loadCachedFactory()
      : await instantiateFactory(client, user, { cache: true });

  return factory.factoryCfg;
}

export { getFactoryCfg };
