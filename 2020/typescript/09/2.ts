import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const invalidNumber = 1124361034;
const values: number[] = [];
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
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
  contiguousValues[0] + contiguousValues[contiguousValues.length - 1],
);
