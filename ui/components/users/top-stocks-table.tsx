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
import { TrendingUp, TrendingDown, MoreHorizontal } from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useEffect, useState } from "react";

type Stock = {
  id: number;
  ticker: string;
  price: number;
  change: number;
  changePercent: number;
  marketCap: number;
};

export function TopStocksTable() {
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mock data
    const mockStocks = [
      {
        id: 1,
        ticker: "ALEX",
        price: 6.3,
        change: 0.2,
        changePercent: 3.28,
        marketCap: 6300.0,
      },
      {
        id: 2,
        ticker: "EMMA",
        price: 10.5,
        change: -0.35,
        changePercent: -3.23,
        marketCap: 5250.0,
      },
      {
        id: 3,
        ticker: "JOHN",
        price: 8.75,
        change: 0.45,
        changePercent: 5.42,
        marketCap: 17500.0,
      },
      {
        id: 4,
        ticker: "SARA",
        price: 12.2,
        change: 1.2,
        changePercent: 10.91,
        marketCap: 12200.0,
      },
      {
        id: 5,
        ticker: "MIKE",
        price: 4.5,
        change: -0.25,
        changePercent: -5.26,
        marketCap: 4500.0,
      },
    ];

    setTimeout(() => {
      setStocks(mockStocks);
      setLoading(false);
    }, 500);
  }, []);

  if (loading) {
    return <div className="flex justify-center p-4">Loading auctions...</div>;
  }

  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Ticker</TableHead>
          <TableHead>Price</TableHead>
          <TableHead>24h Change</TableHead>
          <TableHead className="hidden md:table-cell">Market Cap</TableHead>
          <TableHead className="text-right">Actions</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {stocks.map((stock) => (
          <TableRow key={stock.id}>
            <TableCell className="font-medium">{stock.ticker}</TableCell>
            <TableCell>${stock.price.toFixed(2)}</TableCell>
            <TableCell>
              <div className="flex items-center">
                {stock.change >= 0 ? (
                  <TrendingUp className="mr-1 h-4 w-4 text-green-500" />
                ) : (
                  <TrendingDown className="mr-1 h-4 w-4 text-red-500" />
                )}
                <span
                  className={
                    stock.change >= 0 ? "text-green-500" : "text-red-500"
                  }
                >
                  {stock.change >= 0 ? "+" : ""}
                  {stock.change.toFixed(2)} ({stock.change >= 0 ? "+" : ""}
                  {stock.changePercent.toFixed(2)}%)
                </span>
              </div>
            </TableCell>
            <TableCell className="hidden md:table-cell">
              ${stock.marketCap.toFixed(2)}
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
                  <DropdownMenuItem>View Details</DropdownMenuItem>
                  <DropdownMenuItem>Quick Buy</DropdownMenuItem>
                  <DropdownMenuItem>Place Buy Order</DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            </TableCell>
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}
