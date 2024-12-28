import fetch from "node-fetch";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join } from "node:path";

const dir = join(import.meta.dirname, "../public/input");

async function getDay(y, d) {
  process.stdout.write(`20${y} day ${d}...`);

  // check if input is already cached
  let d0 = d.toString().padStart(2, "0");
  let pathname = join(dir, `20${y}/day${d0}.txt`);
  await mkdir(dirname(pathname), { recursive: true });

  try {
    await readFile(pathname, "utf-8");
    process.stdout.write("cached 📦\n");
    return;
  } catch (e) {}

  // file doesn't exist, fetch it
  try {
    process.stdout.write("fetching 🌐 ...");
    let res = await fetch(`https://adventofcode.com/20${y}/day/${d}/input`, {
      headers: { cookie: process.env.AOC_SESSION_COOKIE },
    });
    let text = await res.text();

    process.stdout.write("writing 📝 ...");
    await writeFile(pathname, text);

    process.stdout.write("done ✅\n");
    await new Promise((resolve) => setTimeout(resolve, 500)); // don't spam the server
  } catch (e) {
    process.stdout.write("failed ❌\n");
  }
}

async function getYear(y) {
  for (let d = 1; d <= 25; d++) {
    await getDay(y, d);
  }
}

if (process.argv.length === 3) {
  let y = parseInt(process.argv[2]);
  if (y >= 15 && y <= 24) getYear(y);
  else console.log("Year must be between 2015 and 2024");
} else console.log("Usage: getInput.js <year>");
