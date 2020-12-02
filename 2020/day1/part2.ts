import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const values: number[] = [];
const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

for await (const line of readLines(fileReader)) {
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
