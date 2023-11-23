import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

const preambleSize = 25;
const shelf: number[] = [];
const shelfSize = 25;
let shelfCounter = 0;
let index = 0;
for await (const line of readLines(fileReader)) {
  if (!line) {
    break;
  }
  const value = parseInt(line, 10);
  if (index >= preambleSize) {
    if (
      !shelf.find((a, index) =>
        shelf.slice(index + 1).find((b) => a + b === value)
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
