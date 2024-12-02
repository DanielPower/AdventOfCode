import { Year } from "../types.ts";

export const year: Year = {
  typescript: {
    exists: (root, day, part) => {
      const path = `${root}/${day}/${part}.ts`;
      try {
        Deno.statSync(path);
        return true;
      } catch {
        return false;
      }
    },
    run: (day, part) => ({
      args: ["deno", `/data/${day}/${part}.ts`],
    }),
    container: "denoland/deno:1.46.3",
  },
};
