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

type Share = {
  id: number;
  stock_id: number;
  ticker: string;
  no_of_shares: number;
  value_per_share: string;
  total_value: string;
};

export function SharesTable() {
  const [shares, setShares] = useState<Share[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mock data - would be replaced with actual contract query
    const mockShares: Share[] = [
      {
        id: 1,
        stock_id: 101,
        ticker: "ALEX",
        no_of_shares: 100,
        value_per_share: "5.20",
        total_value: "520.00",
      },
      {
        id: 2,
        stock_id: 102,
        ticker: "EMMA",
        no_of_shares: 50,
        value_per_share: "10.50",
        total_value: "525.00",
      },
      {
        id: 3,
        stock_id: 103,
        ticker: "JOHN",
        no_of_shares: 200,
        value_per_share: "3.75",
        total_value: "750.00",
      },
    ];

    setTimeout(() => {
      setShares(mockShares);
      setLoading(false);
    }, 500);
  }, []);

  if (loading) {
    return (
      <div className="flex justify-center p-4">Loading your shares...</div>
    );
  }

  return (
    <div>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Stock ID</TableHead>
            <TableHead>Ticker</TableHead>
            <TableHead className="text-right">Shares</TableHead>
            <TableHead className="text-right">Value/Share</TableHead>
            <TableHead className="text-right">Total Value</TableHead>
            <TableHead className="text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {shares.length === 0 ? (
            <TableRow>
              <TableCell colSpan={6} className="text-center">
                You don't own any shares yet
              </TableCell>
            </TableRow>
          ) : (
            shares.map((share) => (
              <TableRow key={share.id}>
                <TableCell>{share.stock_id}</TableCell>
                <TableCell>{share.ticker}</TableCell>
                <TableCell className="text-right">
                  {share.no_of_shares}
                </TableCell>
                <TableCell className="text-right">
                  {share.value_per_share}
                </TableCell>
                <TableCell className="text-right">
                  {share.total_value}
                </TableCell>
                <TableCell className="text-right">
                  <Button variant="outline" size="sm" className="mr-2">
                    Buy
                  </Button>
                  <Button variant="outline" size="sm">
                    Sell
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
