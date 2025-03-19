"use client";

import { Button } from "@/components/ui/button";
import { useWallet } from "@/providers/wallet";
import { Wallet } from "lucide-react";
import Link from "next/link";

export function Navbar() {
  const { connected, connect, disconnect, address, connecting } = useWallet();

  return (
    <header className="border-b sticky top-0 bg-white z-50">
      <div className="w-full px-4 sm:px-8 mx-auto max-w-[80rem]">
        <div className="flex h-16 items-center justify-between">
          <div className="flex items-center">
            <Link
              href="/"
              className="text-2xl font-bold bg-gradient-to-r from-blue-900 via-purple-900 to-pink-700 text-transparent bg-clip-text"
            >
              HypeX
            </Link>
          </div>

          {!!address && (
            <span className="hidden md:inline-flex items-center rounded-full bg-green-100 px-4 py-1 text-sm font-medium text-green-800">
              {address}
            </span>
          )}

          <Button
            onClick={() => (connected ? disconnect() : connect())}
            className="flex items-center gap-2"
          >
            <Wallet className="h-4 w-4" />
            {connecting
              ? "Connecting..."
              : connected
                ? "Disconnect"
                : "Connect Wallet"}
          </Button>
        </div>
      </div>
    </header>
  );
}
