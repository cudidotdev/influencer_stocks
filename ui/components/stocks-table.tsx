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

type Stock = {
  id: number;
  ticker: string;
  influencer: string;
  total_shares: number;
  in_auction: boolean;
  created_at: string;
};

export function StocksTable() {
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mock data - would be replaced with actual contract query
    const mockStocks: Stock[] = [
      {
        id: 101,
        ticker: "ALEX",
        influencer: "alex.near",
        total_shares: 1000,
        in_auction: false,
        created_at: "2023-05-15",
      },
      {
        id: 102,
        ticker: "EMMA",
        influencer: "emma.near",
        total_shares: 500,
        in_auction: true,
        created_at: "2023-06-20",
      },
      {
        id: 103,
        ticker: "JOHN",
        influencer: "john.near",
        total_shares: 2000,
        in_auction: false,
        created_at: "2023-04-10",
      },
    ];

    setTimeout(() => {
      setStocks(mockStocks);
      setLoading(false);
    }, 500);
  }, []);

  if (loading) {
    return <div className="flex justify-center p-4">Loading stocks...</div>;
  }

  return (
    <div>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>ID</TableHead>
            <TableHead>Ticker</TableHead>
            <TableHead>Influencer</TableHead>
            <TableHead className="text-right">Total Shares</TableHead>
            <TableHead>Status</TableHead>
            <TableHead>Created</TableHead>
            <TableHead className="text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {stocks.length === 0 ? (
            <TableRow>
              <TableCell colSpan={7} className="text-center">
                No stocks available
              </TableCell>
            </TableRow>
          ) : (
            stocks.map((stock) => (
              <TableRow key={stock.id}>
                <TableCell>{stock.id}</TableCell>
                <TableCell>{stock.ticker}</TableCell>
                <TableCell>{stock.influencer}</TableCell>
                <TableCell className="text-right">
                  {stock.total_shares}
                </TableCell>
                <TableCell>
                  {stock.in_auction ? (
                    <span className="inline-flex items-center rounded-full bg-amber-100 px-2.5 py-0.5 text-xs font-medium text-amber-800">
                      In Auction
                    </span>
                  ) : (
                    <span className="inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800">
                      Trading
                    </span>
                  )}
                </TableCell>
                <TableCell>{stock.created_at}</TableCell>
                <TableCell className="text-right">
                  <Button variant="outline" size="sm">
                    View
                  </Button>
                </TableCell>
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>
    </div>
  );
}
