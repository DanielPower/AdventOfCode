import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const values: number[] = [];

const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
  values.push(parseInt(line, 10));
}

values.forEach((a, index) => {
  values.slice(index + 1).forEach((b) => {
    if (a + b === 2020) {
      console.log(a * b);
      return;
    }
  });
});
