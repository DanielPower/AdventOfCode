import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

const joltages: number[] = [0];
for await (const line of readLines(fileReader)) {
  if (!line) {
    break;
  }
  joltages.push(parseInt(line, 10));
}
joltages.sort((a, b) => (a > b ? 1 : -1));
const memoize: { [key: number]: number } = {};
const countArrangements = (index: number, depth = 0): number => {
  if (memoize[index]) {
    return memoize[index];
  }
  if (index === 0) {
    return 1;
  }
  let arrangements = 0;
  for (let i = 1; i <= 3; i++) {
    if (index - i < 0) {
      break;
    }
    if (joltages[index] - joltages[index - i] <= 3) {
      arrangements += countArrangements(index - i, depth + 1);
    }
  }
  memoize[index] = arrangements;
  return arrangements;
};
console.log(countArrangements(joltages.length - 1));
