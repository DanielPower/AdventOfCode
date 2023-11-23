import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);

let waypoint = [10, 1];
const position = [0, 0];

for await (const line of readLines(fileReader)) {
  if (!line) {
    break;
  }
  const action = line.slice(0, 1);
  const value = parseInt(line.slice(1), 10);
  if (["N", "S"].includes(action)) {
    waypoint[1] += value * (action === "N" ? 1 : -1);
  } else if (["E", "W"].includes(action)) {
    waypoint[0] += value * (action === "E" ? 1 : -1);
  } else if (["L", "R"].includes(action)) {
    const theta = value * (Math.PI / 180) * (action === "R" ? -1 : 1);
    const newWaypointX = Math.round(
      waypoint[0] * Math.cos(theta) - waypoint[1] * Math.sin(theta)
    );
    const newWaypointY = Math.round(
      waypoint[1] * Math.cos(theta) + waypoint[0] * Math.sin(theta)
    );
    waypoint = [newWaypointX, newWaypointY];
  } else if (action === "F") {
    [0, 1].forEach((index) => {
      position[index] += waypoint[index] * value;
    });
  }
}

console.log(Math.abs(position[0]) + Math.abs(position[1]));
