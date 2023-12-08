const MAX_QUANTITIES = {
  red: 12,
  green: 13,
  blue: 14,
};

const validGames = [];
let gameId = 0;
outer: for await (let game of console) {
  gameId += 1;
  if (!game) break;
  [, game] = game.split(": ");
  console.error(`Game #${gameId}: ${game}`);
  const rounds = game.split("; ");
  for (const round of rounds) {
    const pulls = round.split(", ");
    for (const pull of pulls) {
      const [quantity, color] = pull.split(" ");
      if (quantity > MAX_QUANTITIES[color]) {
        console.error(`Game #${gameId}: invalid pull`);
        continue outer;
      }
    }
  }
  validGames.push(gameId);
}
console.log(validGames.reduce((a, b) => a + b, 0));
