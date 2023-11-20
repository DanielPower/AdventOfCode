import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const slopes = [
  { x: 0, vx: 1, vy: 1, trees: 0 },
  { x: 0, vx: 3, vy: 1, trees: 0 },
  { x: 0, vx: 5, vy: 1, trees: 0 },
  { x: 0, vx: 7, vy: 1, trees: 0 },
  { x: 0, vx: 1, vy: 2, trees: 0 },
];

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);
let y = 0;
for await (const line of readLines(fileReader)) {
  slopes.forEach((slope) => {
    if (y % slope.vy === 0) {
      if (line[slope.x] === "#") {
        slope.trees += 1;
      }
      slope.x = (slope.x + slope.vx) % line.length;
    }
  });
  y += 1;
}
let product = slopes[0].trees;
slopes.slice(1).forEach((slope) => {
  product *= slope.trees;
});
console.log(product);
