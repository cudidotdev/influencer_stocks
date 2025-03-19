"use client";

import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { usePathname } from "next/navigation";
import Link from "next/link";

export default function Layout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  const pathname = usePathname();

  // Extract the active tab from the pathname
  const activeTab = pathname.split("/")[2] || "stocks";

  return (
    <div className="w-full max-w-[80rem] mx-auto px-4 sm:px-8 py-2 mt-4 space-y-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">
          Influencer Dashboard
        </h1>
        <p className="text-muted-foreground">
          Manage your stock offerings, auctions, and view your shareholders.
        </p>
      </div>

      <Card className="w-full">
        <CardHeader className="pb-3">
          <CardTitle className="!text-xl">Stock Management</CardTitle>
          <CardDescription>
            Create, manage, and monitor your stock offerings.
          </CardDescription>
        </CardHeader>

        <CardContent>
          <Tabs value={activeTab} className="w-full">
            <TabsList className="w-full grid grid-cols-6 mb-4 !h-fit !p-1">
              <TabsTrigger value="stocks" asChild className="px-6 py-2">
                <Link href="/influencer" className="w-full">
                  My Stocks
                </Link>
              </TabsTrigger>

              <TabsTrigger value="create-stock" asChild className="px-6 py-2">
                <Link href="/influencer/create-stock" className="w-full">
                  Create Stock
                </Link>
              </TabsTrigger>

              <TabsTrigger value="auctions" asChild className="px-6 py-2">
                <Link href="/influencer/auctions" className="w-full">
                  Auctions
                </Link>
              </TabsTrigger>

              <TabsTrigger value="bids" asChild className="px-6 py-2">
                <Link href="/influencer/bids" className="w-full">
                  Bids
                </Link>
              </TabsTrigger>

              <TabsTrigger value="shareholders" asChild className="px-6 py-2">
                <Link href="/influencer/shareholders" className="w-full">
                  Shareholders
                </Link>
              </TabsTrigger>

              <TabsTrigger value="analytics" asChild className="px-6 py-2">
                <Link href="/influencer/analytics" className="w-full">
                  Analytics
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
