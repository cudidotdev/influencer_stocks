"use client";

import type React from "react";

import Link from "next/link";
import { usePathname } from "next/navigation";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";

export default function UserDashboardLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const pathname = usePathname();

  // Extract the active tab from the pathname
  const activeTab = pathname.split("/")[1] || "overview";

  return (
    <div className="w-full mx-auto max-w-[80rem] px-4 sm:px-8 py-2">
      <div className="flex justify-end mb-4">
        <p className="text-sm text-muted-foreground">
          Are you an influencer?{" "}
          <Link href="/influencer" className="text-primary hover:underline">
            Create your stock!
          </Link>
        </p>
      </div>

      <Card className="w-full">
        <CardHeader>
          <CardTitle className="!text-xl">Your Dashboard</CardTitle>
          <CardDescription>
            Manage your shares, stocks, bids, orders, and auctions
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Tabs value={activeTab} className="w-full">
            <TabsList className="w-full grid grid-cols-3 md:grid-cols-6 mb-4 !h-fit !p-1">
              <TabsTrigger value="overview" asChild className="px-6 py-2">
                <Link href="/" className="w-full">
                  Overview
                </Link>
              </TabsTrigger>

              <TabsTrigger value="trade" asChild className="px-6 py-2">
                <Link href="/trade" className="w-full">
                  Trade
                </Link>
              </TabsTrigger>

              <TabsTrigger value="place-bid" asChild className="px-6 py-2">
                <Link href="/place-bid" className="w-full">
                  Bid
                </Link>
              </TabsTrigger>

              <TabsTrigger value="orders" asChild className="px-6 py-2">
                <Link href="/orders" className="w-full">
                  My Orders
                </Link>
              </TabsTrigger>

              <TabsTrigger value="auctions" asChild className="px-6 py-2">
                <Link href="/auctions" className="w-full">
                  Auctions
                </Link>
              </TabsTrigger>

              <TabsTrigger value="my-bids" asChild className="px-6 py-2">
                <Link href="/my-bids" className="w-full">
                  My Bids
                </Link>
              </TabsTrigger>
            </TabsList>
            {children}
          </Tabs>
        </CardContent>
      </Card>
    </div>
  );
}
