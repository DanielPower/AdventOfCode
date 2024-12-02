import { Year } from "./types.ts";
import { getProjectResult } from "./util.ts";

const year = new Date().getFullYear().toString();
const projects: Year = (await import(`./${year}/index.ts`)).year;

Object.entries(projects).forEach(([language, project]) => {
  const promises: Promise<void>[] = [];
  for (let day = 1; day <= 25; day += 1) {
    const dayString = day.toString().padStart(2, "0");
    promises.push(getProjectResult(project, year, language, dayString, "1"));
    promises.push(getProjectResult(project, year, language, dayString, "2"));
  }
});
