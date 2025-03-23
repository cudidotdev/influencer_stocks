"use client";

import { ContractClient } from "@/lib/contract/Contract.client";
import { getStockById, getStocksInSale } from "@/lib/stocks";
import { useContract } from "@/providers/contract";
import { useWallet } from "@/providers/wallet";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import { StockSelector } from "./stock-selector";
import { useRouter } from "next/navigation";
import { Stock } from "@/lib/contract/Contract.types";
import { TradeForm } from "./trade-form";

export function TradeStock({ stockId }: { stockId: string | undefined }) {
  const [loading, setLoading] = useState(false);
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [stock, setStock] = useState<Stock>();
  const { connect } = useWallet();
  const { contractClient } = useContract();
  const router = useRouter();

  async function loadStocks(contractClient: ContractClient) {
    try {
      setLoading(true);

      const stocks = await getStocksInSale(contractClient);

      setStocks(stocks);
    } catch (error: any) {
      toast.error("Error: " + error?.message);
    } finally {
      setLoading(false);
    }
  }

  async function loadStock(contractClient: ContractClient, stockId: number) {
    try {
      setLoading(true);

      const stock = await getStockById(contractClient, stockId);

      setStock(stock);
    } catch (error: any) {
      toast.error("Error: " + error?.message);
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    if (!contractClient) {
      connect();
      return;
    }

    if (!stockId) loadStocks(contractClient);
    else {
      loadStocks(contractClient);
      loadStock(contractClient, +stockId);
    }
  }, [contractClient?.sender, stockId]);

  if (!contractClient?.sender) {
    return (
      <div className="flex justify-center p-4">
        Please conect wallet to continue
      </div>
    );
  }

  if (loading) {
    return <div className="flex justify-center p-4">Loading your data...</div>;
  }

  if (!stockId)
    return (
      <div className="mt-4">
        <StockSelector
          stocks={stocks.map((e) => ({ ...e, stock_id: e.id }))}
          onSelect={(stock) => router.push("/trade?stock_id=" + stock.stock_id)}
          selectedStock={null}
          title="Select a stock to place an order..."
        />
      </div>
    );

  if (stock)
    return (
      <div className="mt-4 mb-4" key={stock.id}>
        <StockSelector
          stocks={stocks.map((e) => ({ ...e, stock_id: e.id }))}
          onSelect={(stock) => router.push("/trade?stock_id=" + stock.stock_id)}
          selectedStock={null}
          title="Select a stock to place an order..."
        />

        <div className="flex justify-center gap-8 mt-6">
          <TradeForm stock={stock} />
        </div>
      </div>
    );

  return <></>;
}
