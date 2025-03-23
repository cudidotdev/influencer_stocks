import { ContractClient } from "./contract/Contract.client";

export async function getOpenBidsByStock(
  contractClient: ContractClient,
  stockId: number,
) {
  const res = await contractClient.getOpenBidsByStock({
    stockId,
  });

  return res.bids;
}
