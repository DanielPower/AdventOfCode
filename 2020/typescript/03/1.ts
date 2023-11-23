import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

let x = 0;
let trees = 0;
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
  if (line[x] === "#") {
    trees += 1;
  }
  x = (x + 3) % line.length;
}

console.log(trees);
