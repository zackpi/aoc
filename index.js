import init, * as aoc from "./pkg/aoc.js";
await init();

async function day(y, d, override) {
  let y0 = y.toString().padStart(2, "0");
  let d0 = d.toString().padStart(2, "0");
  let path = override || `/input/20${y0}/day${d0}.txt`;
  let input = await (await fetch(path)).text();
  return aoc[`y${y0}d${d0}`](input);
}

// for (let d = 1; d <= 25; d++) {
//   await day(15, d);
// }

let d = 15;
// await day(15, d);
await day(15, d, "/input/small.txt");
