"use client";

import { AuctionsTable } from "@/components/auctions-table";
import { ContractClient } from "@/lib/contract/Contract.client";
import { AuctionStock, getAuctionedStock } from "@/lib/stocks";
import { useContract } from "@/providers/contract";
import { useWallet } from "@/providers/wallet";
import { useEffect, useState } from "react";
import { toast } from "sonner";

export function AuctionsView() {
  const [loading, setLoading] = useState(true);
  const [auctionedStocks, setAuctionedStocks] = useState<AuctionStock[]>([]);
  const { connect } = useWallet();
  const { contractClient } = useContract();

  async function loadData(contractClient: ContractClient) {
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

  useEffect(() => {
    if (!contractClient) {
      connect();
      return;
    }

    loadData(contractClient);
  }, [contractClient?.sender]);

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

  return <AuctionsTable auctionedStocks={auctionedStocks} />;
}
