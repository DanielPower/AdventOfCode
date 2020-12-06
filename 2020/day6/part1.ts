import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

let count = 0;
let valid: Set<string> = new Set();
for await (const line of readLines(fileReader)) {
  if (line === null) {
    break;
  }
  if (line === "") {
    count += valid.size;
    valid = new Set();
  }
  line.split("").forEach((char) => valid.add(char));
}

console.log(count);
