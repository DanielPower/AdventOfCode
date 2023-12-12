const raceOptions = (duration: number): number[] => {
  const options = [];
  for (let i = 1; i < duration; i++) {
    options.push(i * (duration - i));
  }
  return options;
};

const solveRace = (duration: number, record: number) => {
  const options = raceOptions(duration);
  const result = options.filter((option) => option > record);
  return result.length;
};

let times: string[] = [];
let distances: string[] = [];

for await (const line of console) {
  times = line.split(":")[1].trim().split(/\s+/);
  break;
}

for await (const line of console) {
  distances = line.split(":")[1].trim().split(/\s+/);
  break;
}

let product = 1;
for (let i = 0; i < times.length; i++) {
  const time = times[i];
  const distance = distances[i];
  const result = solveRace(parseInt(time), parseInt(distance));
  product *= result;
}

console.log(product);
