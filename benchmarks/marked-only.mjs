import fs from "node:fs";
import { performance } from "node:perf_hooks";
import { marked } from "marked";

const [iterationsText = "20", repetitionsText = "5000", ...files] = process.argv.slice(2);
const iterations = Number(iterationsText);
const repetitions = Number(repetitionsText);
if (!Number.isInteger(iterations) || !Number.isInteger(repetitions) || iterations < 1 || repetitions < 1) {
  throw new Error("iterations and repetitions must be positive integers");
}

const sample = `# Benchmark document

This is a **small** paragraph with *emphasis*, [a link](https://example.com),
an ![image](image.png), and \`inline code\`.
`;
const corpus = files.length === 0 ? sample : files.map((file) => fs.readFileSync(file, "utf8")).join("\n\n");
const source = corpus.repeat(repetitions);

for (let i = 0; i < 3; i += 1) marked.parse(source, { gfm: true });
const started = performance.now();
let outputLength = 0;
for (let i = 0; i < iterations; i += 1) outputLength += marked.parse(source, { gfm: true }).length;
if (outputLength === 0) throw new Error("unexpected empty benchmark output");
console.log(`mean_ms=${(performance.now() - started) / iterations}`);
