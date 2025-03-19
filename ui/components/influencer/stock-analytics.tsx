"use client";

import { useState, useEffect } from "react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import {
  LineChart,
  Line,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  Legend,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
} from "recharts";
import { TrendingUp, Users, DollarSign, BarChart3 } from "lucide-react";

type Stock = {
  id: number;
  ticker: string;
  total_shares: number;
};

type AnalyticsData = {
  priceHistory: {
    date: string;
    price: number;
  }[];
  shareholderDistribution: {
    name: string;
    value: number;
  }[];
  keyMetrics: {
    currentPrice: string;
    priceChange: string;
    priceChangePercentage: string;
    buyVolume: number;
    sellVolume: number;
    totalShareholders: number;
    marketCap: string;
  };
};

const COLORS = ["#0088FE", "#00C49F", "#FFBB28", "#FF8042", "#8884D8"];

export function StockAnalytics() {
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [selectedStock, setSelectedStock] = useState<string>("");
  const [analyticsData, setAnalyticsData] = useState<AnalyticsData | null>(
    null,
  );
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    // Mock data - would be replaced with actual contract query
    const mockStocks: Stock[] = [
      { id: 101, ticker: "ALEX", total_shares: 1000 },
      { id: 103, ticker: "JOHN", total_shares: 2000 },
    ];

    setTimeout(() => {
      setStocks(mockStocks);
      setSelectedStock(mockStocks[0].id.toString());
      setLoading(false);
    }, 500);
  }, []);

  useEffect(() => {
    if (selectedStock) {
      setLoading(true);

      // Mock analytics data - would be replaced with actual contract query
      const mockAnalyticsData: AnalyticsData = {
        priceHistory: [
          { date: "May 22", price: 5.0 },
          { date: "May 23", price: 5.2 },
          { date: "May 24", price: 5.1 },
          { date: "May 25", price: 5.3 },
          { date: "May 26", price: 5.5 },
          { date: "May 27", price: 5.4 },
          { date: "May 28", price: 5.6 },
          { date: "May 29", price: 5.8 },
          { date: "May 30", price: 5.7 },
          { date: "May 31", price: 5.9 },
          { date: "Jun 1", price: 6.0 },
          { date: "Jun 2", price: 6.2 },
          { date: "Jun 3", price: 6.1 },
          { date: "Jun 4", price: 6.3 },
        ],
        shareholderDistribution: [
          { name: "influencer.near", value: 350 },
          { name: "user1.near", value: 200 },
          { name: "Others", value: 150 },
          { name: "user3.near", value: 300 },
        ],
        keyMetrics: {
          currentPrice: "6.30",
          priceChange: "+0.20",
          priceChangePercentage: "+3.28%",
          sellVolume: 2580,
          buyVolume: 1000,
          totalShareholders: 4,
          marketCap: "6,300.00",
        },
      };

      setTimeout(() => {
        setAnalyticsData(mockAnalyticsData);
        setLoading(false);
      }, 500);
    }
  }, [selectedStock]);

  if (loading && !analyticsData) {
    return (
      <div className="flex justify-center p-4">Loading analytics data...</div>
    );
  }

  return (
    <div className="space-y-6">
      <div className="flex justify-between items-center">
        <div>
          <h2 className="text-xl font-semibold">Stock Analytics</h2>
          <p className="text-muted-foreground">
            View detailed analytics for your stocks.
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

      {analyticsData && (
        <>
          <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-5 gap-4">
            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Current Price
                </CardTitle>
                <TrendingUp className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">
                  ${analyticsData.keyMetrics.currentPrice}
                </div>
                <p
                  className={`text-xs ${analyticsData.keyMetrics.priceChange.startsWith("+") ? "text-green-600" : "text-red-600"}`}
                >
                  {analyticsData.keyMetrics.priceChange} (
                  {analyticsData.keyMetrics.priceChangePercentage})
                </p>
              </CardContent>
            </Card>
            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Market Cap
                </CardTitle>
                <DollarSign className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">
                  ${analyticsData.keyMetrics.marketCap}
                </div>
                <p className="text-xs text-muted-foreground">
                  Based on current price
                </p>
              </CardContent>
            </Card>
            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Buy Volume
                </CardTitle>
                <BarChart3 className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">
                  {analyticsData.keyMetrics.buyVolume}
                </div>
                <p className="text-xs text-muted-foreground">
                  Available Buy Shares
                </p>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Sell Volume
                </CardTitle>
                <BarChart3 className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">
                  {analyticsData.keyMetrics.sellVolume}
                </div>
                <p className="text-xs text-muted-foreground">
                  Available Sell Shares
                </p>
              </CardContent>
            </Card>

            <Card>
              <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
                <CardTitle className="text-sm font-medium">
                  Shareholders
                </CardTitle>
                <Users className="h-4 w-4 text-muted-foreground" />
              </CardHeader>
              <CardContent>
                <div className="text-2xl font-bold">
                  {analyticsData.keyMetrics.totalShareholders}
                </div>
                <p className="text-xs text-muted-foreground">Unique owners</p>
              </CardContent>
            </Card>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <Card>
              <CardHeader>
                <CardTitle>Price History</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="h-80">
                  <ResponsiveContainer width="100%" height="100%">
                    <LineChart
                      data={analyticsData.priceHistory}
                      margin={{
                        top: 5,
                        right: 30,
                        left: 20,
                        bottom: 5,
                      }}
                    >
                      <CartesianGrid strokeDasharray="3 3" />
                      <XAxis dataKey="date" />
                      <YAxis />
                      <Tooltip formatter={(value) => [`$${value}`, "Price"]} />
                      <Legend />
                      <Line
                        type="monotone"
                        dataKey="price"
                        stroke="#8884d8"
                        activeDot={{ r: 8 }}
                      />
                    </LineChart>
                  </ResponsiveContainer>
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>Shareholder Distribution</CardTitle>
              </CardHeader>
              <CardContent>
                <div className="h-80">
                  <ResponsiveContainer width="100%" height="100%">
                    <PieChart>
                      <Pie
                        data={analyticsData.shareholderDistribution}
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
                        {analyticsData.shareholderDistribution.map(
                          (entry, index) => (
                            <Cell
                              key={`cell-${index}`}
                              fill={COLORS[index % COLORS.length]}
                            />
                          ),
                        )}
                      </Pie>
                      <Tooltip
                        formatter={(value) => [
                          `${value} shares`,
                          "Shares Owned",
                        ]}
                      />
                      <Legend />
                    </PieChart>
                  </ResponsiveContainer>
                </div>
              </CardContent>
            </Card>
          </div>
        </>
      )}
    </div>
  );
}
