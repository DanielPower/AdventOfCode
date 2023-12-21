const diffs = (acc: number[][]): number[][] => {
  const numbers = acc.at(-1)!;
  if (numbers.every((n) => n === 0)) {
    return acc;
  }
  const result = [];
  for (let i = 0; i < numbers.length - 1; i++) {
    result.push(numbers[i + 1] - numbers[i]);
  }
  return diffs([...acc, result]);
};

const sum = (numbers: number[]): number =>
  numbers.reduce((acc, n) => acc + n, 0);

let total = 0;
for await (const line of console) {
  if (!line) break;
  const numbers = line.split(" ").reverse();
  const result = diffs([numbers.map((n) => parseInt(n))]);
  total += sum(result.map((numbers) => numbers.at(-1)!));
}
console.log(total);
