import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const chars = [];
for (let i = "a".charCodeAt(0); i <= "z".charCodeAt(0); i++) {
  chars.push(String.fromCharCode(i));
}

let count = 0;
let validChars = [...chars];
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
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
