"use client";

import { Button } from "@/components/ui/button";
import { Wallet } from "lucide-react";
import Link from "next/link";
import { useState } from "react";

export function Navbar() {
  const [connected, setConnected] = useState(false);

  const handleConnect = async () => {
    // This would be replaced with actual wallet connection logic
    setConnected(!connected);
  };

  return (
    <header className="border-b">
      <div className="w-full px-4 mx-auto sm:max-w-[40rem] md:max-w-[48rem] lg:max-w-[64rem] xl:max-w-[80rem]">
        <div className="flex h-16 items-center justify-between">
          <div className="flex items-center">
            <Link
              href="/"
              className="text-2xl font-bold bg-gradient-to-r from-blue-900 via-purple-800 to-pink-600 text-transparent bg-clip-text"
            >
              HypeX
            </Link>
          </div>
          <Button onClick={handleConnect} className="flex items-center gap-2">
            <Wallet className="h-4 w-4" />
            {connected ? "Connected" : "Connect Wallet"}
          </Button>
        </div>
      </div>
    </header>
  );
}
