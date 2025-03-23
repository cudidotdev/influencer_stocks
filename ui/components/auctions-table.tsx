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
import { AuctionStock } from "@/lib/stocks";
import Link from "next/link";

export function AuctionsTable({
  auctionedStocks: auctions,
}: {
  auctionedStocks: AuctionStock[];
}) {
  return (
    <div>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Stock ID</TableHead>
            <TableHead>Ticker</TableHead>
            <TableHead className="text-right">Total Shares</TableHead>
            <TableHead className="text-right">Lowest Bid Price</TableHead>
            <TableHead>End Time</TableHead>
            <TableHead className="text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {auctions.length === 0 ? (
            <TableRow>
              <TableCell colSpan={8} className="text-center">
                No active auctions
              </TableCell>
            </TableRow>
          ) : (
            auctions.map((auction) => (
              <TableRow key={auction.stock_id}>
                <TableCell>{auction.stock_id}</TableCell>
                <TableCell>{auction.ticker}</TableCell>
                <TableCell className="text-right">
                  {auction.total_shares}
                </TableCell>
                <TableCell className="text-right">
                  {auction.lowest_bid_price}
                </TableCell>
                <TableCell>{auction.auction_end}</TableCell>
                <TableCell className="text-right space-x-1">
                  <Button variant="outline" size="sm" asChild>
                    <Link href={"/place-bid?stock_id=" + auction.stock_id}>
                      View
                    </Link>
                  </Button>

                  <Button variant="outline" size="sm" asChild>
                    <Link href={"/place-bid?stock_id=" + auction.stock_id}>
                      Place Bid
                    </Link>
                  </Button>
                </TableCell>
              </TableRow>
            ))
          )}
        </TableBody>
      </Table>
    </div>
  );
}
