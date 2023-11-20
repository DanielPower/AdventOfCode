import { readLines } from "https://deno.land/std/io/mod.ts";
import * as path from "https://deno.land/std/path/mod.ts";

const filename = path.join(Deno.cwd(), "input.txt");
const fileReader = await Deno.open(filename);
const bagMap: { [name: string]: [string, number][] } = {};

const getContents = (bag: string): number => {
  let count = 1;
  console.log(bag);
  bagMap[bag].forEach(([subBagName, subBagQuantity]) => {
    count += getContents(subBagName) * subBagQuantity;
  });
  return count;
};

for await (const line of readLines(fileReader)) {
  if (!line) {
    break;
  }
  const [containingBag, contents] = line.split(" bags contain ");
  if (contents === "no other bags.") {
    bagMap[containingBag] = [];
  } else {
    bagMap[containingBag] = contents.split(", ").map((bag) => {
      const [quantity, ...name] = bag.split(" ").slice(0, -1);
      return [name.join(" "), parseInt(quantity, 10)];
    });
  }
}

console.log(getContents("shiny gold") - 1);
