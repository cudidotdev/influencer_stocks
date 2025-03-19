import { NextRequest } from "next/server";

export const dynamic = "force-dynamic"; // Required for POST requests

export async function POST(req: NextRequest) {
  try {
    // Validate and decode the RPC URL
    const { searchParams } = new URL(req.url);
    const rpcUrl = searchParams.get("url");

    if (!rpcUrl) return new Response(null);

    // Security: Validate allowed RPC endpoints
    const allowedDomains = [process.env.NEXT_PUBLIC_RPC_ENDPOINT!];

    if (!allowedDomains.some((d) => rpcUrl.startsWith(d))) {
      return new Response("Unauthorized RPC endpoint", { status: 403 });
    }

    // Forward the request
    const body = await req.text();
    const headers = new Headers(req.headers);

    // Remove problematic headers
    headers.delete("host");
    headers.delete("origin");
    headers.delete("referer");

    const response = await fetch(rpcUrl, {
      method: "POST",
      headers,
      body,
      redirect: "follow",
    });

    // Handle RPC response
    const data = await response.text();

    return new Response(data, {
      status: response.status,
      headers: {
        "Content-Type":
          response.headers.get("Content-Type") || "text/plain;charset=UTF-8",
      },
    });
  } catch (error) {
    console.error("RPC Proxy Error:", error);
    return new Response("Internal Server Error", { status: 500 });
  }
}

export async function GET(req: NextRequest) {
  try {
    // Validate and decode the RPC URL
    const { searchParams } = new URL(req.url);
    const rpcUrl = searchParams.get("url");

    if (!rpcUrl) return new Response(null);

    // Security: Validate allowed RPC endpoints
    const allowedDomains = [process.env.NEXT_PUBLIC_RPC_ENDPOINT!];

    if (!allowedDomains.some((d) => rpcUrl.startsWith(d))) {
      return new Response("Unauthorized RPC endpoint", { status: 403 });
    }

    // Forward the request
    const body = await req.text();
    const headers = new Headers(req.headers);

    // Remove problematic headers
    headers.delete("host");
    headers.delete("origin");
    headers.delete("referer");

    const response = await fetch(rpcUrl, {
      method: "GET",
      headers,
      body,
      redirect: "follow",
    });

    // Handle RPC response
    const data = await response.text();

    return new Response(data, {
      status: response.status,
      headers: {
        "Content-Type":
          response.headers.get("Content-Type") || "text/plain;charset=UTF-8",
      },
    });
  } catch (error) {
    console.error("RPC Proxy Error:", error);
    return new Response("Internal Server Error", { status: 500 });
  }
}
