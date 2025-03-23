"use client";

import { Check, ChevronsUpDown, TrendingUp } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { cn } from "@/lib/utils";
import { useEffect, useState } from "react";
import { Badge } from "@/components/ui/badge";

type Stock = {
  stock_id: number;
  ticker: string;
  lowest_bid_price?: string;
};

interface StockSelectorProps {
  stocks: Stock[];
  selectedStock: Stock | null;
  onSelect: (stock: Stock) => void;
  title: string;
}

export function StockSelector({
  title,
  stocks: _stocks,
  selectedStock,
  onSelect,
}: StockSelectorProps) {
  const [open, setOpen] = useState(false);
  const [stocks, setStocks] = useState(_stocks);

  useEffect(() => {
    setStocks(_stocks);
  }, [_stocks]);

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className="w-full justify-between h-14 px-4 border-2 bg-card cursor-pointer"
        >
          {selectedStock ? (
            <div className="flex items-center gap-3">
              <div className="flex h-8 w-8 items-center justify-center rounded-full bg-primary/10">
                <TrendingUp className="h-4 w-4 text-primary" />
              </div>
              <div className="flex flex-col items-start">
                <span className="font-medium">{selectedStock.ticker}</span>
                <span className="text-xs text-muted-foreground">
                  {selectedStock.ticker}
                </span>
              </div>
            </div>
          ) : (
            <span>{title}</span>
          )}
          <ChevronsUpDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-full p-0">
        <Command className="sm:w-[30rem]">
          <CommandInput placeholder="Search stocks..." />
          <CommandList>
            <CommandEmpty>No stocks found.</CommandEmpty>
            <CommandGroup>
              {stocks.map((stock) => (
                <CommandItem
                  key={stock.stock_id}
                  value={stock.ticker}
                  onSelect={() => {
                    onSelect(stock);
                    setOpen(false);
                  }}
                  className="py-3 cursor-pointer"
                >
                  <div className="flex items-center gap-3 w-full">
                    <div className="flex h-8 w-8 items-center justify-center rounded-full bg-primary/10">
                      <TrendingUp className="h-4 w-4 text-primary" />
                    </div>
                    <div className="flex flex-col flex-1">
                      <div className="flex items-center justify-between">
                        <span className="font-medium">{stock.ticker}</span>
                        <Badge variant="outline" className="ml-2 font-mono">
                          {stock.ticker}
                        </Badge>
                      </div>
                      {!!stock.lowest_bid_price && (
                        <div className="flex items-center justify-between">
                          <span className="text-xs text-muted-foreground">
                            Lowest Bid Price: {stock.lowest_bid_price} HUAHUA
                          </span>
                        </div>
                      )}
                    </div>
                  </div>
                  <Check
                    className={cn(
                      "ml-auto h-4 w-4",
                      selectedStock?.stock_id === stock.stock_id
                        ? "opacity-100"
                        : "opacity-0",
                    )}
                  />
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
