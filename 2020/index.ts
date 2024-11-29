import { Year } from "../types.ts";

export const year: Year = {
  typescript: {
    run: (day, part) => [
      "deno",
      `/data/${day.toString().padStart(2, "0")}/${part}.ts`,
    ],
    container: "denoland/deno:1.46.3",
  },
};
