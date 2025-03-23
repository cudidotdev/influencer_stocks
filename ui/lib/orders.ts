import { ContractClient } from "./contract/Contract.client";

export type Order = {
  id: number;
  stock_id: number;
  ticker: string;
  type: "buy" | "sell";
  shares: number;
  price_per_share: string;
  total_price: string;
  status: "open" | "filled" | "cancelled";
  created_at: string;
};

export async function getOrdersByOwner(
  contractClient: ContractClient,
  owner: string,
) {
  let buy_orders = await contractClient.getOpenBuyOrdersByOwner({
    sortBy: "created_at_desc",
    owner,
  });

  let sell_orders = await contractClient.getOpenSellOrdersByOwner({
    owner,
    sortBy: "created_at_desc",
  });
}
