/* should not generate diagnostics */
const process = { env: { GIPHY_KEY: "value" } };

const apiKey = process.env.GIPHY_KEY;
