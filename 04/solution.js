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
  .filter(([[a, b], [c, d]]) => {
    if (a <= c) {
      if (d <= b) {
        return true;
      }
    }

    if (c <= a) {
      if (b <= d) {
        return true;
      }
    }

    return false;
  });

console.log(lines.length);
