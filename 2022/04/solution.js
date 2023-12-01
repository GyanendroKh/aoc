import path from 'path';
import fs from 'fs';

const __dirname = new URL('.', import.meta.url).pathname.slice(0, -1);
const inputFilePath = path.join(__dirname, 'input.txt');

const lines = fs
  .readFileSync(inputFilePath, 'utf-8')
  .trim()
  .split('\n')
  .map(l => l.split(','))
  .map(l => l.map(s => s.split('-').map(n => Number(n))))
  // Part 1
  // .filter(([[a, b], [c, d]]) => {
  //   return (a <= c && d <= b) || (c <= a && b <= d);
  // }); // End Part 1

  // Part 2
  .filter(([[a, b], [c, d]]) => {
    return (a >= c && a <= d) || (c >= a && c <= b);
  }); // End Part 2

console.log(lines.length);
