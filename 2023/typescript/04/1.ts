let totalPoints = 0;
for await (const card of console) {
  if (!card) break;
  const [, table] = card.split(":");
  const [haveSection, needSection] = table.split("|");
  const have = haveSection.trim().split(/\s+/);
  const need = needSection.trim().split(/\s+/);
  let haveOfNeed = 0;
  for (const i of have) {
    if (need.includes(i)) {
      haveOfNeed++;
    }
  }
  totalPoints += haveOfNeed ? 2 ** (haveOfNeed - 1) : 0;
}
console.log(totalPoints);
