import { readdirSync, writeFileSync } from "node:fs";
import { markdownTable } from "markdown-table";

const languageNames = ["haskell", "python", "typescript", "rust"];
const tables = await Promise.all(
  readdirSync(".")
    .filter((f) => f.match(/^\d{4}$/))
    .map(async (year) => {
      const list = await Promise.all(
        readdirSync(year)
          .filter((f) => languageNames.includes(f))
          .map(
            async (language) =>
              (await import(`./${year}/${language}/list.ts`)).default,
          ),
      );
      return [year, list];
    }),
);

console.log(
  markdownTable([
    ["Language", "Stars", "Progress"],
    ["a", "b", "c"],
  ]),
);
console.log(tables);

// writeFileSync("README.md", progressTables);
// console.log(progressTables);
