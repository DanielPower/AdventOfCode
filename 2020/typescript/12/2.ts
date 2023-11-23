import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

let waypoint = [10, 1];
const position = [0, 0];

const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
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
      waypoint[0] * Math.cos(theta) - waypoint[1] * Math.sin(theta),
    );
    const newWaypointY = Math.round(
      waypoint[1] * Math.cos(theta) + waypoint[0] * Math.sin(theta),
    );
    waypoint = [newWaypointX, newWaypointY];
  } else if (action === "F") {
    [0, 1].forEach((index) => {
      position[index] += waypoint[index] * value;
    });
  }
}

console.log(Math.abs(position[0]) + Math.abs(position[1]));
