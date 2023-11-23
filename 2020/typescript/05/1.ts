import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const bsp = (min: number, max: number, high: boolean): [number, number] => {
  if (high) {
    return [min + (max - min + 1) / 2, max];
  }
  return [min, max - (max - min + 1) / 2];
};

let maxSeatId = 0;
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
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
  maxSeatId = Math.max(maxSeatId, minRow * 8 + minCol);
}

console.log(maxSeatId);
