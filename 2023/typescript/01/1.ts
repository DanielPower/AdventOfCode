const numbers: number[] = [];
for await (const line of console) {
  if (!line) continue;
  let first_digit = line.match(/^[^\d]*(\d)/);
  let last_digit = line.match(/(\d)[^\d]*$/);
  numbers.push(parseInt(first_digit![1] + last_digit![1], 10));
}
console.log(numbers.reduce((a, b) => a + b, 0));
