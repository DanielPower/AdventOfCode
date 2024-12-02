import { Year } from "../types.ts";
import { fileExists } from "../util.ts";

export const year: Year = {
  rust: {
    exists: (root, day, part) => {
      const path = `${root}/${day}/part${part}`;
      return fileExists(path);
    },
    build: (day, part) => {
      return {
        args: ["cargo", "build"],
        cwd: `${day}/part${part}`,
      };
    },
    run: (day, part) => ({
      args: ["cargo", "run"],
      cwd: `${day}/part${part}`,
    }),
    container: "rust:1.82",
  },
};
