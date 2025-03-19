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
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Card, CardContent } from "@/components/ui/card";
import {
  PieChart,
  Pie,
  Cell,
  ResponsiveContainer,
  Tooltip,
  Legend,
} from "recharts";

type Shareholder = {
  id: number;
  stock_id: number;
  ticker: string;
  owner: string;
  shares: number;
  percentage: number;
  acquisition_date: string;
};

type Stock = {
  id: number;
  ticker: string;
  total_shares: number;
  status: "in_auction" | "trading";
};

const COLORS = [
  "#0088FE",
  "#00C49F",
  "#FFBB28",
  "#FF8042",
  "#8884D8",
  "#82CA9D",
];

export function ShareholdersView() {
  const [shareholders, setShareholders] = useState<Shareholder[]>([]);
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [selectedStock, setSelectedStock] = useState<string>("");
  const [loading, setLoading] = useState(true);
  const [chartData, setChartData] = useState<any[]>([]);

  useEffect(() => {
    // Mock data - would be replaced with actual contract query
    const mockStocks: Stock[] = [
      { id: 101, ticker: "ALEX", total_shares: 1000, status: "trading" },
      { id: 103, ticker: "JOHN", total_shares: 2000, status: "trading" },
    ];

    const mockShareholders: Shareholder[] = [
      {
        id: 2001,
        stock_id: 101,
        ticker: "ALEX",
        owner: "user1.near",
        shares: 200,
        percentage: 20,
        acquisition_date: "2023-05-22",
      },
      {
        id: 2002,
        stock_id: 101,
        ticker: "ALEX",
        owner: "user2.near",
        shares: 150,
        percentage: 15,
        acquisition_date: "2023-05-22",
      },
      {
        id: 2003,
        stock_id: 101,
        ticker: "ALEX",
        owner: "user3.near",
        shares: 300,
        percentage: 30,
        acquisition_date: "2023-05-22",
      },
      {
        id: 2004,
        stock_id: 101,
        ticker: "ALEX",
        owner: "influencer.near",
        shares: 350,
        percentage: 35,
        acquisition_date: "2023-05-15",
      },
      {
        id: 2005,
        stock_id: 103,
        ticker: "JOHN",
        owner: "user4.near",
        shares: 500,
        percentage: 25,
        acquisition_date: "2023-04-17",
      },
      {
        id: 2006,
        stock_id: 103,
        ticker: "JOHN",
        owner: "user5.near",
        shares: 400,
        percentage: 20,
        acquisition_date: "2023-04-17",
      },
      {
        id: 2007,
        stock_id: 103,
        ticker: "JOHN",
        owner: "influencer.near",
        shares: 1100,
        percentage: 55,
        acquisition_date: "2023-04-01",
      },
    ];

    setTimeout(() => {
      setStocks(mockStocks);
      setShareholders(mockShareholders);
      setSelectedStock(mockStocks[0].id.toString());
      setLoading(false);
    }, 500);
  }, []);

  useEffect(() => {
    if (selectedStock && shareholders.length > 0) {
      const stockId = Number.parseInt(selectedStock);

      const filteredShareholders = shareholders.filter(
        (shareholder) => shareholder.stock_id === stockId,
      );

      const chartData = filteredShareholders.map((shareholder) => ({
        name: shareholder.owner,
        value: shareholder.shares,
      }));

      setChartData(chartData);
    }
  }, [selectedStock, shareholders]);

  const filteredShareholders = shareholders.filter(
    (shareholder) => shareholder.stock_id.toString() === selectedStock,
  );

  const selectedStockData = stocks.find(
    (stock) => stock.id.toString() === selectedStock,
  );

  if (loading) {
    return (
      <div className="flex justify-center p-4">
        Loading shareholders data...
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-xl font-semibold">Shareholders</h2>
          <p className="text-muted-foreground">
            View all shareholders for your stocks.
          </p>
        </div>
        <div className="w-64">
          <Select value={selectedStock} onValueChange={setSelectedStock}>
            <SelectTrigger>
              <SelectValue placeholder="Select Stock" />
            </SelectTrigger>
            <SelectContent>
              {stocks.map((stock) => (
                <SelectItem key={stock.id} value={stock.id.toString()}>
                  {stock.ticker}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      </div>

      <Card>
        <CardContent className="pt-6">
          {selectedStockData && (
            <div className="mb-6">
              <h3 className="text-lg font-medium mb-2">
                {selectedStockData.ticker} Overview
              </h3>
              <p className="text-muted-foreground">
                Total Shares: {selectedStockData.total_shares.toLocaleString()}
              </p>
            </div>
          )}

          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
            <div>
              <h3 className="text-lg font-medium mb-4">
                Ownership Distribution
              </h3>
              <div className="h-80">
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={chartData}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      outerRadius={80}
                      fill="#8884d8"
                      dataKey="value"
                      label={({ name, percent }) =>
                        `${name} (${(percent * 100).toFixed(0)}%)`
                      }
                    >
                      {chartData.map((entry, index) => (
                        <Cell
                          key={`cell-${index}`}
                          fill={COLORS[index % COLORS.length]}
                        />
                      ))}
                    </Pie>
                    <Tooltip
                      formatter={(value) => [`${value} shares`, "Shares Owned"]}
                    />
                    <Legend />
                  </PieChart>
                </ResponsiveContainer>
              </div>
            </div>

            <div>
              <h3 className="text-lg font-medium mb-4">Shareholders List</h3>
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Owner</TableHead>
                    <TableHead className="text-right">Shares</TableHead>
                    <TableHead className="text-right">Percentage</TableHead>
                    <TableHead>Acquisition Date</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {filteredShareholders.length === 0 ? (
                    <TableRow>
                      <TableCell colSpan={4} className="text-center">
                        No shareholders found
                      </TableCell>
                    </TableRow>
                  ) : (
                    filteredShareholders.map((shareholder) => (
                      <TableRow key={shareholder.id}>
                        <TableCell className="font-medium">
                          {shareholder.owner}
                        </TableCell>
                        <TableCell className="text-right">
                          {shareholder.shares.toLocaleString()}
                        </TableCell>
                        <TableCell className="text-right">
                          {shareholder.percentage}%
                        </TableCell>
                        <TableCell>{shareholder.acquisition_date}</TableCell>
                      </TableRow>
                    ))
                  )}
                </TableBody>
              </Table>
            </div>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
