"use client";

import { ContractClient } from "@/lib/contract/Contract.client";
import { ContractMsgComposer } from "@/lib/contract/Contract.message-composer";
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";

import {
  createContext,
  useContext,
  useEffect,
  useState,
  type ReactNode,
} from "react";
import { useWallet } from "./wallet";
import { OfflineSigner } from "@cosmjs/proto-signing";
import { toast } from "sonner";
import { GasPrice } from "@cosmjs/stargate";

interface ContractContextType {
  contractClient: ContractClient | null;
  signingClient: SigningCosmWasmClient | null;
  msgComposer: ContractMsgComposer | null;
}

const ContractContext = createContext<ContractContextType>({
  contractClient: null,
  signingClient: null,
  msgComposer: null,
});

export const useContract = () => useContext(ContractContext);

interface ContractProviderProps {
  children: ReactNode;
}

const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CONTRACT_ADDRESS!;

export function ContractProvider({ children }: ContractProviderProps) {
  const { address, offlineSigner } = useWallet();
  const [contractClient, setContractClient] = useState<ContractClient | null>(
    null,
  );
  const [signingClient, setSigningClient] =
    useState<SigningCosmWasmClient | null>(null);
  const [msgComposer, setMsgComposer] = useState<ContractMsgComposer | null>(
    null,
  );

  async function createClient(address: string, offlineSigner: OfflineSigner) {
    // encountered cors errors, so I had to improvise
    const rpcEndpoint =
      window.location.origin +
      "/api/rpc-proxy?url=" +
      process.env.NEXT_PUBLIC_RPC_ENDPOINT!;

    try {
      // Create signing client

      const signingClient = await SigningCosmWasmClient.connectWithSigner(
        rpcEndpoint,
        offlineSigner,
        {
          gasPrice: GasPrice.fromString("0.00001huahua"),
        },
      );

      // Create contract client instance
      const contractClient = new ContractClient(
        signingClient,
        address,
        CONTRACT_ADDRESS,
      );

      // Create Message Composer
      const msgComposer = new ContractMsgComposer(address, CONTRACT_ADDRESS);

      setContractClient(contractClient);
      setSigningClient(signingClient);
      setMsgComposer(msgComposer);
    } catch (error: any) {
      toast.error("Creating contract client failed: " + error?.message);
    }
  }

  useEffect(() => {
    if (!address || !offlineSigner) return setContractClient(null);

    createClient(address, offlineSigner);
  }, [address, offlineSigner]);

  return (
    <ContractContext.Provider
      value={{
        contractClient,
        signingClient,
        msgComposer,
      }}
    >
      {children}
    </ContractContext.Provider>
  );
}
