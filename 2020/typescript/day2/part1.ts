import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

let validPasswords = 0;

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);
for await (const line of readLines(fileReader)) {
  const [match, min, max, char, password]: string[] =
    /^(\d+)-(\d+) (\w): (\w+)/.exec(line) || [];
  if (!match) {
    break;
  }
  const count = (password.match(RegExp(`${char}`, "g")) || []).length;
  if (count >= parseInt(min, 10) && count <= parseInt(max, 10)) {
    validPasswords += 1;
  }
}

console.log(validPasswords);
