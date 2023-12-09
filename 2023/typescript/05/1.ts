let seeds: number[] = [];
// Due to a Bun bug, I don't have a good way of grabbing a single line for stdin
// https://github.com/oven-sh/bun/issues/7541
for await (const line of console) {
  seeds = line.split(": ")[1].split(" ").map(Number);
  break;
}

// Store the maps in a 3d array
const maps: [number, number, number][][] = [];
for await (const line of console) {
  if (!line) continue;
  if (line.includes(":")) {
    maps.push([]);
    continue;
  }
  const [destination, source, range] = line.split(" ").map(Number);
  maps[maps.length - 1].push([source, destination - source, range]);
}

// Sort the maps by source
maps.forEach((map) => map.sort((a, b) => b[0] - a[0]));

// Calculate the locations
const locations = seeds.map((seed) => {
  let currentValue = seed;
  maps.forEach((map) => {
    const result = map.find((values) => {
      return values[0] <= currentValue;
    })!;
    if (!result) return;
    const [source, modifier, range] = result;
    if (currentValue <= source + range) {
      currentValue = modifier + currentValue;
    }
  });
  return currentValue;
});

// Give the smallest location
console.log(Math.min(...locations));
