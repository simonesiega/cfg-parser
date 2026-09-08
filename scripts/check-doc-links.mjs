#!/usr/bin/env node

/** Validate local paths and heading anchors referenced by Markdown files. */

import { existsSync, readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, extname, isAbsolute, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const skippedDirectories = new Set([".git", "target"]);
const externalPrefixes = ["http://", "https://", "mailto:", "tel:", "data:"];

function findMarkdownFiles(directory) {
  const files = [];
  for (const entry of readdirSync(directory, { withFileTypes: true })) {
    if (entry.isDirectory() && skippedDirectories.has(entry.name)) continue;
    const fullPath = resolve(directory, entry.name);
    if (entry.isDirectory()) files.push(...findMarkdownFiles(fullPath));
    else if (entry.isFile() && extname(entry.name).toLowerCase() === ".md")
      files.push(fullPath);
  }
  return files.sort();
}

function withoutFencedCode(text) {
  const output = [];
  let fence = null;
  for (const line of text.split(/\r?\n/u)) {
    const match = line.match(/^\s*(```|~~~)/u);
    if (match && fence === null) {
      fence = match[1];
      continue;
    }
    if (match && match[1] === fence) {
      fence = null;
      continue;
    }
    if (fence === null) output.push(line);
  }
  return output.join("\n");
}

function githubSlug(heading) {
  return heading
    .replace(/<[^>]+>/gu, "")
    .replace(/!\[([^\]]*)\]\([^)]*\)/gu, "$1")
    .replace(/\[([^\]]+)\]\([^)]*\)/gu, "$1")
    .replaceAll("`", "")
    .toLowerCase()
    .trim()
    .replace(/[^\p{L}\p{N}_\- ]/gu, "")
    .replace(/\s+/gu, "-");
}

function headingAnchors(file) {
  const text = withoutFencedCode(readFileSync(file, "utf8"));
  const anchors = new Set();
  const occurrences = new Map();
  for (const match of text.matchAll(/^#{1,6}\s+(.+?)\s*#*\s*$/gmu)) {
    const base = githubSlug(match[1]);
    const count = occurrences.get(base) ?? 0;
    anchors.add(count === 0 ? base : `${base}-${count}`);
    occurrences.set(base, count + 1);
  }
  return anchors;
}

function referencedLinks(file) {
  const text = withoutFencedCode(readFileSync(file, "utf8"));
  const links = new Set();
  const markdownPattern =
    /!?\[[^\]]*\]\((<[^>]+>|[^\s)]+)(?:\s+["'][^"']*["'])?\)/gu;
  const htmlPattern = /(?:href|src)\s*=\s*["']([^"']+)["']/giu;
  for (const match of text.matchAll(markdownPattern)) links.add(match[1]);
  for (const match of text.matchAll(htmlPattern)) links.add(match[1]);
  return [...links].sort();
}

function displayPath(file) {
  return relative(root, file).replaceAll("\\", "/");
}

const files = findMarkdownFiles(root);
const anchorsByFile = new Map(
  files.map((file) => [file, headingAnchors(file)]),
);
const failures = [];
let checked = 0;

for (const source of files) {
  for (const rawLink of referencedLinks(source)) {
    let link;
    try {
      link = decodeURIComponent(rawLink.replace(/^<|>$/gu, ""));
    } catch {
      failures.push(`${displayPath(source)}: invalid URL encoding: ${rawLink}`);
      continue;
    }

    if (!link || externalPrefixes.some((prefix) => link.startsWith(prefix)))
      continue;
    const hashIndex = link.indexOf("#");
    const pathPart = hashIndex === -1 ? link : link.slice(0, hashIndex);
    const anchor = hashIndex === -1 ? "" : link.slice(hashIndex + 1);
    if (pathPart.startsWith("/")) continue;

    const target = pathPart ? resolve(dirname(source), pathPart) : source;
    checked += 1;

    const relativeTarget = relative(root, target);
    if (relativeTarget.startsWith("..") || isAbsolute(relativeTarget)) {
      failures.push(
        `${displayPath(source)}: link escapes repository: ${rawLink}`,
      );
      continue;
    }
    if (!existsSync(target)) {
      failures.push(`${displayPath(source)}: missing target: ${rawLink}`);
      continue;
    }
    if (
      anchor &&
      statSync(target).isFile() &&
      extname(target).toLowerCase() === ".md"
    ) {
      const anchors = anchorsByFile.get(target) ?? headingAnchors(target);
      anchorsByFile.set(target, anchors);
      if (!anchors.has(anchor)) {
        failures.push(
          `${displayPath(source)}: missing anchor #${anchor} in ${displayPath(target)}`,
        );
      }
    }
  }
}

if (failures.length > 0) {
  console.error("Documentation link validation failed:");
  for (const failure of failures) console.error(`  - ${failure}`);
  process.exitCode = 1;
} else {
  console.log(
    `Validated ${checked} local links across ${files.length} Markdown files.`,
  );
}
