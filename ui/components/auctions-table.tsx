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

type Auction = {
  id: number;
  stock_id: number;
  ticker: string;
  total_shares: number;
  highest_bid: string;
  end_time: string;
  status: "active" | "upcoming";
};

export function AuctionsTable() {
  const [auctions, setAuctions] = useState<Auction[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mock data - would be replaced with actual contract query
    const mockAuctions: Auction[] = [
      {
        id: 401,
        stock_id: 102,
        ticker: "EMMA",
        total_shares: 500,
        highest_bid: "10.50",
        end_time: "2023-07-25 14:00",
        status: "active",
      },
      {
        id: 402,
        stock_id: 104,
        ticker: "MIKE",
        total_shares: 1000,
        highest_bid: "7.25",
        end_time: "2023-07-20 16:00",
        status: "upcoming",
      },
      {
        id: 403,
        stock_id: 105,
        ticker: "SARA",
        total_shares: 750,
        highest_bid: "8.75",
        end_time: "2023-07-15 12:00",
        status: "active",
      },
    ];

    setTimeout(() => {
      setAuctions(mockAuctions);
      setLoading(false);
    }, 500);
  }, []);

  if (loading) {
    return <div className="flex justify-center p-4">Loading auctions...</div>;
  }

  return (
    <div>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>ID</TableHead>
            <TableHead>Stock</TableHead>
            <TableHead className="text-right">Total Shares</TableHead>
            <TableHead className="text-right">Highest Bid</TableHead>
            <TableHead>End Time</TableHead>
            <TableHead>Status</TableHead>
            <TableHead className="text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {auctions.length === 0 ? (
            <TableRow>
              <TableCell colSpan={8} className="text-center">
                No active auctions
              </TableCell>
            </TableRow>
          ) : (
            auctions.map((auction) => (
              <TableRow key={auction.id}>
                <TableCell>{auction.id}</TableCell>
                <TableCell>{auction.ticker}</TableCell>
                <TableCell className="text-right">
                  {auction.total_shares}
                </TableCell>
                <TableCell className="text-right">
                  {auction.highest_bid}
                </TableCell>
                <TableCell>{auction.end_time}</TableCell>
                <TableCell>
                  {auction.status === "active" && (
                    <span className="inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800">
                      Active
                    </span>
                  )}
                  {auction.status === "upcoming" && (
                    <span className="inline-flex items-center rounded-full bg-blue-100 px-2.5 py-0.5 text-xs font-medium text-blue-800">
                      Upcoming
                    </span>
                  )}
                </TableCell>
                <TableCell className="text-right">
                  {auction.status === "active" && (
                    <Button variant="outline" size="sm">
                      Place Bid
                    </Button>
                  )}
                  {auction.status === "upcoming" && (
                    <Button variant="outline" size="sm" disabled>
                      Coming Soon
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
