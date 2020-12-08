import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);
const bagMap: { [name: string]: string[] } = {};

const getContents = (bag: string): string[] => {
  const subBags = bagMap[bag].map((subBag) => getContents(subBag)).flat();
  return [bag, ...subBags];
};

let shinyGoldCount = 0;
for await (const line of readLines(fileReader)) {
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
        bagWithQuantity.split(" ").slice(1, -1).join(" ")
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
