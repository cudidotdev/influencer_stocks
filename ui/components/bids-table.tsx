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

type Bid = {
  id: number;
  stock_id: number;
  ticker: string;
  shares_requested: number;
  price_per_share: string;
  total_price: string;
  status: "active" | "completed" | "cancelled";
  created_at: string;
};

export function BidsTable() {
  const [bids, setBids] = useState<Bid[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mock data - would be replaced with actual contract query
    const mockBids: Bid[] = [
      {
        id: 201,
        stock_id: 101,
        ticker: "ALEX",
        shares_requested: 50,
        price_per_share: "5.00",
        total_price: "250.00",
        status: "active",
        created_at: "2023-07-10",
      },
      {
        id: 202,
        stock_id: 102,
        ticker: "EMMA",
        shares_requested: 25,
        price_per_share: "10.00",
        total_price: "250.00",
        status: "completed",
        created_at: "2023-07-05",
      },
    ];

    setTimeout(() => {
      setBids(mockBids);
      setLoading(false);
    }, 500);
  }, []);

  if (loading) {
    return <div className="flex justify-center p-4">Loading your bids...</div>;
  }

  return (
    <div>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>ID</TableHead>
            <TableHead>Stock</TableHead>
            <TableHead className="text-right">Shares</TableHead>
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
                    <span className="inline-flex items-center rounded-full bg-gray-100 px-2.5 py-0.5 text-xs font-medium text-gray-800">
                      Cancelled
                    </span>
                  )}
                </TableCell>
                <TableCell>{bid.created_at}</TableCell>
                <TableCell className="text-right">
                  {bid.status === "active" && (
                    <Button variant="outline" size="sm">
                      Cancel
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
