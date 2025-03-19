"use client";

import { useState } from "react";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import * as z from "zod";
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
import { toast } from "sonner";
import { Card, CardContent } from "@/components/ui/card";
import { AlertCircle, CheckCircle2 } from "lucide-react";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { useContract } from "@/providers/contract";

const formSchema = z.object({
  ticker: z
    .string()
    .min(2, { message: "Ticker must be at least 2 characters." })
    .max(10, { message: "Ticker must not exceed 10 characters." })
    .toUpperCase(),
  description: z.string().optional(),
  initialSupply: z
    .number()
    .min(100, { message: "Initial supply must be at least 100 shares." })
    .max(1000000, {
      message: "Initial supply must not exceed 1,000,000 shares.",
    }),
});

export function CreateStock() {
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  const { signingClient, contractClient, msgComposer } = useContract();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      ticker: "",
      initialSupply: 1_000_000,
    },
  });

  async function onSubmit(values: z.infer<typeof formSchema>) {
    setIsSubmitting(true);

    // Simulate API call to create stock
    try {
      if (!msgComposer || !signingClient || !contractClient)
        return toast.error("Please connect wallet");

      const msg = msgComposer.createStock({ ticker: values.ticker });

      await signingClient!.signAndBroadcast(
        contractClient.sender,
        [msg],
        "auto", // or specify gas
      );

      setIsSuccess(true);
      toast.success("Stock Created Successfully");

      // Reset form after success
      form.reset();

      // Reset success state after 3 seconds
      setTimeout(() => {
        setIsSuccess(false);
      }, 3000);
    } catch (error: any) {
      toast.error("Error Creating Stock: " + error?.message);
    } finally {
      setIsSubmitting(false);
    }
  }

  if (!contractClient?.sender) {
    return (
      <div className="flex justify-center p-4">
        Please conect wallet to continue
      </div>
    );
  }

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-xl font-semibold">Create New Stock</h2>
        <p className="text-muted-foreground">
          Create a new stock offering for your followers to invest in.
        </p>
      </div>

      {isSuccess && (
        <Alert className="bg-green-50 text-green-800 border-green-200">
          <CheckCircle2 className="h-4 w-4" />
          <AlertTitle>Success!</AlertTitle>
          <AlertDescription>
            Your stock has been created successfully.
          </AlertDescription>
        </Alert>
      )}

      <Card>
        <CardContent className="pt-6">
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)} className="space-y-8">
              <FormField
                control={form.control}
                name="ticker"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Ticker Symbol</FormLabel>
                    <FormControl>
                      <Input
                        placeholder="ALEX"
                        {...field}
                        className="uppercase"
                      />
                    </FormControl>
                    <FormDescription>
                      This is the symbol that will represent your stock (e.g.,
                      ALEX, EMMA).
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <FormField
                control={form.control}
                name="initialSupply"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Initial Supply</FormLabel>
                    <FormControl>
                      <Input
                        type="number"
                        {...field}
                        onChange={(e) => field.onChange(Number(e.target.value))}
                        disabled
                      />
                    </FormControl>
                    <FormDescription>
                      The total number of shares available for your stock.
                    </FormDescription>
                    <FormMessage />
                  </FormItem>
                )}
              />

              <Alert className="bg-blue-50 text-blue-800 border-blue-200">
                <AlertCircle className="h-4 w-4" />
                <AlertTitle>Important</AlertTitle>
                <AlertDescription>
                  After creating your stock, you&apos;ll need to start an
                  auction to allow investors to bid on shares.
                </AlertDescription>
              </Alert>

              <Button type="submit" disabled={isSubmitting}>
                {isSubmitting ? "Creating..." : "Create Stock"}
              </Button>
            </form>
          </Form>
        </CardContent>
      </Card>
    </div>
  );
}
