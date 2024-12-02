import { Project } from "./types.ts";

export const fetchInput = async (year: string, day: string) => {
  const response = await fetch(
    `https://adventofcode.com/${year}/day/${parseInt(day)}/input`,
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
  await Deno.writeTextFile(`${year}/data/${day}.in`, input);
};

export const run = async (
  project: Project,
  year: string,
  language: string,
  day: string,
  part: string,
) => {
  let inputReadable;
  try {
    inputReadable = (await Deno.open(`${year}/data/${day}.in`)).readable;
  } catch {
    await fetchInput(year, day);
    inputReadable = (await Deno.open(`${year}/data/${day}.in`)).readable;
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
  day: string,
  part: string,
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
  day: string,
  part: string,
  stdout: ReadableStream,
) => {
  let expected;
  try {
    expected = await Deno.readTextFile(`${year}/data/${day}.${part}.out`);
  } catch {
    expected = null;
  }

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

export const getProjectResult = async (
  project: Project,
  year: string,
  language: string,
  day: string,
  part: string,
) => {
  try {
    if (!project.exists(`${year}/${language}`, day, part)) {
      return;
    }
    await build(project, year, language, day, part);
    const { stdout } = await run(project, year, language, day, part);
    const { result, actual, expected } = await test(year, day, part, stdout!);
    if (result) {
      console.log(`✅ ${year}: Day ${day} Part ${part}: Success`);
    } else {
      console.log(`❌ ${year}: Day ${day} Part ${part}: Failure`);
      console.log(`Expected: ${expected}`);
      console.log(`Recevied: ${actual}`);
    }
  } catch (e) {
    console.log(`❌ ${year}: Day ${day} Part ${part}: Failure`);
    console.log(`Unexpected error`);
    console.log(e);
  }
};
