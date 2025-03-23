"use client";

import { useState, useEffect, useCallback } from "react";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { MoreHorizontal, TrendingUp, Clock, Gavel } from "lucide-react";
import Link from "next/link";
import { useContract } from "@/providers/contract";
import { useWallet } from "@/providers/wallet";
import { ContractClient } from "@/lib/contract/Contract.client";
import { getStocksByInfluencer, Stock } from "@/lib/stocks";
import { toast } from "sonner";
import { useRouter } from "next/navigation";

export function MyStocks() {
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [loading, setLoading] = useState(true);
  const { connect } = useWallet();
  const { signingClient, contractClient, msgComposer } = useContract();
  const router = useRouter();

  async function loadStocks(
    contractClient: ContractClient,
    influencer: string,
  ) {
    try {
      setLoading(true);

      const stocks = await getStocksByInfluencer(contractClient, influencer);

      setStocks(stocks);
    } catch (error: any) {
      toast.error("Error fetching stocks: " + error?.message);
    } finally {
      setLoading(false);
    }
  }

  const startAuction = useCallback(
    async (stockId: number) => {
      try {
        if (!msgComposer || !signingClient || !contractClient)
          return toast.error("Please connect wallet");

        const msg = msgComposer.startAuction({ stockId });

        await signingClient!.signAndBroadcast(
          contractClient.sender,
          [msg],
          "auto", // or specify gas
        );

        router.refresh();
      } catch (error: any) {
        toast.error(error?.message);
      }
    },
    [signingClient, msgComposer],
  );

  const endAuction = useCallback(
    async (stockId: number) => {
      try {
        if (!msgComposer || !signingClient || !contractClient)
          return toast.error("Please connect wallet");

        const msg = msgComposer.endAuction({ stockId });

        await signingClient!.signAndBroadcast(
          contractClient.sender,
          [msg],
          "auto", // or specify gas
        );

        router.refresh();
      } catch (error: any) {
        toast.error(error?.message);
      }
    },
    [signingClient, msgComposer],
  );

  useEffect(() => {
    if (!contractClient) {
      connect();
      return;
    }

    loadStocks(contractClient, contractClient.sender);
  }, [contractClient?.sender]);

  const getStatusBadge = (status: string) => {
    switch (status) {
      case "upcoming":
        return (
          <Badge variant="outline" className="bg-blue-100 text-blue-800">
            Upcoming
          </Badge>
        );
      case "in_auction":
        return (
          <Badge variant="outline" className="bg-amber-100 text-amber-800">
            In Auction
          </Badge>
        );
      case "trading":
        return (
          <Badge variant="outline" className="bg-green-100 text-green-800">
            Trading
          </Badge>
        );
      default:
        return null;
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case "upcoming":
        return <Clock className="h-4 w-4 text-blue-600" />;
      case "in_auction":
        return <Gavel className="h-4 w-4 text-amber-600" />;
      case "trading":
        return <TrendingUp className="h-4 w-4 text-green-600" />;
      default:
        return null;
    }
  };

  if (!contractClient?.sender) {
    return (
      <div className="flex justify-center p-4">
        Please conect wallet to continue
      </div>
    );
  }

  if (loading) {
    return (
      <div className="flex justify-center p-4">Loading your stocks...</div>
    );
  }

  return (
    <div className="space-y-4">
      <div className="flex justify-between items-center">
        <h2 className="text-xl font-semibold">Your Stock Offerings</h2>

        <Button asChild>
          <Link href="/influencer/create-stock">Create New Stock</Link>
        </Button>
      </div>

      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Ticker</TableHead>
            <TableHead>Status</TableHead>
            <TableHead className="text-right">Total Shares</TableHead>
            <TableHead>Created</TableHead>
            <TableHead>Auction Period</TableHead>
            <TableHead className="text-right">Lowest Buy/Bid Price</TableHead>
            <TableHead className="text-right">Shareholders</TableHead>
            <TableHead className="text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {stocks.length === 0 ? (
            <TableRow>
              <TableCell colSpan={8} className="text-center">
                You haven&apos;t created any stocks yet
              </TableCell>
            </TableRow>
          ) : (
            stocks.map((stock) => (
              <TableRow key={stock.id}>
                <TableCell className="font-medium">{stock.ticker}</TableCell>
                <TableCell>
                  <div className="flex items-center gap-2">
                    {getStatusIcon(stock.status)}
                    {getStatusBadge(stock.status)}
                  </div>
                </TableCell>
                <TableCell className="text-right">
                  {stock.total_shares.toLocaleString()}
                </TableCell>
                <TableCell>{stock.created_at}</TableCell>
                <TableCell>
                  {stock.auction_start && stock.auction_end
                    ? `${stock.auction_start} - ${stock.auction_end}`
                    : "N/A"}
                </TableCell>
                <TableCell className="text-right">
                  {stock.lowest_price || stock.lowest_bid || "N/A"}
                </TableCell>
                <TableCell className="text-right">
                  {stock.total_shareholders || "0"}
                </TableCell>
                <TableCell className="text-right">
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="ghost" className="h-8 w-8 p-0">
                        <span className="sr-only">Open menu</span>
                        <MoreHorizontal className="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent
                      align="end"
                      className="[&_button]:cursor-pointer"
                    >
                      <DropdownMenuLabel>Actions</DropdownMenuLabel>
                      <DropdownMenuSeparator />
                      {stock.status === "upcoming" && (
                        <DropdownMenuItem>
                          <button onClick={() => startAuction(stock.id)}>
                            Start Auction
                          </button>
                        </DropdownMenuItem>
                      )}
                      {stock.status === "in_auction" && (
                        <DropdownMenuItem>
                          <button onClick={() => endAuction(stock.id)}>
                            End Auction
                          </button>
                        </DropdownMenuItem>
                      )}
                      <DropdownMenuItem>View Details</DropdownMenuItem>
                      <DropdownMenuItem>View Bids</DropdownMenuItem>
                      <DropdownMenuItem>View Shareholders</DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                </TableCell>
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>
    </div>
  );
}
