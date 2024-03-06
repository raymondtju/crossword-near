export const nearConnectionConfig = {
  networkId: "testnet",
  // keyStore: myKeyStore, // first create a key store
  contractName: "crossword.ferro.testnet",
  nodeUrl: "https://rpc.testnet.near.org",
  walletUrl: "https://testnet.mynearwallet.com/",
  helperUrl: "https://helper.testnet.near.org",
  explorerUrl: "https://testnet.nearblocks.io",
};

import * as NearAPI from "near-api-js";
import { CodeResult } from "near-api-js/lib/providers/provider";

export async function viewMethodOnContract(
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  nearConfig: any,
  methodName: string
) {
  console.log(1);

  const provider = new NearAPI.providers.JsonRpcProvider(nearConfig.nodeUrl);
  // console.log(provider);

  const rawResult = (await provider.query(
    `call/${nearConfig.contractName}/${methodName}`,
    ""
  )) as CodeResult;

  const hashed = rawResult.result.map((x) => String.fromCharCode(x)).join("");

  return JSON.parse(hashed);
}
