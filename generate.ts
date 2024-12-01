import { readdirSync } from "node:fs";
import { Project, Year } from "./types.ts";
import { build, run, test } from "./util.ts";

const yearDirectories = readdirSync(".").filter((f) => f.match(/^\d{4}$/));
const years: Array<[string, Year]> = await Promise.all(
  yearDirectories.map(async (year) => [
    year,
    (await import(`./${year}/index.ts`)).year as Year,
  ]),
);

const getProjectResult = async (
  project: Project,
  year: string,
  language: string,
  day: number,
  part: number,
) => {
  try {
    if (!project.exists(`${year}/${language}`, day, part)) {
      return;
    }
    await build(project, year, language, day, part);
    const { stdout } = await run(project, year, language, day, part);
    const { result, actual, expected } = await test(year, day, part, stdout!);
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
};

years.forEach(([year, projects]) => {
  Object.entries(projects).forEach(([language, project]) => {
    const promises: Promise<void>[] = [];
    for (let day = 1; day <= 25; day += 1) {
      for (let part = 1; part <= 2; part += 1) {
        promises.push(getProjectResult(project, year, language, day, part));
      }
    }
  });
});
