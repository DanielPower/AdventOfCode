import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

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
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
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
