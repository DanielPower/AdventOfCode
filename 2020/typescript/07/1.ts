import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const bagMap: { [name: string]: string[] } = {};

const getContents = (bag: string): string[] => {
  const subBags = bagMap[bag].map((subBag) => getContents(subBag)).flat();
  return [bag, ...subBags];
};

let shinyGoldCount = 0;
const decoder = new TextDecoder();
for await (const buffer of readline(Deno.stdin)) {
  const line = decoder.decode(buffer);
  if (!line) {
    break;
  }
  const [containingBag, contents] = line.split(" bags contain ");
  if (contents === "no other bags.") {
    bagMap[containingBag] = [];
  } else {
    bagMap[containingBag] = contents
      .split(", ")
      .map((bagWithQuantity) =>
        bagWithQuantity.split(" ").slice(1, -1).join(" "),
      );
  }
}

Object.keys(bagMap).forEach((bag) => {
  if (bag === "shiny gold") {
    return;
  }
  if (getContents(bag).includes("shiny gold")) {
    shinyGoldCount += 1;
  }
});

console.log(shinyGoldCount);
