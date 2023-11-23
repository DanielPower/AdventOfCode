import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const slopes = [
  { x: 0, vx: 1, vy: 1, trees: 0 },
  { x: 0, vx: 3, vy: 1, trees: 0 },
  { x: 0, vx: 5, vy: 1, trees: 0 },
  { x: 0, vx: 7, vy: 1, trees: 0 },
  { x: 0, vx: 1, vy: 2, trees: 0 },
];

let y = 0;
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
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
