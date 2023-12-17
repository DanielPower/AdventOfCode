const CARDS = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"];
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

const identifyHand = (hand: string): number => {
  const map = new Map<string, number>();
  for (const card of hand) {
    map.set(card, (map.get(card) || 0) + 1);
  }
  const sortedByCount = Array.from(map.entries()).sort((a, b) => b[1] - a[1]);
  if (sortedByCount[0][1] === 5) return HANDS.FIVE_OF_A_KIND;
  if (sortedByCount[0][1] === 4) return HANDS.FOUR_OF_A_KIND;
  if (sortedByCount[0][1] === 3) {
    if (sortedByCount[1][1] === 2) return HANDS.FULL_HOUSE;
    return HANDS.THREE_OF_A_KIND;
  }
  if (sortedByCount[0][1] === 2) {
    if (sortedByCount[1][1] === 2) return HANDS.TWO_PAIRS;
    return HANDS.PAIR;
  }
  return HANDS.HIGH_CARD;
};

const handValue = (hand: string): number => {
  const kicker = Array.from(hand).reduce(
    (acc, card, index) => acc + CARDS.length ** (4 - index) * CARD_VALUES[card],
    0,
  );
  const categorySize = CARDS.length ** 5 + CARDS.length + CARD_VALUES.A * 5;
  return categorySize * identifyHand(hand) + kicker;
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
