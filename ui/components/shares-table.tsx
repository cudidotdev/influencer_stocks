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
import { Share } from "@/lib/shares";
import Link from "next/link";

export function SharesTable({ shares }: { shares: Share[] }) {
  return (
    <div>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Stock ID</TableHead>
            <TableHead>Ticker</TableHead>
            <TableHead className="text-right">Shares</TableHead>
            <TableHead className="text-right">Value/Share</TableHead>
            <TableHead className="text-right">Total Value</TableHead>
            <TableHead className="text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {shares.length === 0 ? (
            <TableRow>
              <TableCell colSpan={6} className="text-center">
                <div className="pt-4 flex gap-4 justify-center items-center">
                  <p>You don&apos;t own any shares yet</p>

                  <Button variant="outline" asChild>
                    <Link href="/trade">Buy Stocks</Link>
                  </Button>
                </div>
              </TableCell>
            </TableRow>
          ) : (
            shares.map((share) => (
              <TableRow key={share.id}>
                <TableCell>{share.stock_id}</TableCell>
                <TableCell>{share.ticker}</TableCell>
                <TableCell className="text-right">
                  {share.no_of_shares}
                </TableCell>
                <TableCell className="text-right">
                  {share.value_per_share}
                </TableCell>
                <TableCell className="text-right">
                  {share.total_value}
                </TableCell>
                <TableCell className="text-right">
                  <Button asChild variant="outline" size="sm" className="mr-2">
                    <Link href={"/trade?stock_id=" + share.stock_id}>Buy</Link>
                  </Button>

                  <Button variant="outline" size="sm">
                    <Link href={"/trade?stock_id=" + share.stock_id}>Sell</Link>
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
