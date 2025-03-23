import { TradeStock } from "@/components/users/trade-stock";
import { Suspense } from "react";

export default function TradePage() {
  return (
    <div className="container py-6 space-y-1">
      <p className="text-sm text-muted-foreground">
        Select a stock and place your order.
      </p>

      <Suspense fallback={<div>Loading...</div>}>
        <TradeStock />
      </Suspense>
    </div>
  );
}
