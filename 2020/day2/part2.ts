import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

let validPasswords = 0;

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);
for await (const line of readLines(fileReader)) {
  const [match, first, second, char, password]: string[] =
    /^(\d+)-(\d+) (\w): (\w+)/.exec(line) || [];
  if (!match) {
    break;
  }
  const a = password[parseInt(first, 10) - 1];
  const b = password[parseInt(second, 10) - 1];
  console.log(char, a, b, password, (a === char) !== (b === char));
  if ((a === char) !== (b === char)) {
    validPasswords += 1;
  }
}

console.log(validPasswords);
