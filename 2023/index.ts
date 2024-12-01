import { Year } from "../types.ts";
import { fileExists } from "../util.ts";

export const year: Year = {
  rust: {
    exists: (root, day, part) => {
      const path = `${root}/${day.toString().padStart(2, "0")}/part${part}`;
      return fileExists(path);
    },
    build: (day, part) => {
      return {
        args: ["cargo", "build"],
        cwd: `${day.toString().padStart(2, "0")}/part${part}`,
      };
    },
    run: (day, part) => ({
      args: ["cargo", "run"],
      cwd: `${day.toString().padStart(2, "0")}/part${part}`,
    }),
    container: "rust:1.82",
  },
};
