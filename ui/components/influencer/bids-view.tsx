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
import { Badge } from "@/components/ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Card, CardContent } from "@/components/ui/card";
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Legend,
} from "recharts";

type Bid = {
  id: number;
  stock_id: number;
  ticker: string;
  bidder: string;
  shares_requested: number;
  price_per_share: string;
  total_price: string;
  created_at: string;
  status: "active" | "winning" | "outbid" | "expired";
};

type Stock = {
  id: number;
  ticker: string;
  status: "in_auction" | "trading";
};

export function BidsView() {
  const [bids, setBids] = useState<Bid[]>([]);
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [selectedStock, setSelectedStock] = useState<string>("all");
  const [loading, setLoading] = useState(true);
  const [chartData, setChartData] = useState<any[]>([]);

  useEffect(() => {
    // Mock data - would be replaced with actual contract query
    const mockStocks: Stock[] = [
      { id: 102, ticker: "EMMA", status: "in_auction" },
      { id: 103, ticker: "JOHN", status: "trading" },
    ];

    const mockBids: Bid[] = [
      {
        id: 1001,
        stock_id: 102,
        ticker: "EMMA",
        bidder: "user1.near",
        shares_requested: 50,
        price_per_share: "10.50",
        total_price: "525.00",
        created_at: "2023-06-21 14:30",
        status: "winning",
      },
      {
        id: 1002,
        stock_id: 102,
        ticker: "EMMA",
        bidder: "user2.near",
        shares_requested: 25,
        price_per_share: "10.25",
        total_price: "256.25",
        created_at: "2023-06-21 15:45",
        status: "outbid",
      },
      {
        id: 1003,
        stock_id: 102,
        ticker: "EMMA",
        bidder: "user3.near",
        shares_requested: 100,
        price_per_share: "10.00",
        total_price: "1000.00",
        created_at: "2023-06-22 09:15",
        status: "active",
      },
      {
        id: 1004,
        stock_id: 103,
        ticker: "JOHN",
        bidder: "user4.near",
        shares_requested: 200,
        price_per_share: "3.75",
        total_price: "750.00",
        created_at: "2023-04-12 10:30",
        status: "winning",
      },
      {
        id: 1005,
        stock_id: 103,
        ticker: "JOHN",
        bidder: "user5.near",
        shares_requested: 150,
        price_per_share: "3.50",
        total_price: "525.00",
        created_at: "2023-04-13 16:20",
        status: "winning",
      },
    ];

    setTimeout(() => {
      setStocks(mockStocks);
      setBids(mockBids);

      // Prepare chart data
      const bidPrices = mockBids
        .filter((bid) => bid.stock_id === 102) // Default to first stock
        .map((bid) => ({
          time: bid.created_at.split(" ")[1],
          price: Number.parseFloat(bid.price_per_share),
          shares: bid.shares_requested,
        }))
        .sort((a, b) => a.time.localeCompare(b.time));

      setChartData(bidPrices);
      setLoading(false);
    }, 500);
  }, []);

  useEffect(() => {
    if (bids.length > 0) {
      const stockId =
        selectedStock === "all"
          ? bids[0].stock_id
          : Number.parseInt(selectedStock);

      const bidPrices = bids
        .filter((bid) => selectedStock === "all" || bid.stock_id === stockId)
        .map((bid) => ({
          time: bid.created_at.split(" ")[1],
          price: Number.parseFloat(bid.price_per_share),
          shares: bid.shares_requested,
        }))
        .sort((a, b) => a.time.localeCompare(b.time));

      setChartData(bidPrices);
    }
  }, [selectedStock, bids]);

  const filteredBids = bids.filter(
    (bid) =>
      selectedStock === "all" || bid.stock_id.toString() === selectedStock,
  );

  const getStatusBadge = (status: string) => {
    switch (status) {
      case "winning":
        return (
          <Badge variant="outline" className="bg-green-100 text-green-800">
            Winning
          </Badge>
        );
      case "active":
        return (
          <Badge variant="outline" className="bg-blue-100 text-blue-800">
            Active
          </Badge>
        );
      case "outbid":
        return (
          <Badge variant="outline" className="bg-amber-100 text-amber-800">
            Outbid
          </Badge>
        );
      case "expired":
        return (
          <Badge variant="outline" className="bg-gray-100 text-gray-800">
            Expired
          </Badge>
        );
      default:
        return null;
    }
  };

  if (loading) {
    return <div className="flex justify-center p-4">Loading bids data...</div>;
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-xl font-semibold">Bids Overview</h2>
          <p className="text-muted-foreground">
            View all bids placed on your stocks.
          </p>
        </div>
        <div className="w-64">
          <Select value={selectedStock} onValueChange={setSelectedStock}>
            <SelectTrigger>
              <SelectValue placeholder="Select Stock" />
            </SelectTrigger>
            <SelectContent>
              <SelectItem value="all">All Stocks</SelectItem>
              {stocks.map((stock) => (
                <SelectItem key={stock.id} value={stock.id.toString()}>
                  {stock.ticker}{" "}
                  {stock.status === "in_auction" ? "(In Auction)" : ""}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>

      <Card>
        <CardContent className="pt-6">
          <div className="h-80 mb-6">
            <h3 className="text-lg font-medium mb-4">Bid Price History</h3>
            <ResponsiveContainer width="100%" height="100%">
              <BarChart
                data={chartData}
                margin={{
                  top: 5,
                  right: 30,
                  left: 20,
                  bottom: 5,
                }}
              >
                <CartesianGrid strokeDasharray="3 3" />
                <XAxis dataKey="time" />
                <YAxis yAxisId="left" orientation="left" stroke="#8884d8" />
                <YAxis yAxisId="right" orientation="right" stroke="#82ca9d" />
                <Tooltip />
                <Legend />
                <Bar
                  yAxisId="left"
                  dataKey="price"
                  name="Price per Share"
                  fill="#8884d8"
                />
                <Bar
                  yAxisId="right"
                  dataKey="shares"
                  name="Shares Requested"
                  fill="#82ca9d"
                />
              </BarChart>
            </ResponsiveContainer>
          </div>

          <h3 className="text-lg font-medium mb-4">All Bids</h3>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Stock</TableHead>
                <TableHead>Bidder</TableHead>
                <TableHead className="text-right">Shares</TableHead>
                <TableHead className="text-right">Price/Share</TableHead>
                <TableHead className="text-right">Total</TableHead>
                <TableHead>Time</TableHead>
                <TableHead>Status</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {filteredBids.length === 0 ? (
                <TableRow>
                  <TableCell colSpan={7} className="text-center">
                    No bids found
                  </TableCell>
                </TableRow>
              ) : (
                filteredBids.map((bid) => (
                  <TableRow key={bid.id}>
                    <TableCell className="font-medium">{bid.ticker}</TableCell>
                    <TableCell>{bid.bidder}</TableCell>
                    <TableCell className="text-right">
                      {bid.shares_requested}
                    </TableCell>
                    <TableCell className="text-right">
                      {bid.price_per_share}
                    </TableCell>
                    <TableCell className="text-right">
                      {bid.total_price}
                    </TableCell>
                    <TableCell>{bid.created_at}</TableCell>
                    <TableCell>{getStatusBadge(bid.status)}</TableCell>
                  </TableRow>
                ))
              )}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
    </div>
  );
}
