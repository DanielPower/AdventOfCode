import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const values: number[] = [];

const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
  values.push(parseInt(line, 10));
}

values.forEach((a, aIndex) => {
  values.slice(aIndex + 1).forEach((b, bIndex) => {
    values.slice(aIndex + bIndex + 1).forEach((c) => {
      if (a + b + c === 2020) {
        console.log(a * b * c);
        return;
      }
    });
  });
});
