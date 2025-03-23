import { PlaceBid } from "@/components/users/place-bid";
import { Suspense } from "react";

export default function PlaceBidPage() {
  return (
    <div className="container py-6 space-y-1">
      <p className="text-sm text-muted-foreground">
        Select a stock and place your bid. Auctions last for 24 hours.
      </p>

      <Suspense fallback={<div>Loading...</div>}>
        <PlaceBid />
      </Suspense>
    </div>
  );
}
