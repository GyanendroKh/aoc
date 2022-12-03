import path from 'path';
import fs from 'fs';

const __dirname = new URL('.', import.meta.url).pathname.slice(0, -1);
const inputFilePath = path.join(__dirname, 'input.txt');

const shape = ['X', 'Y', 'Z'];
const shapeMap = { A: 'X', B: 'Y', C: 'Z' };
const points = { X: 1, Y: 2, Z: 3 };

const matchRes = (a, b) => {
  const aPos = shape.indexOf(a);
  const [l, c, r] = [aPos - 1, aPos, (aPos + 1) % shape.length].map(i =>
    shape.at(i)
  );

  // Second part
  if (b === 'X') {
    return l;
  } else if (b === 'Y') {
    return c;
  } else if (b === 'Z') {
    return r;
  }

  if (b === l) {
    return 'L';
  } else if (b === r) {
    return 'W';
  } else if (b === c) {
    return 'D';
  }

  throw new Error('Invalid match');
};

const lines = fs.readFileSync(inputFilePath, 'utf8').trim().split('\n');

const scores = lines.map(l => {
  let [opp, you] = l.split(' ');
  opp = shapeMap[opp];

  const res = matchRes(opp, you);
  //                   you    for second part; you should be replaced with res for first part
  const score = points[res] + (you === 'X' ? 0 : you === 'Z' ? 6 : 3);

  return score;
});

console.log(
  'Total',
  scores.reduce((acc, curr) => acc + curr, 0)
);
