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
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { toast } from "sonner";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Card, CardContent } from "@/components/ui/card";
import { AlertCircle, Gavel, CheckCircle } from "lucide-react";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";

type Stock = {
  id: number;
  ticker: string;
  total_shares: number;
  status: "upcoming" | "in_auction" | "trading";
  auction_start?: string;
  auction_end?: string;
  created_at: string;
  highest_bid?: string;
  total_bids?: number;
};

export function AuctionManagement() {
  const [stocks, setStocks] = useState<Stock[]>([]);
  const [loading, setLoading] = useState(true);
  const [auctionDuration, setAuctionDuration] = useState(7);
  const [selectedStock, setSelectedStock] = useState<Stock | null>(null);
  const [isStartingAuction, setIsStartingAuction] = useState(false);
  const [isEndingAuction, setIsEndingAuction] = useState(false);
  const [activeTab, setActiveTab] = useState("eligible");

  useEffect(() => {
    // Mock data - would be replaced with actual contract query
    const mockStocks: Stock[] = [
      {
        id: 101,
        ticker: "ALEX",
        total_shares: 1000,
        status: "upcoming",
        created_at: "2023-05-15",
      },
      {
        id: 102,
        ticker: "EMMA",
        total_shares: 500,
        status: "in_auction",
        auction_start: "2023-06-20",
        auction_end: "2023-06-27",
        created_at: "2023-06-15",
        highest_bid: "10.50",
        total_bids: 15,
      },
      {
        id: 103,
        ticker: "JOHN",
        total_shares: 2000,
        status: "trading",
        auction_start: "2023-04-10",
        auction_end: "2023-04-17",
        created_at: "2023-04-01",
        highest_bid: "3.75",
        total_bids: 32,
      },
    ];

    setTimeout(() => {
      setStocks(mockStocks);
      setLoading(false);
    }, 500);
  }, []);

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
      // This would be replaced with actual contract interaction
      await new Promise((resolve) => setTimeout(resolve, 1500));

      // Update the stock status in the UI
      setStocks(
        stocks.map((stock) =>
          stock.id === selectedStock.id
            ? {
                ...stock,
                status: "in_auction",
                auction_start: new Date().toISOString().split("T")[0],
                auction_end: new Date(
                  Date.now() + auctionDuration * 24 * 60 * 60 * 1000,
                )
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

  if (loading) {
    return (
      <div className="flex justify-center p-4">Loading auction data...</div>
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
        <TabsList className="mb-4">
          <TabsTrigger value="eligible">Eligible for Auction</TabsTrigger>
          <TabsTrigger value="in-auction">In Auction</TabsTrigger>
          <TabsTrigger value="completed">Completed Auctions</TabsTrigger>
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
                              >
                                Start Auction
                              </Button>
                            </DialogTrigger>
                            <DialogContent>
                              <DialogHeader>
                                <DialogTitle>
                                  Start Auction for {selectedStock?.ticker}
                                </DialogTitle>
                                <DialogDescription>
                                  Set the duration for the auction. Once
                                  started, users will be able to place bids on
                                  your stock.
                                </DialogDescription>
                              </DialogHeader>
                              <div className="grid gap-4 py-4">
                                <div className="grid grid-cols-4 items-center gap-4">
                                  <Label
                                    htmlFor="duration"
                                    className="text-right"
                                  >
                                    Duration (days)
                                  </Label>
                                  <Input
                                    id="duration"
                                    type="number"
                                    value={auctionDuration}
                                    onChange={(e) =>
                                      setAuctionDuration(Number(e.target.value))
                                    }
                                    min={1}
                                    max={30}
                                    className="col-span-3"
                                  />
                                </div>
                              </div>
                              <Alert className="bg-blue-50 text-blue-800 border-blue-200">
                                <AlertCircle className="h-4 w-4" />
                                <AlertTitle>Important</AlertTitle>
                                <AlertDescription>
                                  Once the auction starts, it will run for{" "}
                                  {auctionDuration} days. After that,
                                  you&apos;ll need to manually end the auction.
                                </AlertDescription>
                              </Alert>
                              <DialogFooter>
                                <Button
                                  onClick={handleStartAuction}
                                  disabled={isStartingAuction}
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
                      <TableHead className="text-right">Highest Bid</TableHead>
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
                          {stock.highest_bid || "No bids yet"}
                        </TableCell>
                        <TableCell className="text-right">
                          {stock.total_bids || 0}
                        </TableCell>
                        <TableCell className="text-right">
                          <div className="flex justify-end gap-2">
                            <Button
                              variant="outline"
                              onClick={() => setActiveTab("bids")}
                            >
                              View Bids
                            </Button>
                            <Dialog>
                              <DialogTrigger asChild>
                                <Button
                                  onClick={() => setSelectedStock(stock)}
                                  variant="default"
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
                      <TableHead className="text-right">
                        Final Highest Bid
                      </TableHead>
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
                          {stock.highest_bid || "No bids"}
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
