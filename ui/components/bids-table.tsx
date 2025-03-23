"use client";

import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { useState, useEffect } from "react";
import Link from "next/link";
import { useContract } from "@/providers/contract";
import { ContractClient } from "@/lib/contract/Contract.client";
import { toast } from "sonner";
import moment from "moment";

type Bid = {
  id: number;
  stock_id: number;
  ticker: string;
  shares_requested: number;
  remaining_shares: number;
  price_per_share: string;
  total_price: string;
  status: "active" | "completed" | "cancelled";
  created_at: string;
};

export function BidsTable() {
  const [bids, setBids] = useState<Bid[]>([]);
  const [loading, setLoading] = useState(true);
  const { contractClient } = useContract();

  async function loadBids(contractClient: ContractClient) {
    try {
      setLoading(true);

      const bids_res = await contractClient.getBidsByBidder({
        bidder: contractClient.sender,
      });

      const bids = [];

      for (const bid of bids_res.bids) {
        const status: Bid["status"] = !bid.active
          ? "completed"
          : bid.open
            ? "active"
            : "cancelled";

        const total_price = (
          (bid.price_per_share / 1_000_000) *
          bid.remaining_shares
        ).toFixed(6);

        const price_per_share = (bid.price_per_share / 1_000_000).toFixed(6);

        const ticker = (
          await contractClient.getStockById({ stockId: bid.stock_id })
        ).stock.ticker;

        const created_at = moment.utc(bid.created_at).format("YYYY-MM-DD");

        bids.push({
          ...bid,
          status,
          price_per_share,
          total_price,
          ticker,
          created_at,
        });
      }

      setBids(bids);
    } catch (error: any) {
      toast.error("Error: " + error?.message);
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    if (contractClient) loadBids(contractClient);
  }, [contractClient?.sender]);

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
    <div>
      <p className="text-sm text-muted-foreground mb-1">
        Funds are refunded when a share is outbid
      </p>

      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>ID</TableHead>
            <TableHead>Stock</TableHead>
            <TableHead className="text-right">Shares Requested</TableHead>
            <TableHead className="text-right">Shares Remained</TableHead>
            <TableHead className="text-right">Price/Share</TableHead>
            <TableHead className="text-right">Total</TableHead>
            <TableHead>Status</TableHead>
            <TableHead>Created</TableHead>
            <TableHead className="text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {bids.length === 0 ? (
            <TableRow>
              <TableCell colSpan={8} className="text-center">
                You haven&apos;t placed any bids yet
              </TableCell>
            </TableRow>
          ) : (
            bids.map((bid) => (
              <TableRow key={bid.id}>
                <TableCell>{bid.id}</TableCell>
                <TableCell>{bid.ticker}</TableCell>
                <TableCell className="text-right">
                  {bid.shares_requested}
                </TableCell>
                <TableCell className="text-right">
                  {bid.remaining_shares}
                </TableCell>
                <TableCell className="text-right">
                  {bid.price_per_share}
                </TableCell>
                <TableCell className="text-right">{bid.total_price}</TableCell>
                <TableCell>
                  {bid.status === "active" && (
                    <span className="inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800">
                      Active
                    </span>
                  )}
                  {bid.status === "completed" && (
                    <span className="inline-flex items-center rounded-full bg-blue-100 px-2.5 py-0.5 text-xs font-medium text-blue-800">
                      Completed
                    </span>
                  )}
                  {bid.status === "cancelled" && (
                    <span className="inline-flex items-center rounded-full bg-amber-100 px-2.5 py-0.5 text-xs font-medium text-amber-800">
                      Outbid
                    </span>
                  )}
                </TableCell>
                <TableCell>{bid.created_at}</TableCell>
                <TableCell className="text-right">
                  {(bid.status === "active" || bid.status == "cancelled") && (
                    <Button variant="outline" size="sm" asChild>
                      <Link href={"/place-bid?stock_id=" + bid.stock_id}>
                        Place Another Bid
                      </Link>
                    </Button>
                  )}
                </TableCell>
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>
    </div>
  );
}
