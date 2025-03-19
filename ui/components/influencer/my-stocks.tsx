"use client";

import { useState, useEffect } from "react";
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

type Stock = {
  id: number;
  ticker: string;
  total_shares: number;
  status: "upcoming" | "in_auction" | "trading";
  auction_start?: string;
  auction_end?: string;
  created_at: string;
  highest_bid?: string;
  total_shareholders?: number;
};

export function MyStocks() {
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mock data - would be replaced with actual contract query
    const mockStocks: Stock[] = [
      {
        id: 101,
        ticker: "ALEX",
        total_shares: 1000,
        status: "trading",
        created_at: "2023-05-15",
        highest_bid: "5.20",
        total_shareholders: 12,
      },
      {
        id: 102,
        ticker: "EMMA",
        total_shares: 500,
        status: "in_auction",
        auction_start: "2023-06-20",
        auction_end: "2023-06-27",
        created_at: "2023-06-15",
        highest_bid: "10.50",
      },
      {
        id: 103,
        ticker: "JOHN",
        total_shares: 2000,
        status: "upcoming",
        created_at: "2023-07-01",
      },
    ];

    setTimeout(() => {
      setStocks(mockStocks);
      setLoading(false);
    }, 500);
  }, []);

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
            <TableHead className="text-right">Highest Bid</TableHead>
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
                  {stock.highest_bid || "N/A"}
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
                    <DropdownMenuContent align="end">
                      <DropdownMenuLabel>Actions</DropdownMenuLabel>
                      <DropdownMenuSeparator />
                      {stock.status === "upcoming" && (
                        <DropdownMenuItem>Start Auction</DropdownMenuItem>
                      )}
                      {stock.status === "in_auction" && (
                        <DropdownMenuItem>End Auction</DropdownMenuItem>
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
