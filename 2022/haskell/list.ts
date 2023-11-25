import { readdirSync } from "node:fs";

export default new Map(
  readdirSync(`${import.meta.dir}/src`)
    .filter((day) => day.match(/^Day\d+.hs$/))
    .map((day) => [parseInt(day.match(/\d+/)![0], 10), 2]),
);
