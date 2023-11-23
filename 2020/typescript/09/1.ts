import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const preambleSize = 25;
const shelf: number[] = [];
const shelfSize = 25;
let shelfCounter = 0;
let index = 0;
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
  if (!line) {
    break;
  }
  const value = parseInt(line, 10);
  if (index >= preambleSize) {
    if (
      !shelf.find((a, index) =>
        shelf.slice(index + 1).find((b) => a + b === value),
      )
    ) {
      console.log(line);
      break;
    }
  }
  shelf[shelfCounter] = value;
  shelfCounter = (shelfCounter + 1) % shelfSize;
  index += 1;
}
