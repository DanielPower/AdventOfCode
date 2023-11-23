import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

let x = 0;
let trees = 0;
const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);
for await (const line of readLines(fileReader)) {
  if (line[x] === "#") {
    trees += 1;
  }
  x = (x + 3) % line.length;
}

console.log(trees);
