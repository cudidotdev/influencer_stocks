import { ContractClient } from "./contract/Contract.client";
import { getStockById, getStockValue } from "./stocks";

export type Share = {
  id: number;
  stock_id: number;
  ticker: string;
  no_of_shares: number;
  value_per_share: string;
  total_value: string;
};

export async function getSharesbyOwner(contractClient: ContractClient) {
  const shares_res = await contractClient.getSharesByOwner({
    owner: contractClient.sender,
  });

  const shares: Share[] = [];

  for (const share of shares_res.shares) {
    const value = await getStockValue(contractClient, share.stock_id);

    const ticker = (await getStockById(contractClient, share.stock_id)).ticker;

    const formatted_shares: Share = {
      id: share.id,
      stock_id: share.stock_id,
      no_of_shares: share.no_of_shares,
      value_per_share: value.toFixed(6),
      total_value: (share.no_of_shares * value).toFixed(6),
      ticker,
    };

    shares.push(formatted_shares);
  }

  return shares;
}
