import { readline } from "https://deno.land/x/readline@v1.1.0/mod.ts";

const bagMap: { [name: string]: [string, number][] } = {};

const getContents = (bag: string): number => {
  let count = 1;
  bagMap[bag].forEach(([subBagName, subBagQuantity]) => {
    count += getContents(subBagName) * subBagQuantity;
  });
  return count;
};

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
    bagMap[containingBag] = contents.split(", ").map((bag) => {
      const [quantity, ...name] = bag.split(" ").slice(0, -1);
      return [name.join(" "), parseInt(quantity, 10)];
    });
  }
}

console.log(getContents("shiny gold") - 1);
