import { join } from "path";

export default {
  contracts: [
    {
      name: "Auction",
      dir: join(__dirname, "../contract/schema"),
    },
  ],
  outPath: "./src/codegen",
  options: {
    bundle: { bundleFile: "index.ts", scope: "contracts" },
    types: { enabled: true },
    client: { enabled: true },
    reactQuery: { enabled: true },
  },
};
