"use client";

import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { useState } from "react";
import { useContract } from "@/providers/contract";
import { toast } from "sonner";
import { useWallet } from "@/providers/wallet";
import { useRouter } from "next/navigation";

// Create bid form schema with zod
const bidFormSchema = z.object({
  shares: z.coerce
    .number()
    .positive("Number of shares must be positive")
    .int("Number of shares must be a whole number"),
  price: z.coerce
    .number()
    .positive("Bid price must be positive")
    .multipleOf(0.000001, "Price must have at most 6 decimal places"),
});

type BidFormValues = z.infer<typeof bidFormSchema>;

export function BidForm({ stockId }: { stockId: number }) {
  const [loading, setLoading] = useState(false);
  const [fetchingPrice, setFetchingPrice] = useState(false);
  const { connect } = useWallet();
  const { signingClient, contractClient, msgComposer } = useContract();
  const [minPrice, setMinPrice] = useState<number | undefined>(undefined);
  const [step, setStep] = useState<"shares" | "price">("shares");
  const router = useRouter();

  const form = useForm<BidFormValues>({
    resolver: zodResolver(bidFormSchema),
    defaultValues: {
      shares: 1,
    },
  });

  const watchedShares = form.watch("shares");

  const handleSubmit = async (values: BidFormValues) => {
    try {
      if (minPrice && values.price < minPrice)
        return toast.error(`The minimum price is ${minPrice}`);

      if (!msgComposer || !signingClient || !contractClient) {
        connect();
        return toast.error("Please connect wallet");
      }

      setLoading(true);

      console.log(values);

      const msg = msgComposer.placeBid(
        {
          //@ts-ignore
          pricePerShare: Math.floor(values.price * 1_000_000).toString(), // Convert to string
          shares: values.shares,
          stockId: stockId,
        },
        [
          {
            amount: Math.floor(
              values.price * 1_000_000 * values.shares,
            ).toString(),
            denom: "uhuahua",
          },
        ],
      );

      await signingClient.signAndBroadcast(
        contractClient.sender,
        [msg],
        "auto",
      );

      router.push("/my-bids");

      setStep("shares");
      setMinPrice(undefined);
    } catch (error: any) {
      // Handle error
      toast.error("Error: " + error?.message);
    } finally {
      setLoading(false);
    }
  };

  async function getMinimumBidPrice(sharesRequested: number, stockId: number) {
    if (!contractClient || !stockId || !sharesRequested) return;

    try {
      setFetchingPrice(true);

      const min_price = (
        await contractClient.getMinimumBidPrice({ sharesRequested, stockId })
      ).min_price;

      const toDecimal = +(+min_price / 1_000_000).toFixed(6);

      setMinPrice(toDecimal);
      form.setValue("price", toDecimal);
    } catch (error: any) {
      // Handle error
      console.error("Failed to fetch minimum price:", error);
      toast.error("Failed to fetch minimum price: " + error?.message);
      setMinPrice(undefined);
    } finally {
      setFetchingPrice(false);
    }
  }

  const handleContinue = async (watchedShares: number) => {
    const sharesResult = await form.trigger("shares");

    if (sharesResult) {
      if (watchedShares > 1_000_000)
        return toast.error(
          `You exceeded the number of available shares. Max 1,000,000`,
        );

      setFetchingPrice(true);
      await getMinimumBidPrice(+watchedShares, stockId);
      setStep("price");
    }
  };

  const handleBack = () => {
    setStep("shares");
  };

  return (
    <div>
      <div className="mb-6">
        <h2 className="text-lg font-semibold">Place your bid</h2>
        <p className="text-sm text-muted-foreground">
          {step === "shares"
            ? "Step 1: Enter the number of shares you want to bid on"
            : "Step 2: Enter your bid price (must be at least the minimum price)"}
        </p>
      </div>

      <Form {...form}>
        <form onSubmit={form.handleSubmit(handleSubmit)} className="space-y-6">
          {step === "shares" ? (
            <>
              <FormField
                control={form.control}
                name="shares"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Number of Shares</FormLabel>
                    <FormControl>
                      <Input
                        type="number"
                        placeholder="Enter number of shares"
                        {...field}
                        max={1_000_000}
                      />
                    </FormControl>
                    <FormDescription>
                      Enter the number of shares you want to bid on
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <Button
                type="button"
                className="w-full cursor-pointer"
                onClick={() => handleContinue(watchedShares)}
                disabled={fetchingPrice || !watchedShares || watchedShares <= 0}
              >
                {fetchingPrice ? "Calculating minimum price..." : "Continue"}
              </Button>
            </>
          ) : (
            <>
              <div className="p-4 bg-muted rounded-lg mb-4">
                <div className="flex justify-between">
                  <span className="font-medium">Number of Shares:</span>
                  <span>{watchedShares}</span>
                </div>
                <div className="flex justify-between mt-2">
                  <span className="font-medium">Minimum Bid Price:</span>
                  <span>{minPrice} HUAHUA</span>
                </div>
              </div>

              <FormField
                control={form.control}
                name="price"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Your Bid Price (per share)</FormLabel>
                    <FormControl>
                      <Input
                        type="number"
                        placeholder={`Enter price (min: ${minPrice} HUAHUA)`}
                        {...field}
                      />
                    </FormControl>
                    <FormDescription>
                      Enter the price you&apos;re willing to pay per share
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <div className="flex gap-4">
                <Button
                  type="button"
                  variant="outline"
                  className="w-1/2 cursor-pointer"
                  onClick={handleBack}
                  disabled={loading}
                >
                  Change amount
                </Button>
                <Button
                  type="submit"
                  className="w-1/2 cursor-pointer"
                  disabled={loading}
                >
                  {loading ? "Placing Bid..." : "Place Bid"}
                </Button>
              </div>
            </>
          )}
        </form>
      </Form>
    </div>
  );
}
