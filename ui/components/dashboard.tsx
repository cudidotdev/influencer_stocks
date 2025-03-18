"use client";

import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { SharesTable } from "./shares-table";
import { StocksTable } from "./stocks-table";
import { BidsTable } from "./bids-table";
import { OrdersTable } from "./orders-table";
import { AuctionsTable } from "./auctions-table";

export function Dashboard() {
  const [activeTab, setActiveTab] = useState("shares");

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle>Your Dashboard</CardTitle>
        <CardDescription>
          Manage your shares, stocks, bids, orders, and auctions
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Tabs
          defaultValue="shares"
          value={activeTab}
          onValueChange={setActiveTab}
        >
          <TabsList className="grid grid-cols-5 mb-4">
            <TabsTrigger value="shares">Shares</TabsTrigger>
            <TabsTrigger value="stocks">Stocks</TabsTrigger>
            <TabsTrigger value="bids">Bids</TabsTrigger>
            <TabsTrigger value="orders">Orders</TabsTrigger>
            <TabsTrigger value="auctions">Auctions</TabsTrigger>
          </TabsList>
          <TabsContent value="shares">
            <SharesTable />
          </TabsContent>
          <TabsContent value="stocks">
            <StocksTable />
          </TabsContent>
          <TabsContent value="bids">
            <BidsTable />
          </TabsContent>
          <TabsContent value="orders">
            <OrdersTable />
          </TabsContent>
          <TabsContent value="auctions">
            <AuctionsTable />
          </TabsContent>
        </Tabs>
      </CardContent>
    </Card>
  );
}
