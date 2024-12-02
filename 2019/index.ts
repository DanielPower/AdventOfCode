import { Year } from "../types.ts";

export const year: Year = {
  python: {
    exists: (root, day, part) => {
      const path = `${root}/${day}/${part}.py`;
      try {
        Deno.statSync(path);
        return true;
      } catch {
        return false;
      }
    },
    run: (day, part) => ({
      args: ["python", `/data/${day}/${part}.py`],
    }),
    container: "python:3.12",
  },
};
