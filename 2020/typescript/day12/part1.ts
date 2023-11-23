import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

const directions = [
  [1, 0],
  [0, -1],
  [-1, 0],
  [0, 1],
];
let direction = 0;
const position = [0, 0];

for await (const line of readLines(fileReader)) {
  if (!line) {
    break;
  }
  const action = line.slice(0, 1);
  const value = parseInt(line.slice(1), 10);
  if (action === "N") {
    position[1] += value;
  } else if (action === "S") {
    position[1] -= value;
  } else if (action === "E") {
    position[0] += value;
  } else if (action === "W") {
    position[0] -= value;
  } else if (action === "L") {
    direction = (4 + direction - value / 90) % 4;
  } else if (action === "R") {
    direction = (4 + direction + value / 90) % 4;
  } else if (action === "F") {
    [0, 1].forEach((index) => {
      position[index] += directions[direction][index] * value;
    });
  }
}

console.log(Math.abs(position[0]) + Math.abs(position[1]));
