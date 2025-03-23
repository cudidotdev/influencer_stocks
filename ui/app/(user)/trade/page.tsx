"use client";

import { TradeStock } from "@/components/users/trade-stock";
import { useSearchParams } from "next/navigation";
import { useEffect, useState } from "react";

export default function TradePage() {
  const searchParams = useSearchParams();
  const [stockId, setStockId] = useState<string>();

  useEffect(() => {
    let stockId = searchParams.get("stock_id");

    if (stockId) setStockId(stockId);
    else setStockId(undefined);
  }, [searchParams]);

  return (
    <div className="container py-6 space-y-1">
      <p className="text-sm text-muted-foreground">
        Select a stock and place your order.
      </p>

      <TradeStock stockId={stockId} />
    </div>
  );
}
