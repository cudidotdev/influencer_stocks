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
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useState, useEffect } from "react";

type Order = {
  id: number;
  stock_id: number;
  ticker: string;
  type: "buy" | "sell";
  shares: number;
  price_per_share: string;
  total_price: string;
  status: "open" | "filled" | "cancelled";
  created_at: string;
};

export function OrdersTable() {
  const [orders, setOrders] = useState<Order[]>([]);
  const [loading, setLoading] = useState(true);
  const [orderType, setOrderType] = useState<"all" | "buy" | "sell">("all");

  useEffect(() => {
    // Mock data - would be replaced with actual contract query
    const mockOrders: Order[] = [
      {
        id: 301,
        stock_id: 101,
        ticker: "ALEX",
        type: "buy",
        shares: 30,
        price_per_share: "5.10",
        total_price: "153.00",
        status: "open",
        created_at: "2023-07-15",
      },
      {
        id: 302,
        stock_id: 103,
        ticker: "JOHN",
        type: "sell",
        shares: 50,
        price_per_share: "3.80",
        total_price: "190.00",
        status: "open",
        created_at: "2023-07-14",
      },
      {
        id: 303,
        stock_id: 102,
        ticker: "EMMA",
        type: "buy",
        shares: 20,
        price_per_share: "10.25",
        total_price: "205.00",
        status: "filled",
        created_at: "2023-07-10",
      },
    ];

    setTimeout(() => {
      setOrders(mockOrders);
      setLoading(false);
    }, 500);
  }, []);

  const filteredOrders = orders.filter((order) => {
    if (orderType === "all") return true;
    return order.type === orderType;
  });

  if (loading) {
    return (
      <div className="flex justify-center p-4">Loading your orders...</div>
    );
  }

  return (
    <div>
      <Tabs
        defaultValue="all"
        value={orderType}
        onValueChange={(value) => setOrderType(value as "all" | "buy" | "sell")}
      >
        <TabsList className="mb-4">
          <TabsTrigger value="all">All Orders</TabsTrigger>
          <TabsTrigger value="buy">Buy Orders</TabsTrigger>
          <TabsTrigger value="sell">Sell Orders</TabsTrigger>
        </TabsList>
      </Tabs>

      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>ID</TableHead>
            <TableHead>Stock</TableHead>
            <TableHead>Type</TableHead>
            <TableHead className="text-right">Shares</TableHead>
            <TableHead className="text-right">Price/Share</TableHead>
            <TableHead className="text-right">Total</TableHead>
            <TableHead>Status</TableHead>
            <TableHead>Created</TableHead>
            <TableHead className="text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {filteredOrders.length === 0 ? (
            <TableRow>
              <TableCell colSpan={9} className="text-center">
                No orders found
              </TableCell>
            </TableRow>
          ) : (
            filteredOrders.map((order) => (
              <TableRow key={order.id}>
                <TableCell>{order.id}</TableCell>
                <TableCell>{order.ticker}</TableCell>
                <TableCell>
                  {order.type === "buy" ? (
                    <span className="inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800">
                      Buy
                    </span>
                  ) : (
                    <span className="inline-flex items-center rounded-full bg-red-100 px-2.5 py-0.5 text-xs font-medium text-red-800">
                      Sell
                    </span>
                  )}
                </TableCell>
                <TableCell className="text-right">{order.shares}</TableCell>
                <TableCell className="text-right">
                  {order.price_per_share}
                </TableCell>
                <TableCell className="text-right">
                  {order.total_price}
                </TableCell>
                <TableCell>
                  {order.status === "open" && (
                    <span className="inline-flex items-center rounded-full bg-blue-100 px-2.5 py-0.5 text-xs font-medium text-blue-800">
                      Open
                    </span>
                  )}
                  {order.status === "filled" && (
                    <span className="inline-flex items-center rounded-full bg-green-100 px-2.5 py-0.5 text-xs font-medium text-green-800">
                      Filled
                    </span>
                  )}
                  {order.status === "cancelled" && (
                    <span className="inline-flex items-center rounded-full bg-gray-100 px-2.5 py-0.5 text-xs font-medium text-gray-800">
                      Cancelled
                    </span>
                  )}
                </TableCell>
                <TableCell>{order.created_at}</TableCell>
                <TableCell className="text-right">
                  {order.status === "open" && (
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
