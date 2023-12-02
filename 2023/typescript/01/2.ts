const numberMap = [
  ["1", "1"],
  ["2", "2"],
  ["3", "3"],
  ["4", "4"],
  ["5", "5"],
  ["6", "6"],
  ["7", "7"],
  ["8", "8"],
  ["9", "9"],
  ["0", "0"],
  ["one", "1"],
  ["two", "2"],
  ["three", "3"],
  ["four", "4"],
  ["five", "5"],
  ["six", "6"],
  ["seven", "7"],
  ["eight", "8"],
  ["nine", "9"],
];

const reversedNumberMap = numberMap.map(([key, value]) => [
  key.split("").reverse().join(""),
  value,
]);

const firstWith = (values: string[][], line: string): string => {
  if (!line) throw new Error("No match");
  const digit = values.find(([key]) => line.startsWith(key));
  if (digit) {
    return digit[1];
  }
  return firstWith(values, line.slice(1));
};

const numbers: number[] = [];
for await (const line of console) {
  if (!line) continue;
  const first_digit = firstWith(numberMap, line);
  const last_digit = firstWith(
    reversedNumberMap,
    line.split("").reverse().join(""),
  );
  numbers.push(parseInt(first_digit + last_digit, 10));
}
console.log(numbers.reduce((a, b) => a + b, 0));
