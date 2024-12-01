import { Year } from "../types.ts";
import { fileExists } from "../util.ts";

export const year: Year = {
  rust: {
    exists: (day, part) => {
      const path = `/data/${day.toString().padStart(2, "0")}/${part}.rs`;
      return fileExists(path);
    },
    build: (day, part) => {
      return ["rustc", `${day.toString().padStart(2, "0")}/${part}.rs`];
    },
    run: (day, part) => [
      "deno",
      `/data/${day.toString().padStart(2, "0")}/${part}.ts`,
    ],
    container: "rust:1.55.0",
  },
};
