import path from 'path';
import fs from 'fs';

const __dirname = new URL('.', import.meta.url).pathname.slice(0, -1);
const inputFilePath = path.join(__dirname, 'input.txt');

const lines = fs
  .readFileSync(inputFilePath, 'utf-8')
  .trim()
  .split('\n')
  .map(l => [l.substring(0, l.length / 2), l.substring(l.length / 2)])
  .map(([a, b]) => [a.split(''), b.split('')])
  .map(([a, b]) => {
    const i = a.find(v => b.includes(v));

    return i;
  })
  .map(c => {
    const ascii = c.charCodeAt(0);

    if (ascii >= 65 && ascii <= 90) {
      return ascii - 65 + 27;
    }

    return ascii - 97 + 1;
  })
  .reduce((acc, v) => acc + v, 0);

console.log(lines);
