let instructions = "";
for await (const line of console) {
  if (!line) break;
  instructions = line;
}

let chart = new Map();
const pattern = /(\w+) = \((\w+), (\w+)\)/;
for await (const line of console) {
  if (!line) break;
  const match = pattern.exec(line);
  if (!match) throw new Error(`Invalid line: ${line}`);
  const node = match[1];
  const left = match[2];
  const right = match[3];
  chart.set(node, { left, right });
}

let steps = 0;
let position = "AAA";
while (position !== "ZZZ") {
  const { left, right } = chart.get(position);
  const direction = instructions[steps % instructions.length];
  if (direction === "L") {
    position = left;
  } else {
    position = right;
  }
  steps++;
}
console.log(steps);
