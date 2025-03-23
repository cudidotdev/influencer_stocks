"use client";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

import { TrendingUp } from "lucide-react";
import { AuctionsTable } from "../auctions-table";
import { TopStocksTable } from "./top-stocks-table";
import { SharesTable } from "../shares-table";
import { useWallet } from "@/providers/wallet";
import { useEffect, useState } from "react";
import { useContract } from "@/providers/contract";
import { ContractClient } from "@/lib/contract/Contract.client";
import { toast } from "sonner";
import { getSharesbyOwner, Share } from "@/lib/shares";
import { AuctionStock, getAuctionedStock } from "@/lib/stocks";
import Link from "next/link";

export function MarketOverview() {
  const [loading, setLoading] = useState(true);
  const [shares, setShares] = useState<Share[]>([]);
  const [auctionedStocks, setAuctionedStocks] = useState<AuctionStock[]>([]);
  const { connect } = useWallet();
  const { contractClient } = useContract();

  async function loadData(contractClient: ContractClient) {
    try {
      setLoading(true);

      const shares = await getSharesbyOwner(contractClient);
      const auctionedStocks = await getAuctionedStock(contractClient);

      setShares(shares);
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

  return (
    <div className="space-y-4">
      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Your Balance</CardTitle>
            <TrendingUp className="h-4 w-4 text-muted-foreground" />
          </CardHeader>

          <CardContent>
            <div className="text-xl font-bold">
              {shares.reduce((acc, share) => (acc += +share.total_value), 0)}{" "}
              HUAHUA
            </div>

            <p className="text-xs text-muted-foreground">
              Total value of your shares
            </p>
          </CardContent>
        </Card>

        <Card>
          <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
            <CardTitle className="text-sm font-medium">Total Stocks</CardTitle>
            <TrendingUp className="h-4 w-4 text-muted-foreground" />
          </CardHeader>
          <CardContent>
            <div className="text-xl font-bold">{shares.length}</div>
            <p className="text-xs text-muted-foreground">
              Number of stocks you own part of
            </p>
          </CardContent>
        </Card>

        <Link href="/auctions">
          <Card>
            <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
              <CardTitle className="text-sm font-medium">
                Active Auctions
              </CardTitle>
              <TrendingUp className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              <div className="text-xl font-bold">{auctionedStocks.length}</div>
              <p className="text-xs text-muted-foreground">
                You can bid on these stocks
              </p>
            </CardContent>
          </Card>
        </Link>
      </div>

      <Card className="col-span-full">
        <CardHeader>
          <CardTitle>Your Shares</CardTitle>
          <CardDescription>
            <p className="mt-1">Stocks whose shares you own</p>
            <p>Prices are in HUAHUA</p>
          </CardDescription>
        </CardHeader>
        <CardContent>
          <SharesTable shares={shares} />
        </CardContent>
      </Card>

      <Card className="col-span-full">
        <CardHeader>
          <CardTitle>Available Auctions</CardTitle>
          <CardDescription>
            <p className="mt-1">Stocks available for bidding</p>
            <p>Prices are in HUAHUA</p>
          </CardDescription>
        </CardHeader>
        <CardContent>
          <AuctionsTable auctionedStocks={auctionedStocks} />
        </CardContent>
      </Card>

      <Card className="col-span-full">
        <CardHeader>
          <CardTitle>Top Active Stocks</CardTitle>
          <CardDescription>
            Overview of the most active stocks on the platform
          </CardDescription>
        </CardHeader>
        <CardContent>
          <TopStocksTable />
        </CardContent>
      </Card>
    </div>
  );
}
