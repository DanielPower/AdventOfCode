const CARDS = ["J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A"];
const CARD_VALUES = Object.fromEntries(
  CARDS.map((card, index) => [card, index]),
);

const HANDS = {
  HIGH_CARD: 0,
  PAIR: 1,
  TWO_PAIRS: 2,
  THREE_OF_A_KIND: 3,
  FULL_HOUSE: 4,
  FOUR_OF_A_KIND: 5,
  FIVE_OF_A_KIND: 6,
};

const upgrade = (handType: number, upgrades: number): number => {
  if (upgrades === 0 || handType === HANDS.FIVE_OF_A_KIND) {
    return handType;
  }
  if (handType === HANDS.THREE_OF_A_KIND) {
    return upgrade(HANDS.FOUR_OF_A_KIND, upgrades - 1);
  }
  if (handType === HANDS.PAIR) {
    return upgrade(HANDS.THREE_OF_A_KIND, upgrades - 1);
  }
  if (handType === HANDS.TWO_PAIRS) {
    return upgrade(HANDS.FULL_HOUSE, upgrades - 1);
  }
  return upgrade(handType + 1, upgrades - 1);
};

const identifyHand = (hand: string) => {
  const map = new Map<string, number>();
  for (const card of hand) {
    map.set(card, (map.get(card) || 0) + 1);
  }
  const sortedByCount = Array.from(map.entries())
    .filter(([key]) => key !== "J")
    .sort((a, b) => b[1] - a[1]);
  const jCount = map.get("J") || 0;
  if (sortedByCount.length < 2) {
    return upgrade(HANDS.FIVE_OF_A_KIND, jCount);
  }
  const [a, b] = sortedByCount.map(([, count]) => count);
  if (a === 5) return upgrade(HANDS.FIVE_OF_A_KIND, jCount);
  if (a === 4) return upgrade(HANDS.FOUR_OF_A_KIND, jCount);
  if (a === 3) {
    if (b === 2) return upgrade(HANDS.FULL_HOUSE, jCount);
    return upgrade(HANDS.THREE_OF_A_KIND, jCount);
  }
  if (a === 2) {
    if (b === 2) return upgrade(HANDS.TWO_PAIRS, jCount);
    return upgrade(HANDS.PAIR, jCount);
  }
  return upgrade(HANDS.HIGH_CARD, jCount);
};

const kickerValue = (hand: string): number =>
  Array.from(hand).reduce(
    (acc, card, index) => acc + CARDS.length ** (4 - index) * CARD_VALUES[card],
    0,
  );

const handValue = (hand: string): number => {
  const categorySize = CARDS.length ** 5 + CARDS.length + CARD_VALUES.A * 5;
  return categorySize * identifyHand(hand) + kickerValue(hand);
};

const hands: [number, number][] = [];
for await (const line of console) {
  if (!line) break;
  const [hand, bid] = line.split(" ");
  hands.push([handValue(hand), parseInt(bid, 10)]);
}

const sortedHands = hands.sort((a, b) => a[0] - b[0]);
const totalWinnings = sortedHands.reduce((acc, [_, bid], index) => {
  const handValue = (index + 1) * bid;
  return acc + handValue;
}, 0);
console.log(totalWinnings);
