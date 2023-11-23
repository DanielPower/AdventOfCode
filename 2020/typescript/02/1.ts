import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

let validPasswords = 0;

const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
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
