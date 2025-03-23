"use client";

import { ContractClient } from "@/lib/contract/Contract.client";
import { AuctionStock, getAuctionedStock } from "@/lib/stocks";
import { useContract } from "@/providers/contract";
import { useWallet } from "@/providers/wallet";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import { StockSelector } from "./stock-selector";
import { useRouter, useSearchParams } from "next/navigation";
import { getOpenBidsByStock } from "@/lib/bids";
import { Bid, Stock } from "@/lib/contract/Contract.types";
import { BidDistribution } from "./bid-distribution";
import { CountdownTimer } from "./countdown-timer";
import moment from "moment";
import { BidForm } from "./bid-form";

export function PlaceBid() {
  const [loading, setLoading] = useState(false);
  const [auctionedStocks, setAuctionedStocks] = useState<AuctionStock[]>([]);
  const [stock, setStock] = useState<Stock>();
  const [openBids, setOpenBids] = useState<Bid[]>([]);
  const [auctionEnd, setAuctionEnd] = useState<Date>();
  const { connect } = useWallet();
  const { contractClient } = useContract();
  const router = useRouter();
  const [stockId, setStockId] = useState<number>();
  const searchParams = useSearchParams();

  useEffect(() => {
    const stockId = searchParams.get("stock_id");

    if (stockId) setStockId(+stockId);
    else setStockId(undefined);
  }, [searchParams]);

  async function loadStocks(contractClient: ContractClient) {
    try {
      setLoading(true);

      const auctionedStocks = await getAuctionedStock(contractClient);

      setAuctionedStocks(auctionedStocks);
    } catch (error: any) {
      toast.error("Error: " + error?.message);
    } finally {
      setLoading(false);
    }
  }

  async function loadBids(contractClient: ContractClient, stockId: number) {
    try {
      setLoading(true);

      const bids = await getOpenBidsByStock(contractClient, stockId);

      const stock = (await contractClient.getStockById({ stockId })).stock;

      if (!stock.auction_start || !stock.auction_end)
        throw {
          message: "Bid not in auction",
        };

      setStock(stock);

      setAuctionEnd(moment.utc(stock.auction_end).toDate());

      setOpenBids(bids);
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
      loadBids(contractClient, +stockId);
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
          stocks={auctionedStocks}
          onSelect={(stock) =>
            router.push("/place-bid?stock_id=" + stock.stock_id)
          }
          selectedStock={null}
          title="Select a stock to place a bid..."
        />
      </div>
    );

  if (stock)
    return (
      <div className="mt-4 mb-16" key={stock.id}>
        <StockSelector
          stocks={auctionedStocks}
          onSelect={(stock) =>
            router.push("/place-bid?stock_id=" + stock.stock_id)
          }
          selectedStock={null}
          title="Select a stock to place a bid..."
        />

        <div className="mt-6">
          <CountdownTimer endTime={auctionEnd!} />
        </div>

        <div className="grid md:grid-cols-2 gap-8 mt-6">
          {stock.marked_as_active_auction && <BidForm stockId={+stockId} />}

          <BidDistribution bids={openBids} />
        </div>
      </div>
    );

  return <></>;
}
