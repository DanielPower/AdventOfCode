import { readdirSync, writeFileSync } from "node:fs";

const languageNames = ["haskell", "python", "typescript", "rust"];
const years = readdirSync(".").filter((f) => f.match(/^\d{4}$/));

let progressTables = "";
for (const year of years) {
  const languages = readdirSync(year).filter((f) => languageNames.includes(f));
  progressTables += `# ${year}\n`;
  progressTables += "| Language | Progress |  \n";
  progressTables += "| -------- | -------- |  \n";
  for (const language of languages) {
    progressTables += `| ${language} |`;
    const list = (await import(`./${year}/${language}/list.ts`)).default;
    for (let day = 1; day <= 25; day++) {
      const parts = list.get(day) ?? 0;
      const symbol = parts === 2 ? "✅" : parts === 1 ? "🟡" : "❌";
      progressTables += symbol;
    }
    progressTables += "|\n";
  }
}

writeFileSync("README.md", progressTables);
console.log(progressTables);
