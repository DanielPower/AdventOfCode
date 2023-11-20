import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

const adjacentPositions = [
  [-1, -1],
  [-1, 0],
  [-1, 1],
  [0, -1],
  [0, 1],
  [1, -1],
  [1, 0],
  [1, 1],
];

const map: string[][] = [];
for await (const line of readLines(fileReader)) {
  if (!line) {
    break;
  }
  map.push(line.split(""));
}

while (true) {
  const oldMap: string[][] = [];
  map.forEach((row) => oldMap.push([...row]));
  map.forEach((row, y) => {
    row.forEach((seat, x) => {
      if (seat === ".") {
        return;
      }
      let adjacentsOccupied = 0;
      adjacentPositions.forEach(([dx, dy]) => {
        if (oldMap[y + dy]?.[x + dx] === "#") {
          adjacentsOccupied += 1;
        }
      });
      if (seat === "L" && adjacentsOccupied === 0) {
        map[y][x] = "#";
      } else if (seat === "#" && adjacentsOccupied >= 4) {
        map[y][x] = "L";
      }
    });
  });
  if (oldMap.toString() === map.toString()) {
    break;
  }
}

console.log(map.flat().filter((char) => char === "#").length);
