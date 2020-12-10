import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

const joltages: number[] = [];
for await (const line of readLines(fileReader)) {
  if (!line) {
    break;
  }
  joltages.push(parseInt(line, 10));
}

joltages.sort((a, b) => (a > b ? 1 : -1));
const differences: number[] = [joltages[0], 0, 1];
joltages.slice(1).forEach((jolts, index) => {
  console.log(index, jolts, joltages[index]);
  console.log(jolts - joltages[index]);
  differences[jolts - joltages[index] - 1] += 1;
});

console.log(differences[0] * differences[2]);
