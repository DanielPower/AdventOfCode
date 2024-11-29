import { readdirSync } from "node:fs";
import { Project, Year } from "./types.ts";

const yearDirectories = readdirSync(".").filter((f) => f.match(/^\d{4}$/));
const years: Array<[string, Year]> = await Promise.all(
  yearDirectories.map(async (year) => [
    year,
    (await import(`./${year}/index.ts`)).year as Year,
  ]),
);

const run = async (
  project: Project,
  year: string,
  language: string,
  day: number,
  part: number,
) => {
  const dayString = day.toString().padStart(2, "0");
  let inputReadable;
  let expectedReadable;
  try {
    inputReadable = (await Deno.open(`${year}/data/${dayString}.in`)).readable;
  } catch {
    return {
      result: false,
      expected: null,
      actual: null,
      error: "Can't read input file",
    };
  }
  try {
    expectedReadable = (
      await Deno.open(`${year}/data/${dayString}.${part}.out`)
    ).readable;
  } catch {
    return {
      result: false,
      expected: null,
      actual: null,
      error: "Can't read expected output file",
    };
  }

  const child = new Deno.Command("docker", {
    args: [
      "run",
      "-i",
      "--rm",
      "-v",
      `${Deno.cwd()}/${year}/${language}:/data:z`,
      project.container,
      ...project.run(day, part),
    ],
    stdin: "piped",
    stdout: "piped",
    stderr: "piped",
  }).spawn();

  const writer = child.stdin.getWriter();
  for await (const buffer of inputReadable) {
    await writer.write(buffer);
  }
  writer.close();

  const stdout = new TextDecoder().decode(
    (await child.stdout.getReader().read()).value,
  );
  const stderr = new TextDecoder().decode(
    (await child.stderr.getReader().read()).value,
  );

  const expected = new TextDecoder().decode(
    (await expectedReadable.getReader().read()).value,
  );

  const result = stdout === expected;
  return { result, expected, actual: stdout, error: stderr };
};

years.forEach(([year, projects]) => {
  Object.entries(projects).forEach(async ([language, project]) => {
    for (let day = 1; day <= 25; day += 1) {
      for (let part = 1; part <= 2; part += 1) {
        try {
          const { result, expected, actual, error } = await run(
            project,
            year,
            language,
            day,
            part,
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
