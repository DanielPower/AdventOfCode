import { existsSync, readdirSync } from "node:fs";

export default new Map(
  readdirSync(import.meta.dir)
    .filter((day) => day.match(/^\d{2}$/))
    .map((day) => [
      parseInt(day, 10),
      [
        `${import.meta.dir}/${day}/1.ts`,
        `${import.meta.dir}/${day}/2.ts`,
      ].filter((part) => existsSync(part)).length,
    ]),
);
