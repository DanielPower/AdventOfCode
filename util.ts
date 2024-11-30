import { Project } from "./types.ts";

export const formatDay = (day: number) => day.toString().padStart(2, "0");

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
    return {
      result: false,
      expected: null,
      actual: null,
      error: "Can't read input file",
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

  return { stdout: child.stdout, stderr: child.stderr };
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
