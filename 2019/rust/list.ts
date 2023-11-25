import { existsSync, readdirSync } from "node:fs";

export default new Map(
  readdirSync(import.meta.dir)
    .filter((day) => day.match(/^\d{2}$/))
    .map((day) => [
      parseInt(day, 10),
      [
        `${import.meta.dir}/${day}/1.rs`,
        `${import.meta.dir}/${day}/2.rs`,
      ].filter((part) => existsSync(part)).length,
    ]),
);
