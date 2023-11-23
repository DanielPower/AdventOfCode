import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

const bsp = (min: number, max: number, high: boolean): [number, number] => {
  if (high) {
    return [min + (max - min + 1) / 2, max];
  }
  return [min, max - (max - min + 1) / 2];
};

const seatIds = new Set();
for await (const line of readLines(fileReader)) {
  if (!line) {
    break;
  }
  const rowString = line.slice(0, 7);
  let minRow = 0;
  let maxRow = 127;
  rowString.split("").forEach((rowChar) => {
    [minRow, maxRow] = bsp(minRow, maxRow, rowChar === "B");
  });
  const colString = line.slice(7);
  let minCol = 0;
  let maxCol = 7;
  colString.split("").forEach((colChar) => {
    [minCol, maxCol] = bsp(minCol, maxCol, colChar === "R");
  });
  seatIds.add(minRow * 8 + minCol);
}

let firstSeatFound = false;
for (let i = 0; i < 1024; i++) {
  if (seatIds.has(i)) {
    firstSeatFound = true;
  } else if (firstSeatFound) {
    console.log(i);
    break;
  }
}
