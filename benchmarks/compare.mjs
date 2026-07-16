import fs from "node:fs";
import { performance } from "node:perf_hooks";
import MarkdownIt from "markdown-it";
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

> A quoted line for block parsing.

- first item
- second item

| Parser | Speed |
| :-- | --: |
| marked-rs | ~~pending~~ fast |
`;
const corpus = files.length === 0 ? sample : files.map((file) => fs.readFileSync(file, "utf8")).join("\n\n");
const source = corpus.repeat(repetitions);
const markdownIt = new MarkdownIt({ html: false });
const parsers = [
  ["Marked 18.0.6", (input) => marked.parse(input, { gfm: true })],
  ["markdown-it 14.1.0", (input) => markdownIt.render(input)],
];

console.log(`input: ${source.length} bytes`);
console.log(`iterations: ${iterations}`);
for (const [name, parse] of parsers) {
  for (let i = 0; i < 3; i += 1) parse(source);
  let outputLength = 0;
  const started = performance.now();
  for (let i = 0; i < iterations; i += 1) outputLength += parse(source).length;
  const milliseconds = performance.now() - started;
  const totalMiB = (source.length * iterations) / (1024 * 1024);
  console.log(`${name}: mean ${(milliseconds / iterations).toFixed(2)} ms, throughput ${(totalMiB / (milliseconds / 1000)).toFixed(2)} MiB/s, output ${outputLength} bytes`);
}
