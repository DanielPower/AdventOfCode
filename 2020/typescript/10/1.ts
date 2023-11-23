import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const joltages: number[] = [];
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
  if (!line) {
    break;
  }
  joltages.push(parseInt(line, 10));
}

joltages.sort((a, b) => (a > b ? 1 : -1));
const differences: number[] = [joltages[0], 0, 1];
joltages.slice(1).forEach((jolts, index) => {
  differences[jolts - joltages[index] - 1] += 1;
});

console.log(differences[0] * differences[2]);
