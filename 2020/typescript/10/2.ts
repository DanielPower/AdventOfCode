import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const joltages: number[] = [0];
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
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
