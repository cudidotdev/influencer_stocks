// Define the Chihuahua testnet chain info
export default {
  chainId: "woofnet-5",
  chainName: "Chihuahua Testnet",
  rpc: "http://testnet.chihuahua.wtf:26557",
  rest: "http://testnet.chihuahua.wtf:8885",
  bip44: {
    coinType: 118,
  },
  bech32Config: {
    bech32PrefixAccAddr: "chihuahua",
    bech32PrefixAccPub: "chihuahuapub",
    bech32PrefixValAddr: "chihuahuavaloper",
    bech32PrefixValPub: "chihuahuavaloperpub",
    bech32PrefixConsAddr: "chihuahuavalcons",
    bech32PrefixConsPub: "chihuahuavalconspub",
  },
  currencies: [
    {
      coinDenom: "HUAHUA",
      coinMinimalDenom: "uhuahua",
      coinDecimals: 6,
    },
  ],
  feeCurrencies: [
    {
      coinDenom: "HUAHUA",
      coinMinimalDenom: "uhuahua",
      coinDecimals: 6,
    },
  ],
  stakeCurrency: {
    coinDenom: "HUAHUA",
    coinMinimalDenom: "uhuahua",
    coinDecimals: 6,
  },
  coinType: 118,
  gasPriceStep: {
    low: 0.01,
    average: 0.025,
    high: 0.04,
  },
};
