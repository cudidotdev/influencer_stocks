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
import { useEffect, useState } from "react";
import { useContract } from "@/providers/contract";
import { toast } from "sonner";
import { useRouter } from "next/navigation";
import { ArrowRight } from "lucide-react";
import { ContractClient } from "@/lib/contract/Contract.client";

// Create bid form schema with zod
const quickSellFormSchema = z.object({
  shares: z.coerce
    .number()
    .positive("Number of shares must be positive")
    .int("Number of shares must be a whole number"),
  total_price: z.coerce.number().positive("Total Price must be positive"),
  slippage: z.coerce.number().positive("Slippage must be positive"),
});

type QuickSellFormValues = z.infer<typeof quickSellFormSchema>;

export function QuickSellForm({ stockId }: { stockId: number }) {
  const { signingClient, contractClient, msgComposer } = useContract();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [fetchingPrice, setFetchingPrice] = useState(false);
  const [maxShares, setMaxShares] = useState(0);
  const [step, setStep] = useState<"shares" | "price">("shares");
  const [totalPrice, setTotalPrice] = useState<number>();
  const router = useRouter();

  const form = useForm<QuickSellFormValues>({
    resolver: zodResolver(quickSellFormSchema),
    defaultValues: {
      shares: 1,
      total_price: 0,
      slippage: 5,
    },
  });

  const watchedShares = form.watch("shares");

  async function handleSubmit(values: QuickSellFormValues) {
    if (values.shares > maxShares)
      return toast.error("You exceed the number of shares you have");

    if (!values.total_price) return setStep("shares");

    try {
      if (!msgComposer || !signingClient || !contractClient)
        return toast.error("Please connect wallet");

      setIsSubmitting(true);

      const msg = msgComposer.quickSell({
        //@ts-ignore
        pricePerShare: Math.floor(
          (values.total_price * 1_000_000) / values.shares,
        ).toString(),
        slippage: values.slippage,
        shares: values.shares,
        stockId,
      });

      await signingClient!.signAndBroadcast(
        contractClient.sender,
        [msg],
        "auto", // or specify gas
      );

      router.push("/");
    } catch (error: any) {
      toast.error("Error: " + error?.message);
    } finally {
      setIsSubmitting(false);
    }
  }

  const handleContinue = async (
    contractClient: ContractClient,
    watchedShares: number,
    maxShares: number,
    stockId: number,
  ) => {
    const sharesResult = await form.trigger("shares");

    if (sharesResult) {
      if (watchedShares > maxShares)
        return toast.error(
          `You exceeded the number of available shares. ` + maxShares,
        );

      await getStockPrice(contractClient, +watchedShares, stockId);

      setStep("price");
    }
  };

  async function getTotalAvailabeShares(
    contractClient: ContractClient,
    stockId: number,
  ) {
    const sharesForSale = (await contractClient.getTotalBuyVolume({ stockId }))
      .amount;

    const userShares =
      (
        await contractClient.getSharesByOwner({ owner: contractClient.sender })
      ).shares.filter((share) => share.stock_id == stockId)[0]?.no_of_shares ||
      0;

    const maxShares = Math.min(sharesForSale, userShares);

    setMaxShares(maxShares);

    if (maxShares == 0) return toast.error("No share available for sale");
  }

  async function getStockPrice(
    contractClient: ContractClient,
    requestedShares: number,
    stockId: number,
  ) {
    try {
      setFetchingPrice(true);

      const price = (
        await contractClient.getSellPrice({ requestedShares, stockId })
      ).total_price;

      const toDecimal = +(+price / 1_000_000).toFixed(6);

      setTotalPrice(toDecimal);
    } catch (error: any) {
      // Handle error
      console.error("Failed to fetch minimum price:", error);
      toast.error("Failed to fetch minimum price: " + error?.message);
      setTotalPrice(undefined);
    } finally {
      setFetchingPrice(false);
    }
  }

  const handleBack = () => {
    setStep("shares");
  };

  useEffect(() => {
    if (totalPrice !== undefined) form.setValue("total_price", totalPrice);
    else form.resetField("total_price");
  }, [totalPrice]);

  useEffect(() => {
    if (contractClient) getTotalAvailabeShares(contractClient, stockId);
  }, [contractClient, stockId]);

  if (!contractClient?.sender) return <></>;

  return (
    <Form {...form}>
      <form className="mt-4" onSubmit={form.handleSubmit(handleSubmit)}>
        {step == "shares" ? (
          <div className="space-y-6" key={step}>
            <FormField
              control={form.control}
              name="shares"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Number of Shares to sell</FormLabel>
                  <FormControl>
                    <Input
                      type="number"
                      placeholder="Enter number of shares"
                      {...field}
                      //replace with max shares
                      max={1_000_000}
                    />
                  </FormControl>
                  <FormDescription>Max shares: {maxShares}</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <Button
              type="button"
              className="w-full cursor-pointer"
              onClick={() =>
                handleContinue(
                  contractClient,
                  watchedShares,
                  maxShares,
                  stockId,
                )
              }
              disabled={fetchingPrice || !watchedShares || watchedShares <= 0}
            >
              {fetchingPrice ? "Calculating price..." : "Continue"}
            </Button>
          </div>
        ) : (
          <div className="space-y-6" key={step}>
            <FormField
              control={form.control}
              name="total_price"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Total Price (HUAHUA)</FormLabel>
                  <FormControl>
                    <Input
                      type="number"
                      placeholder="Total Price"
                      disabled
                      {...field}
                    />
                  </FormControl>
                  <FormDescription>Total price of shares</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />

            <FormField
              control={form.control}
              name="slippage"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Slippage (%)</FormLabel>
                  <FormControl>
                    <Input type="number" placeholder="Slippage" {...field} />
                  </FormControl>
                  <FormDescription>
                    Amount of percentage you are willing to subceed the shown
                    price, if the price changes right before the transaction
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
                disabled={isSubmitting}
              >
                Change amount
              </Button>
              <Button
                disabled={isSubmitting}
                type="submit"
                className="flex gap-2 w-1/2 cursor-pointer"
              >
                {isSubmitting ? "Placing order" : "Place your order"}
                <ArrowRight />
              </Button>
            </div>
          </div>
        )}
      </form>
    </Form>
  );
}
