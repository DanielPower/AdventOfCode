import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

const invalidNumber = 1124361034;
const values: number[] = [];
for await (const line of readLines(fileReader)) {
  if (!line) {
    break;
  }
  values.push(parseInt(line, 10));
}

let minIndex = 0;
let maxIndex = 0;
let sum = 0;

while (sum !== invalidNumber) {
  sum += values[maxIndex];
  maxIndex += 1;

  while (sum > invalidNumber) {
    sum -= values[minIndex];
    minIndex += 1;
  }
}

const contiguousValues = values
  .slice(minIndex, maxIndex + 1)
  .sort((a, b) => (a > b ? 1 : -1));

console.log(
  contiguousValues[0] + contiguousValues[contiguousValues.length - 1]
);
