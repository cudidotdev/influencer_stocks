"use client";

import { createContext, useContext, useState, type ReactNode } from "react";
import chainInfo from "@/lib/chain-info";

interface WalletContextType {
  connected: boolean;
  connecting: boolean;
  address: string | null;
  offlineSigner: string | null;
  connect: () => Promise<void>;
  disconnect: () => void;
}

const WalletContext = createContext<WalletContextType>({
  connected: false,
  connecting: false,
  address: null,
  offlineSigner: null,
  connect: async () => {},
  disconnect: () => {},
});

export const useWallet = () => useContext(WalletContext);

interface WalletProviderProps {
  children: ReactNode;
}

export function WalletProvider({ children }: WalletProviderProps) {
  const [connected, setConnected] = useState(false);
  const [connecting, setConnecting] = useState(false);
  const [address, setAddress] = useState<string | null>(null);
  const [offlineSigner, setOfflineSigner] = useState<string | null>(null);

  // Mock wallet connection
  const connect = async () => {
    setConnecting(true);

    // Check if Keplr is installed
    if (!window.keplr) {
      alert("Please install Keplr extension");
      setConnecting(false);
      return;
    }

    try {
      // Add the Chihuahua testnet to Keplr
      await window.keplr.experimentalSuggestChain(chainInfo);

      // Enable the chain in Keplr
      await window.keplr.enable(chainInfo.chainId);

      // Get the offlineSigner from Keplr
      const offlineSigner = window.keplr.getOfflineSigner(chainInfo.chainId);

      // Get user account
      const accounts = await offlineSigner.getAccounts();

      setAddress(accounts[0].address);
      setOfflineSigner(offlineSigner);
    } catch (error: any) {
      console.error("Error connecting to Keplr:", error);
      alert("Failed to connect to Keplr: " + error?.message);
      return;
    }

    setConnected(true);
    setConnecting(false);
  };

  const disconnect = () => {
    setAddress(null);
    setConnected(false);
  };

  return (
    <WalletContext.Provider
      value={{
        connected,
        connecting,
        address,
        connect,
        disconnect,
        offlineSigner,
      }}
    >
      {children}
    </WalletContext.Provider>
  );
}
