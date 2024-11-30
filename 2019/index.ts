import { Year } from "../types.ts";

export const year: Year = {
  python: {
    run: (day, part) => [
      "python",
      `/data/${day.toString().padStart(2, "0")}/${part}.py`,
    ],
    container: "python:3.12",
  },
};
