import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

let validPasswords = 0;

const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
  const [match, first, second, char, password]: string[] =
    /^(\d+)-(\d+) (\w): (\w+)/.exec(line) || [];
  if (!match) {
    break;
  }
  const a = password[parseInt(first, 10) - 1];
  const b = password[parseInt(second, 10) - 1];
  if ((a === char) !== (b === char)) {
    validPasswords += 1;
  }
}

console.log(validPasswords);
