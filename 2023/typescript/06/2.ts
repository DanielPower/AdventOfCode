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

let time: string = "";
let distance: string = "";

for await (const line of console) {
  time = line.split(":")[1].replace(/\s+/g, "");
  break;
}

for await (const line of console) {
  distance = line.split(":")[1].replace(/\s+/g, "");
  break;
}

console.log(time, distance);
let product = 1;
const result = solveRace(parseInt(time), parseInt(distance));
product *= result;

console.log(product);
