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
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { toast } from "sonner";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Card, CardContent } from "@/components/ui/card";
import { AlertCircle, Gavel, CheckCircle } from "lucide-react";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { getStocksByInfluencer, Stock } from "@/lib/stocks";
import { useWallet } from "@/providers/wallet";
import { useContract } from "@/providers/contract";
import { ContractClient } from "@/lib/contract/Contract.client";
import Link from "next/link";

export function AuctionManagement() {
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [loading, setLoading] = useState(true);
  const [selectedStock, setSelectedStock] = useState<Stock | null>(null);
  const [isStartingAuction, setIsStartingAuction] = useState(false);
  const [isEndingAuction, setIsEndingAuction] = useState(false);
  const [activeTab, setActiveTab] = useState("eligible");

  const { connect } = useWallet();
  const { signingClient, contractClient, msgComposer } = useContract();

  async function loadStocks(
    contractClient: ContractClient,
    influencer: string,
  ) {
    try {
      setLoading(true);

      const stocks = await getStocksByInfluencer(contractClient, influencer);

      setStocks(stocks);
    } catch (error: any) {
      toast.error("Error fetching stocks: " + error?.message);
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    if (!contractClient) {
      connect();
      return;
    }

    loadStocks(contractClient, contractClient.sender);
  }, [contractClient?.sender]);

  const eligibleForAuction = stocks.filter(
    (stock) => stock.status === "upcoming",
  );
  const inAuction = stocks.filter((stock) => stock.status === "in_auction");
  const completedAuctions = stocks.filter(
    (stock) => stock.status === "trading",
  );

  const handleStartAuction = async () => {
    if (!selectedStock) return;

    setIsStartingAuction(true);

    try {
      if (!msgComposer || !signingClient || !contractClient)
        return toast.error("Please connect wallet");

      const msg = msgComposer.startAuction({ stockId: selectedStock.id });

      await signingClient!.signAndBroadcast(
        contractClient.sender,
        [msg],
        "auto", // or specify gas
      );

      // Update the stock status in the UI
      setStocks(
        stocks.map((stock) =>
          stock.id === selectedStock.id
            ? {
                ...stock,
                status: "in_auction",
                auction_start: new Date().toISOString().split("T")[0],
                auction_end: new Date(Date.now() + 24 * 60 * 60 * 1000)
                  .toISOString()
                  .split("T")[0],
              }
            : stock,
        ),
      );

      toast.success("Auction Started", {
        description: `Auction for ${selectedStock.ticker} has been started successfully.`,
      });
    } catch (error) {
      toast.error("Error Starting Auction", {
        description:
          "There was an error starting the auction. Please try again.",
      });
    } finally {
      setIsStartingAuction(false);
      setSelectedStock(null);
    }
  };

  const handleEndAuction = async () => {
    if (!selectedStock) return;

    setIsEndingAuction(true);

    try {
      // This would be replaced with actual contract interaction
      await new Promise((resolve) => setTimeout(resolve, 1500));

      // Update the stock status in the UI
      setStocks(
        stocks.map((stock) =>
          stock.id === selectedStock.id
            ? {
                ...stock,
                status: "trading",
              }
            : stock,
        ),
      );

      toast.success("Auction Ended", {
        description: `Auction for ${selectedStock.ticker} has been ended successfully.`,
      });
    } catch (error) {
      toast.error("Error Ending Auction", {
        description: "There was an error ending the auction. Please try again.",
      });
    } finally {
      setIsEndingAuction(false);
      setSelectedStock(null);
    }
  };

  if (!contractClient?.sender) {
    return (
      <div className="flex justify-center p-4">
        Please conect wallet to continue
      </div>
    );
  }

  if (loading) {
    return (
      <div className="flex justify-center p-4">Loading your stocks...</div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold">Auction Management</h2>
        <p className="text-muted-foreground">
          Start and manage auctions for your stock offerings.
        </p>
      </div>

      <Tabs
        defaultValue="eligible"
        value={activeTab}
        onValueChange={setActiveTab}
      >
        <TabsList className="mb-4 !h-fit !p-1">
          <TabsTrigger value="eligible" className="px-6 py-1.5 cursor-pointer">
            Eligible for Auction
          </TabsTrigger>
          <TabsTrigger
            value="in-auction"
            className="px-6 py-1.5 cursor-pointer"
          >
            In Auction
          </TabsTrigger>
          <TabsTrigger value="completed" className="px-6 py-1.5 cursor-pointer">
            Completed Auctions
          </TabsTrigger>
        </TabsList>

        <TabsContent value="eligible">
          <Card>
            <CardContent className="pt-6">
              {eligibleForAuction.length === 0 ? (
                <div className="text-center py-6">
                  <p className="text-muted-foreground">
                    No stocks eligible for auction
                  </p>
                </div>
              ) : (
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead>Ticker</TableHead>
                      <TableHead className="text-right">Total Shares</TableHead>
                      <TableHead>Created</TableHead>
                      <TableHead className="text-right">Actions</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {eligibleForAuction.map((stock) => (
                      <TableRow key={stock.id}>
                        <TableCell className="font-medium">
                          {stock.ticker}
                        </TableCell>
                        <TableCell className="text-right">
                          {stock.total_shares.toLocaleString()}
                        </TableCell>
                        <TableCell>{stock.created_at}</TableCell>
                        <TableCell className="text-right">
                          <Dialog>
                            <DialogTrigger asChild>
                              <Button
                                onClick={() => setSelectedStock(stock)}
                                variant="outline"
                                className="cursor-pointer"
                              >
                                Start Auction
                              </Button>
                            </DialogTrigger>
                            <DialogContent>
                              <DialogHeader>
                                <DialogTitle>
                                  Start Auction for {selectedStock?.ticker}
                                </DialogTitle>
                              </DialogHeader>
                              <Alert className="bg-blue-50 text-blue-800 border-blue-200">
                                <AlertCircle className="h-4 w-4" />
                                <AlertTitle>Important</AlertTitle>
                                <AlertDescription>
                                  Once the auction starts, it will run for 24
                                  hours. After that, you&apos;ll need to
                                  manually end the auction.
                                </AlertDescription>
                              </Alert>
                              <DialogFooter>
                                <Button
                                  onClick={handleStartAuction}
                                  disabled={isStartingAuction}
                                  className="cursor-pointer"
                                >
                                  {isStartingAuction
                                    ? "Starting..."
                                    : "Start Auction"}
                                </Button>
                              </DialogFooter>
                            </DialogContent>
                          </Dialog>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              )}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="in-auction">
          <Card>
            <CardContent className="pt-6">
              {inAuction.length === 0 ? (
                <div className="text-center py-6">
                  <p className="text-muted-foreground">
                    No stocks currently in auction
                  </p>
                </div>
              ) : (
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead>Ticker</TableHead>
                      <TableHead>Status</TableHead>
                      <TableHead className="text-right">Total Shares</TableHead>
                      <TableHead>Auction Period</TableHead>
                      <TableHead className="text-right">
                        Lowest Bid Price
                      </TableHead>
                      <TableHead className="text-right">Total Bids</TableHead>
                      <TableHead className="text-right">Actions</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {inAuction.map((stock) => (
                      <TableRow key={stock.id}>
                        <TableCell className="font-medium">
                          {stock.ticker}
                        </TableCell>
                        <TableCell>
                          <div className="flex items-center gap-2">
                            <Gavel className="h-4 w-4 text-amber-600" />
                            <Badge
                              variant="outline"
                              className="bg-amber-100 text-amber-800"
                            >
                              In Auction
                            </Badge>
                          </div>
                        </TableCell>
                        <TableCell className="text-right">
                          {stock.total_shares.toLocaleString()}
                        </TableCell>
                        <TableCell>
                          {stock.auction_start && stock.auction_end
                            ? `${stock.auction_start} - ${stock.auction_end}`
                            : "N/A"}
                        </TableCell>
                        <TableCell className="text-right">
                          {stock.lowest_bid || "No bids yet"}
                        </TableCell>
                        <TableCell className="text-right">
                          {stock.total_bids || 0}
                        </TableCell>
                        <TableCell className="text-right">
                          <div className="flex justify-end gap-2">
                            <Button
                              variant="outline"
                              asChild
                              className="cursor-pointer"
                            >
                              <Link
                                href={`/influencer/bids?stock_id=${stock.id}`}
                              >
                                View Bids
                              </Link>
                            </Button>
                            <Dialog>
                              <DialogTrigger asChild>
                                <Button
                                  onClick={() => setSelectedStock(stock)}
                                  variant="default"
                                  className="cursor-pointer"
                                >
                                  End Auction
                                </Button>
                              </DialogTrigger>
                              <DialogContent>
                                <DialogHeader>
                                  <DialogTitle>
                                    End Auction for {selectedStock?.ticker}
                                  </DialogTitle>
                                  <DialogDescription>
                                    Are you sure you want to end this auction?
                                    This will finalize all bids and allocate
                                    shares to the winning bidders.
                                  </DialogDescription>
                                </DialogHeader>
                                <div className="py-4">
                                  <Alert className="bg-amber-50 text-amber-800 border-amber-200">
                                    <AlertCircle className="h-4 w-4" />
                                    <AlertTitle>Warning</AlertTitle>
                                    <AlertDescription>
                                      Ending the auction is irreversible. All
                                      winning bids will be processed and shares
                                      will be allocated.
                                    </AlertDescription>
                                  </Alert>
                                </div>
                                <DialogFooter>
                                  <Button
                                    onClick={handleEndAuction}
                                    disabled={isEndingAuction}
                                  >
                                    {isEndingAuction
                                      ? "Ending..."
                                      : "End Auction"}
                                  </Button>
                                </DialogFooter>
                              </DialogContent>
                            </Dialog>
                          </div>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              )}
            </CardContent>
          </Card>
        </TabsContent>

        <TabsContent value="completed">
          <Card>
            <CardContent className="pt-6">
              {completedAuctions.length === 0 ? (
                <div className="text-center py-6">
                  <p className="text-muted-foreground">No completed auctions</p>
                </div>
              ) : (
                <Table>
                  <TableHeader>
                    <TableRow>
                      <TableHead>Ticker</TableHead>
                      <TableHead>Status</TableHead>
                      <TableHead className="text-right">Total Shares</TableHead>
                      <TableHead>Auction Period</TableHead>
                      <TableHead className="text-right">Total Bids</TableHead>
                      <TableHead className="text-right">Actions</TableHead>
                    </TableRow>
                  </TableHeader>
                  <TableBody>
                    {completedAuctions.map((stock) => (
                      <TableRow key={stock.id}>
                        <TableCell className="font-medium">
                          {stock.ticker}
                        </TableCell>
                        <TableCell>
                          <div className="flex items-center gap-2">
                            <CheckCircle className="h-4 w-4 text-green-600" />
                            <Badge
                              variant="outline"
                              className="bg-green-100 text-green-800"
                            >
                              Completed
                            </Badge>
                          </div>
                        </TableCell>
                        <TableCell className="text-right">
                          {stock.total_shares.toLocaleString()}
                        </TableCell>
                        <TableCell>
                          {stock.auction_start && stock.auction_end
                            ? `${stock.auction_start} - ${stock.auction_end}`
                            : "N/A"}
                        </TableCell>
                        <TableCell className="text-right">
                          {stock.total_bids || 0}
                        </TableCell>
                        <TableCell className="text-right">
                          <div className="flex justify-end gap-2">
                            <Button variant="outline">View Results</Button>
                            <Button variant="outline">View Shareholders</Button>
                          </div>
                        </TableCell>
                      </TableRow>
                    ))}
                  </TableBody>
                </Table>
              )}
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
    </div>
  );
}
