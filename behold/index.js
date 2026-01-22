/// <reference lib="dom" />

const contentView = document.querySelector("#contentView");
const controlView = document.querySelector("#controlView");
const state = { shift: false, currentEditable: undefined };

const TOP_LEVEL_KEYS = [
    ["spacer|1", "fn|6", "macro|6", "struct|6"],
    ["spacer|1", "global|6", "const|6", "enum|6"],
    ["spacer|1", "import|6", "include|6", "type|6"],
    ["spacer|1", "memory|6", "?exp|6", "...|6"],
];
const QWERTY_KEYS = [
    ["q", "w", "e", "r", "t", "y", "u", "i", "o", "p"],
    ["spacer|1", "a", "s", "d", "f", "g", "h", "j", "k", "l"],
    ["shift|3", "z", "x", "c", "v", "b", "n", "m", "backspace|3"],
    ["123|3", "space|14", "enter|3"],
];

showKeys(TOP_LEVEL_KEYS);

function showKeys(keys) {
    controlView.innerHTML = "";

    const keyboardView = newEl(`
        <div class="keyboard"></div>
    `);
    controlView.appendChild(keyboardView);

    for (const row of keys) {
        const rowView = newEl(`
            <div class="keyboard-row"></div>
        `);
        keyboardView.appendChild(rowView);

        for (const keyRaw of row) {
            const [key, span = "2"] = keyRaw.split("|");

            const keyView = newEl(`
                <div class="keyboard-key" style="grid-column: span ${span}">${key}</div>
            `);
            rowView.appendChild(keyView);

            switch (key) {
                case "fn":
                    keyView.onclick = () => {
                        const fnView = newEl(`
                            <div>
                                <span class="x-keyword">fn</span>
                                <span class="x-fn-name"></span>
                                <span class="x-delim">(</span>
                                <span class="x-delim">)</span>
                                <span class="x-delim">{</span>
                                <br/>
                                <span class="x-delim">}</span>
                            </div>
                        `);
                        contentView.appendChild(fnView);

                        state.currentEditable =
                            fnView.querySelector(".x-fn-name");
                        showKeys(QWERTY_KEYS);
                    };
                    break;

                case "shift":
                    keyView.innerHTML = "⇧";
                    keyView.onclick = () => {
                        setShift(!state.shift);
                    };
                    break;
                case "backspace":
                    keyView.innerHTML = "⌫";
                    keyView.onclick = () => {
                        state.currentEditable.innerHTML =
                            state.currentEditable.innerHTML.slice(0, -1);
                    };
                    break;
                case "123":
                    keyView.innerHTML = "123";
                    break;
                case "space":
                    keyView.innerHTML = "";
                    keyView.onclick = () => {
                        state.currentEditable.innerHTML += " ";
                    };
                    break;
                case "enter":
                    keyView.innerHTML = "↵";
                    keyView.onclick = () => {
                        showKeys(TOP_LEVEL_KEYS);
                    };
                    break;
                case "spacer":
                    keyView.style.cssText += " visibility: hidden;";
                    break;
                default:
                    keyView.className += " shiftable";
                    keyView.onclick = () => {
                        state.currentEditable.innerHTML += keyView.innerHTML;
                    };
                    break;
            }
        }
    }
}

function setShift(shift) {
    state.shift = shift;

    for (const key of document.querySelectorAll(".shiftable")) {
        if (shift) {
            key.innerHTML = key.innerHTML.toUpperCase();
        } else {
            key.innerHTML = key.innerHTML.toLowerCase();
        }
    }
}

function newEl(html) {
    const template = document.createElement("template");
    template.innerHTML = html.trim();
    const element = template.content.firstElementChild;
    return element;
}
