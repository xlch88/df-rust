#!/usr/bin/env node
const df = require("./index.js");
printTable(df());
function printTable(objs, header = true) {
	const keys = Object.keys(objs[0] ?? {});
	const colWidths = keys.map((k) => Math.max(k.length, ...objs.map((o) => String(o[k] ?? "").length)));
	if (header) {
		console.log(
			keys
				.map((k, i) => k.padEnd(colWidths[i]))
				.join(" ")
				.trim()
		);
	}
	for (const obj of objs) {
		const output = keys
			.map((k, i) => String(obj[k] ?? "").padEnd(colWidths[i]))
			.join(" ")
			.trim();
		console.log(output.length > process.stdout.columns ? output.slice(0, process.stdout.columns - 3) + "..." : output);
	}
}
