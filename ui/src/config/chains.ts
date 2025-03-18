export const CHIHUAHUA_TESTNET = {
  chainId: "woofnet-5",
  chainName: "Chihuahua Testnet",
  rpc: "http://testnet.chihuahua.wtf:26557/",
  rest: "http://testnet.chihuahua.wtf:1317/",
  bech32Prefix: "chihuahua",
  coinType: 118,
  currencies: [
    {
      coinDenom: "HUAHUA",
      coinMinimalDenom: "uhuahua",
      coinDecimals: 6,
    },
  ],
  gasPriceStep: { low: 0.01, average: 0.025, high: 0.03 },
};
