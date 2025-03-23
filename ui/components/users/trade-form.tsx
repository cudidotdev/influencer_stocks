import { useState } from "react";
import { Card, CardContent, CardTitle } from "../ui/card";
import { Tabs, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { SellOrderForm } from "./trade-form/sell-order";
import { BuyOrderForm } from "./trade-form/buy-order";
import { QuickBuyForm } from "./trade-form/quick-buy";
import { QuickSellForm } from "./trade-form/quick-sell";
import { Stock } from "@/lib/contract/Contract.types";

export function TradeForm({ stock }: { stock: Stock }) {
  const [type, setType] = useState("quick");
  const [action, setAction] = useState("buy");

  return (
    <Card className="w-full max-w-[30rem]">
      <CardContent>
        <h2 className="text-xl mb-4 font-semibold">Trade {stock.ticker}</h2>

        <Tabs
          defaultValue="quick"
          value={type}
          onValueChange={(value) => setType(value)}
          className="w-full"
        >
          <TabsList className="mb-0.5 !h-fit !p-1 w-full">
            <TabsTrigger value="quick" className="px-6 py-1.5 cursor-pointer">
              Quick
            </TabsTrigger>
            <TabsTrigger value="order" className="px-6 py-1.5 cursor-pointer">
              Order
            </TabsTrigger>
          </TabsList>
        </Tabs>

        <Tabs
          defaultValue="buy"
          value={action}
          onValueChange={(value) => setAction(value)}
          className="w-full"
        >
          <TabsList className="mb-4 !h-fit !p-1 w-full">
            <TabsTrigger value="buy" className="px-6 py-1.5 cursor-pointer">
              Buy
            </TabsTrigger>
            <TabsTrigger value="sell" className="px-6 py-1.5 cursor-pointer">
              Sell
            </TabsTrigger>
          </TabsList>
        </Tabs>

        {type == "order" && action == "sell" && (
          <SellOrderForm stockId={stock.id} />
        )}

        {type == "order" && action == "buy" && (
          <BuyOrderForm stockId={stock.id} />
        )}

        {type == "quick" && action == "buy" && (
          <QuickBuyForm stockId={stock.id} />
        )}

        {type == "quick" && action == "sell" && (
          <QuickSellForm stockId={stock.id} />
        )}
      </CardContent>
    </Card>
  );
}
