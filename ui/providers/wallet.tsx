"use client";

import {
  createContext,
  useContext,
  useEffect,
  useState,
  type ReactNode,
} from "react";
import chainInfo from "@/lib/chain-info";
import { OfflineSigner } from "@cosmjs/proto-signing";
import { toast } from "sonner";

interface WalletContextType {
  connected: boolean;
  connecting: boolean;
  address: string | null;
  offlineSigner: OfflineSigner | null;
  connect: () => Promise<void>;
  disconnect: () => void;
}

const WalletContext = createContext<WalletContextType>({
  connected: false,
  connecting: false,
  address: null,
  offlineSigner: null,
  connect: async () => {},
  disconnect: async () => {},
});

export const useWallet = () => useContext(WalletContext);

interface WalletProviderProps {
  children: ReactNode;
}

export function WalletProvider({ children }: WalletProviderProps) {
  const [connected, setConnected] = useState(false);
  const [connecting, setConnecting] = useState(false);
  const [address, setAddress] = useState<string | null>(null);
  const [offlineSigner, setOfflineSigner] = useState<OfflineSigner | null>(
    null,
  );

  // Mock wallet connection
  const connect = async () => {
    // Check if Keplr is installed
    if (!window.keplr) {
      toast.error("Please install Keplr extension");

      window.open("https://www.keplr.app/get", "_blank");

      return;
    }

    await setOfflineSignerAndAddress();
  };

  const disconnect = async () => {
    setAddress(null);
    setConnected(false);

    // Revoke permissions
    await window.keplr.disable(chainInfo.chainId);
  };

  async function setOfflineSignerAndAddress() {
    if (!window.keplr) return;

    try {
      setConnecting(true);

      // Add the Chihuahua testnet to Keplr
      await window.keplr.experimentalSuggestChain(chainInfo);

      // Enable the chain in Keplr
      await window.keplr.enable(chainInfo.chainId);

      // Get the offlineSigner from Keplr
      const offlineSigner: OfflineSigner = window.keplr.getOfflineSigner(
        chainInfo.chainId,
      );

      // Get user account
      const accounts = await offlineSigner.getAccounts();

      setAddress(accounts[0].address);
      setOfflineSigner(offlineSigner);

      setConnected(true);
    } catch (error: any) {
      toast.error("Error connecting to Keplr: " + error?.message);
    } finally {
      setConnecting(false);
    }
  }

  useEffect(() => {
    setOfflineSignerAndAddress();

    window.addEventListener("keplr_keystorechange", setOfflineSignerAndAddress);

    () => {
      window.removeEventListener(
        "keplr_keystorechange",
        setOfflineSignerAndAddress,
      );
    };
  }, []);

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
