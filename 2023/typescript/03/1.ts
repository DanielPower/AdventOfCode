const reSymbol = new RegExp(/[^\d.]/);
const reNumber = new RegExp(/\d+/g);

const checkRow = (
  previousRow: string,
  checkingRow: string,
  nextRow: string,
): number => {
  let match;
  const partNumbers: number[] = [];
  while ((match = reNumber.exec(checkingRow))) {
    const left = Math.max(match.index - 1, 0);
    const right = Math.min(
      match.index + match[0].length,
      checkingRow.length - 1,
    );
    const neighbours = [
      checkingRow[left],
      checkingRow[right],
      ...previousRow.slice(left, right + 1),
      ...nextRow.slice(left, right + 1),
    ];
    for (const neighbour of neighbours) {
      if (reSymbol.test(neighbour)) {
        partNumbers.push(Number(match[0]));
      }
    }
  }
  return partNumbers.reduce((a, b) => a + b, 0);
};

let sum = 0;
let checkingRow = "";
let previousRow = "";
for await (const line of console) {
  checkingRow = line;
  previousRow = ".".repeat(checkingRow.length);
  break;
}
for await (const nextRow of console) {
  sum += checkRow(previousRow, checkingRow, nextRow);
  previousRow = checkingRow;
  checkingRow = nextRow;
}
sum += checkRow(previousRow, checkingRow, ".".repeat(checkingRow.length));
console.log(sum);
