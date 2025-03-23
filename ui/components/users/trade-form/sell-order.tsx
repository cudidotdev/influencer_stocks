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
const sellOrderFormSchema = z.object({
  shares: z.coerce
    .number()
    .positive("Number of shares must be positive")
    .int("Number of shares must be a whole number"),
  price: z.coerce
    .number()
    .positive("Bid price must be positive")
    .multipleOf(0.000001, "Price must have at most 6 decimal places"),
});

type SellOrderFormValues = z.infer<typeof sellOrderFormSchema>;

export function SellOrderForm({ stockId }: { stockId: number }) {
  const { signingClient, contractClient, msgComposer } = useContract();
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [maxShares, setMaxShares] = useState(0);
  const router = useRouter();

  const form = useForm<SellOrderFormValues>({
    resolver: zodResolver(sellOrderFormSchema),
    defaultValues: {
      shares: 1,
      price: 0.001,
    },
  });

  async function getNumberOfShares(
    contractClient: ContractClient,
    stockId: number,
  ) {
    const shares = (
      await contractClient.getSharesByOwner({ owner: contractClient.sender })
    ).shares.filter((share) => share.stock_id == stockId);

    if (shares.length == 0) {
      setMaxShares(0);
      return toast.error("You have no share in this stock");
    }

    setMaxShares(shares[0].no_of_shares);
  }

  async function handleSubmit(values: SellOrderFormValues) {
    if (values.shares > maxShares)
      return toast.error("You exceed the number of shares you have");

    try {
      setIsSubmitting(true);

      if (!msgComposer || !signingClient || !contractClient)
        return toast.error("Please connect wallet");

      const msg = msgComposer.createSellOrder({
        //@ts-ignore
        pricePerShare: Math.floor(values.price * 1_000_000).toString(),
        shares: values.shares,
        stockId,
      });

      await signingClient!.signAndBroadcast(
        contractClient.sender,
        [msg],
        "auto", // or specify gas
      );

      router.push("/orders");
    } catch (error: any) {
      toast.error("Error: " + error?.message);
    } finally {
      setIsSubmitting(false);
    }
  }

  useEffect(() => {
    if (contractClient?.sender) {
      getNumberOfShares(contractClient, stockId);
    }
  }, [contractClient?.sender, stockId]);

  return (
    <Form {...form}>
      <form
        className="space-y-6 mt-4"
        onSubmit={form.handleSubmit(handleSubmit)}
      >
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

        <FormField
          control={form.control}
          name="price"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Price per Share</FormLabel>
              <FormControl>
                <Input type="number" placeholder="Price per share" {...field} />
              </FormControl>
              <FormDescription>
                Enter the price you want to sell each share
              </FormDescription>
              <FormMessage />
            </FormItem>
          )}
        />

        <Button
          disabled={isSubmitting}
          type="submit"
          className="flex gap-2 w-full cursor-pointer"
        >
          {isSubmitting ? "Placing order" : "Place your order"}
          <ArrowRight />
        </Button>
      </form>
    </Form>
  );
}
