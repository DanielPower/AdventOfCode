const reNumber = new RegExp(/\d+/g);
const reStar = new RegExp(/\*/g);

const checkRow = (
  previousRow: string,
  checkingRow: string,
  nextRow: string,
): number => {
  let starMatch: RegExpExecArray;
  const gearNumbers: number[] = [];
  while ((starMatch = reStar.exec(checkingRow)!)) {
    const adjacentNumbers: number[] = [];
    [previousRow, checkingRow, nextRow].forEach((row) => {
      let numberMatch;
      while ((numberMatch = reNumber.exec(row))) {
        if (
          starMatch.index >= numberMatch.index - 1 &&
          starMatch.index <= numberMatch.index + numberMatch[0].length
        ) {
          adjacentNumbers.push(Number(numberMatch[0]));
        }
      }
    });
    if (adjacentNumbers.length === 2) {
      gearNumbers.push(adjacentNumbers[0] * adjacentNumbers[1]);
    }
  }
  return gearNumbers.reduce((a, b) => a + b, 0);
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
