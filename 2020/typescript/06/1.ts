import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

let count = 0;
let valid: Set<string> = new Set();
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
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
