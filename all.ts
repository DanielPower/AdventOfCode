import { Year } from "./types.ts";
import { getProjectResult } from "./util.ts";

const yearDirectories = [...Deno.readDirSync(".")]
  .map((f) => f.name)
  .filter((f) => f.match(/^\d{4}$/));
const years: Array<[string, Year]> = await Promise.all(
  yearDirectories.map(async (year) => [
    year,
    (await import(`./${year}/index.ts`)).year as Year,
  ]),
);

years.forEach(([year, projects]) => {
  Object.entries(projects).forEach(([language, project]) => {
    const promises: Promise<void>[] = [];
    for (let day = 1; day <= 25; day += 1) {
      const dayString = day.toString().padStart(2, "0");
      promises.push(getProjectResult(project, year, language, dayString, "1"));
      promises.push(getProjectResult(project, year, language, dayString, "2"));
    }
  });
});
