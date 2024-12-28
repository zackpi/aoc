import init, * as aoc from "./pkg/aoc.js";
await init();

async function day(y, d, override) {
  let y0 = y.toString().padStart(2, "0");
  let d0 = d.toString().padStart(2, "0");
  let path = override || `/input/20${y0}/day${d0}.txt`;
  let input = await (await fetch(path)).text();
  return aoc[`y${y0}d${d0}`](input);
}

// await day(15, 1);
// await day(15, 2);
// await day(15, 3);
// await day(15, 4);
// await day(15, 5);
// await day(15, 6);
// await day(15, 7);
await day(15, 8, "/input/small.txt");
