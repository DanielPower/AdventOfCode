import {readLines} from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const values: number[] = [];
const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

for await (let line of readLines(fileReader)) {
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
