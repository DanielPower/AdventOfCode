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

const countSteps = (startingPosition: string) => {
  let steps = 0;
  let position = startingPosition;
  while (position[2] !== "Z") {
    console.log(position);
    const { left, right } = chart.get(position);
    const direction = instructions[steps % instructions.length];
    position = direction === "L" ? left : right;
    steps += 1;
  }
  return steps;
};

const positions = Array.from(chart.keys()).filter((key) => key[2] === "A");
const steps = positions.map(countSteps);

const gcd = (a: number, b: number): number => {
  if (b === 0) return a;
  return gcd(b, a % b);
};

const lcm = (a: number, b: number): number => {
  return (a * b) / gcd(a, b);
};

console.log(steps.reduce(lcm));
