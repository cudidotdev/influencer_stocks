import { ContractClient } from "./contract/Contract.client";
import moment from "moment";

export type Stock = {
  id: number;
  ticker: string;
  total_shares: number;
  status: "upcoming" | "in_auction" | "trading";
  auction_start?: string;
  auction_end?: string;
  created_at: string;
  lowest_price?: string;
  lowest_bid?: string;
  total_shareholders?: number;
  total_bids?: number;
};

export type AuctionStock = {
  stock_id: number;
  ticker: string;
  total_shares: number;
  lowest_bid_price: string;
  auction_end: string;
};

export async function getStocksByInfluencer(
  contractClient: ContractClient,
  influencer: string,
) {
  const res = await contractClient.getStocksByInfluencer({
    influencer,
  });

  const stocks: Stock[] = [];

  for (const stock of res.stocks) {
    const status: Stock["status"] = !stock.auction_end
      ? "upcoming"
      : stock.marked_as_active_auction
        ? "in_auction"
        : "trading";

    const auction_start = stock.auction_start
      ? moment.utc(stock.auction_start).format("YYYY-MM-DD")
      : undefined;

    const auction_end = stock.auction_end
      ? moment.utc(stock.auction_end).format("YYYY-MM-DD")
      : undefined;

    const created_at = moment.utc(stock.created_at).format("YYYY-MM-DD");

    const shares_res = await contractClient.getSharesByStock({
      stockId: stock.id,
    });

    const formated_stock: Stock = {
      id: stock.id,
      ticker: stock.ticker,
      total_shares: stock.total_shares,
      status,
      auction_start,
      auction_end,
      created_at,
      total_shareholders: shares_res.shares.length,
    };

    if (status != "upcoming") {
      const bids_res = await contractClient.getBidsByStock({
        stockId: stock.id,
      });

      formated_stock.total_bids = bids_res.bids.length;
    }

    if (status == "in_auction") {
      const bids_res = await contractClient.getMinimumBidPrice({
        sharesRequested: 1,
        stockId: stock.id,
      });

      const lowest_bid = bids_res.min_price;

      formated_stock.lowest_bid = lowest_bid;
    }

    if (status == "trading") {
      let lowest_price = "0";

      try {
        const price_res = await contractClient.getBuyPrice({
          requestedShares: 1,
          stockId: stock.id,
        });
        lowest_price = price_res.price_per_share;
      } catch (error) {}

      formated_stock.lowest_price = lowest_price;
    }

    stocks.push(formated_stock);
  }

  return stocks;
}

export async function getAuctionedStock(contractClient: ContractClient) {
  const stocksInAuction = (
    await contractClient.getAllStocks({
      inAuction: true,
    })
  ).stocks;

  const auctionedStocks: AuctionStock[] = [];

  for (const stock of stocksInAuction) {
    const lowest_bid_price = (
      await contractClient.getMinimumBidPrice({
        sharesRequested: 1,
        stockId: stock.id,
      })
    ).min_price;

    const auction_end = moment.utc(stock.auction_end).format("YYYY-MM-DD");

    auctionedStocks.push({
      stock_id: stock.id,
      total_shares: stock.total_shares,
      ticker: stock.ticker,
      lowest_bid_price,
      auction_end,
    });
  }

  return auctionedStocks;
}

export async function getStockValue(
  contractClient: ContractClient,
  stockId: number,
) {
  // ordered by last sale first
  const sales_res = await contractClient.getSalesByStock({ stockId });

  const sales = sales_res.sales;

  // get value of last sale
  const value = sales?.[0].price_per_share || 0;

  // convert from uhuahua to huahua
  return value / 1_000_000;
}

export async function getStockById(
  contractClient: ContractClient,
  stockId: number,
) {
  const stock = (await contractClient.getStockById({ stockId })).stock;

  return stock;
}
