import path from 'path';
import fs from 'fs';

const __dirname = new URL('.', import.meta.url).pathname.slice(0, -1);
const inputFilePath = path.join(__dirname, 'input.txt');

const convertAscii = a => {
  const ascii = a.charCodeAt(0);

  if (ascii >= 65 && ascii <= 90) {
    return ascii - 65 + 27;
  }

  return ascii - 97 + 1;
};

const findCommon = (...args) => {
  const table = new Array(26 * 2).fill(0);

  args.forEach(a => {
    const subTable = new Array(table.length).fill(0);

    a.forEach(b => {
      subTable[convertAscii(b) - 1] = subTable[convertAscii(b) - 1] + 1;
    });

    subTable.forEach((b, i) => {
      if (b > 0) {
        table[i] = table[i] + 1;
      }
    });
  });

  return table.findIndex(v => v === args.length) + 1;
};

const lines = fs
  .readFileSync(inputFilePath, 'utf-8')
  .trim()
  .split('\n')
  // For Part 1
  // .map(l => [l.substring(0, l.length / 2), l.substring(l.length / 2)])
  // .map(([a, b]) => [a.split(''), b.split('')])
  // .map(c => findCommon(...c))
  // End Part 1

  // For Part 2
  .reduce((acc, cur, idx) => {
    let c = [];

    if (idx % 3 === 0) {
      acc.push(c);
    } else {
      c = acc[acc.length - 1];
    }

    c.push(cur);

    return acc;
  }, [])
  .map(a => a.map(b => b.split('')))
  .map(i => findCommon(...i))
  // End Part 2
  .reduce((acc, v) => acc + v, 0);

console.log(lines);
