"use client";

import { Button } from "@/components/ui/button";
import { Wallet } from "lucide-react";
import { useState } from "react";

export function Navbar() {
  const [connected, setConnected] = useState(false);

  const handleConnect = async () => {
    // This would be replaced with actual wallet connection logic
    setConnected(!connected);
  };

  return (
    <header className="border-b">
      <div className="w-full px-4 mx-auto sm:max-w-[640px] md:max-w-[768px] lg:max-w-[1024px]">
        <div className="flex h-16 items-center justify-between">
          <div className="flex items-center">
            <a
              href="/"
              className="text-2xl font-bold bg-gradient-to-r from-blue-900 via-purple-800 to-pink-600 text-transparent bg-clip-text"
            >
              HypeX
            </a>
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
