const input = require("fs").readFileSync(0, { encoding: "utf8" });

console.log(minify(input));

function minify(input) {
    while (true) {
        const trimmed = input
            .replace(/;.*\n/g, "")
            .replace(/\s+/g, " ")
            .replace(/([\(\{\[]) ([\(\{\[])/g, "$1$2")
            .replace(/([\(\{\[]) ([\)\}\]])/g, "$1$2")
            .replace(/([\)\}\]]) ([\)\}\]])/g, "$1$2")
            .replace(/([\)\}\]]) ([\(\{\[])/g, "$1$2");

        if (trimmed.length === input.length) break;
        input = trimmed;
    }

    return input
        .replace(/\(export /g, "\n(export ")
        .replace(/\(fn /g, "\n(fn ")
        .replace(/\(struct /g, "\n(struct ")
        .replace(/\(enum /g, "\n(enum ")
        .replace(/\(global /g, "\n(global ");
}
