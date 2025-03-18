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

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const pathname = usePathname();

  // Extract the active tab from the pathname
  const activeTab = pathname.split("/")[1] || "shares";

  return (
    <div className="w-full mx-auto sm:max-w-[40rem] md:max-w-[48rem] lg:max-w-[64rem] xl:max-w-[80rem] px-4 py-2">
      <div className="flex justify-end mb-4">
        <p className="text-sm text-muted-foreground">
          Are you an influencer?{" "}
          <a href="#" className="text-primary hover:underline">
            Create your stock!
          </a>
        </p>
      </div>

      <Card className="w-full">
        <CardHeader>
          <CardTitle>Your Dashboard</CardTitle>
          <CardDescription>
            Manage your shares, stocks, bids, orders, and auctions
          </CardDescription>
        </CardHeader>
        <CardContent>
          <Tabs value={activeTab} className="w-full">
            <TabsList className="w-full grid grid-cols-5 mb-4 !h-fit !p-1">
              <TabsTrigger value="shares" asChild className="px-6 py-2">
                <Link href="/" className="w-full">
                  Shares
                </Link>
              </TabsTrigger>

              <TabsTrigger value="stocks" asChild className="px-6 py-2">
                <Link href="/stocks" className="w-full">
                  Stocks
                </Link>
              </TabsTrigger>

              <TabsTrigger value="bids" asChild className="px-6 py-2">
                <Link href="/bids" className="w-full">
                  Bids
                </Link>
              </TabsTrigger>

              <TabsTrigger value="orders" asChild className="px-6 py-2">
                <Link href="/orders" className="w-full">
                  Orders
                </Link>
              </TabsTrigger>

              <TabsTrigger value="auctions" asChild className="px-6 py-2">
                <Link href="/auctions" className="w-full">
                  Auctions
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
