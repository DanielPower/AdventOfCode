import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

const chars = [];
for (let i = "a".charCodeAt(0); i <= "z".charCodeAt(0); i++) {
  chars.push(String.fromCharCode(i));
}

let count = 0;
let validChars = [...chars];
for await (const line of readLines(fileReader)) {
  if (line === null) {
    break;
  }
  if (line === "") {
    count += validChars.length;
    validChars = [...chars];
    continue;
  }
  const charsToRemove: string[] = [];
  validChars.forEach((char) => {
    if (!line.includes(char)) {
      charsToRemove.push(char);
    }
  });
  validChars = validChars.filter((char) => !charsToRemove.includes(char));
}

console.log(count);
