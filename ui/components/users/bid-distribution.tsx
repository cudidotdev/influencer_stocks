import { Bid } from "@/lib/contract/Contract.types";
import { useMemo } from "react";

export function BidDistribution({ bids: _bids }: { bids: Bid[] }) {
  // sort by no_of_shares
  const bids = useMemo(
    () => _bids.sort((a, b) => b.remaining_shares - a.remaining_shares),
    [_bids],
  );

  function truncate(item: string) {
    if (item.length < 27) return item;

    return `${item.slice(0, 15)}...${item.slice(-9)}`;
  }

  return (
    <div className="w-full space-y-2">
      <h2 className="text-lg font-semibold mb-4">Bid Distribution</h2>

      {bids.map((bid) => (
        <div
          key={bid.id}
          className="p-2 bg-zinc-200 rounded-lg border border-zinc-400"
        >
          <p className="mb-1 text-sm font-mono text-zinc-500">
            {truncate(bid.bidder)}
          </p>
          <div className="border-zinc-400 w-full border rounded-2xl h-[0.75rem] mb-2 bg-zinc-100">
            <div
              style={{ width: `${(bid.remaining_shares / 1_000_000) * 100}%` }}
              className="h-full bg-zinc-700 rounded-2xl"
            ></div>
          </div>
          <p className="text-amber-600 text-sm">
            Shares: {bid.remaining_shares.toLocaleString()}
          </p>
          <p className="text-amber-600 text-sm">
            Bid price: {(bid.price_per_share / 1_000_000).toFixed(6)}{" "}
          </p>
        </div>
      ))}
    </div>
  );
}
