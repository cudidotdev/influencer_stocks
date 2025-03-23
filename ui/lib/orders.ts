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
