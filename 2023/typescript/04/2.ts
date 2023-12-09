let cards: [string[], string[]][] = [];
let memo: Map<number, number> = new Map();

const scratchCard = (cardId: number) => {
  if (memo.has(cardId)) {
    return memo.get(cardId)!;
  }
  const [have, need] = cards[cardId];
  let points = 1;
  let subCards = 0;
  for (const i of have) {
    if (need.includes(i)) {
      subCards++;
    }
  }
  for (let i = subCards; i > 0; i--) {
    points += scratchCard(cardId + i);
  }
  memo.set(cardId, points);
  return points;
};

for await (const card of console) {
  if (!card) break;
  const [, table] = card.split(":");
  const [haveSection, needSection] = table.split("|");
  const have = haveSection.trim().split(/\s+/);
  const need = needSection.trim().split(/\s+/);
  cards.push([have, need]);
}

let totalPoints = 0;
for (const i of cards.keys()) {
  totalPoints += scratchCard(i);
}
console.log(totalPoints);
