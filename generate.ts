import { readdirSync } from "node:fs";
import { Year } from "./types.ts";
import { build, run, test } from "./util.ts";

const yearDirectories = readdirSync(".").filter((f) => f.match(/^\d{4}$/));
const years: Array<[string, Year]> = await Promise.all(
  yearDirectories.map(async (year) => [
    year,
    (await import(`./${year}/index.ts`)).year as Year,
  ]),
);

years.forEach(([year, projects]) => {
  Object.entries(projects).forEach(async ([language, project]) => {
    for (let day = 1; day <= 25; day += 1) {
      for (let part = 1; part <= 2; part += 1) {
        try {
          if (!project.exists(`${year}/${language}`, day, part)) {
            continue;
          }
          const buildResult = await build(project, year, language, day, part);
          if (!buildResult) {
            console.log(`❌ ${year}: Day ${day} Part ${part}: Build Failure`);
            continue;
          }
          const { stdout } = await run(project, year, language, day, part);
          const { result, actual, expected } = await test(
            year,
            day,
            part,
            stdout!,
          );
          const dayString = day.toString().padStart(2, "0");
          if (result) {
            console.log(`✅ ${year}: Day ${dayString} Part ${part}: Success`);
          } else {
            console.log(`❌ ${year}: Day ${dayString} Part ${part}: Failure`);
            console.log(`Expected: ${expected}`);
            console.log(`Recevied: ${actual}`);
          }
        } catch (e) {
          console.log(`❌ ${year}: Day ${day} Part ${part}: Failure`);
          console.log(`Unexpected error`);
          console.log(e);
        }
      }
    }
  });
});
