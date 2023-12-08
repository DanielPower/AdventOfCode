let sum = 0;
for await (let game of console) {
  if (!game) break;
  [, game] = game.split(": ");
  const quantities = {
    red: 0,
    green: 0,
    blue: 0,
  };
  const rounds = game.split("; ");
  for (const round of rounds) {
    const pulls = round.split(", ");
    for (const pull of pulls) {
      const [quantity, color] = pull.split(" ");
      quantities[color] = Math.max(quantities[color], quantity);
    }
  }
  sum += quantities.red * quantities.green * quantities.blue;
}
console.log(sum);
