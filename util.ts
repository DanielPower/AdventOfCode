import { Project } from "./types.ts";

export const formatDay = (day: number) => day.toString().padStart(2, "0");

export const fetchInput = async (year: string, day: number) => {
  const dayString = formatDay(day);
  const response = await fetch(
    `https://adventofcode.com/${year}/day/${day}/input`,
    {
      headers: {
        cookie: `session=${Deno.env.get("AOC_SESSION")}`,
      },
    },
  );

  if (response.status !== 200) {
    throw new Error(`Failed to fetch input: ${response.status}`);
  }

  const input = await response.text();
  await Deno.writeTextFile(`${year}/data/${dayString}.in`, input);
};

export const run = async (
  project: Project,
  year: string,
  language: string,
  day: number,
  part: number,
) => {
  const dayString = day.toString().padStart(2, "0");
  let inputReadable;
  try {
    inputReadable = (await Deno.open(`${year}/data/${dayString}.in`)).readable;
  } catch {
    await fetchInput(year, day);
    inputReadable = (await Deno.open(`${year}/data/${dayString}.in`)).readable;
    return {
      result: false,
      expected: null,
      actual: null,
      error: "Can't read input file",
    };
  }

  const { cwd, args } = project.run(day, part);
  const child = new Deno.Command("docker", {
    args: [
      "run",
      "-i",
      "--rm",
      "-v",
      `${Deno.cwd()}/${year}/${language}:/data:z`,
      ...(cwd ? ["-w", `/data/${cwd}`] : []),
      project.container,
      ...args,
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

  return { stdout: child.stdout, stderr: child.stderr };
};

export const build = async (
  project: Project,
  year: string,
  language: string,
  day: number,
  part: number,
) => {
  if (!project.build) {
    return true;
  }
  const { args, cwd } = project.build(day, part);
  const child = new Deno.Command("docker", {
    args: [
      "run",
      "-i",
      "--rm",
      "-v",
      `${Deno.cwd()}/${year}/${language}:/data:z`,
      ...(cwd ? ["-w", `/data/${cwd}`] : []),
      project.container,
      ...args,
    ],
    stderr: "piped",
  }).spawn();

  const { code } = await child.status;
  if (code !== 0) {
    console.error(new TextDecoder().decode((await child.output()).stderr));
  }
  return code === 0;
};

export const test = async (
  year: string,
  day: number,
  part: number,
  stdout: ReadableStream,
) => {
  const dayString = formatDay(day);
  const expected = await Deno.readTextFile(
    `${year}/data/${dayString}.${part}.out`,
  );

  const actual = new TextDecoder().decode(
    (await stdout.getReader().read()).value,
  );

  return { result: actual === expected, expected, actual };
};

export const fileExists = (path: string) => {
  try {
    Deno.statSync(path);
    return true;
  } catch {
    return false;
  }
};
