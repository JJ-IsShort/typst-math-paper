class ElementBuilder {
  constructor(tag) {
    this.element = document.createElement(tag);
  }

  setId(id) {
    this.element.id = id;
    return this;
  }

  setClass(className) {
    this.element.className = className;
    return this;
  }

  setText(text) {
    this.element.textContent = text;
    return this;
  }

  setAttribute(name, value) {
    this.element.setAttribute(name, value);
    return this;
  }

  addEventListener(event, callback) {
    this.element.addEventListener(event, callback);
    return this;
  }

  appendChild(child) {
    if (child instanceof ElementBuilder) {
      this.element.appendChild(child.build());
    } else if (child instanceof HTMLElement) {
      this.element.appendChild(child);
    }
    return this;
  }

  build() {
    return this.element;
  }
}

async function load_WASM(display_element, import_path) {
  const module = await import(import_path);
  await module.default({
    canvas: display_element,
  });
}

var background = document.createElement("div");
background.className = "min-h-screen";
background.style = "background-color: rgb(165 243 252)";
var card_parent = document.createElement("div");
background.appendChild(card_parent);
card_parent.className =
  "flex flex-col min-h-screen justify-center items-center";
document.body.appendChild(background);
var format = [
  "page_1",
  "page_2",
  "page_3",
  "interactive_1",
  "page_4",
  "page_5",
];
(async () => {
  for (
    let format_section = 0;
    format_section < format.length;
    format_section++
  ) {
    const card_id = format[format_section];
    const format_type_index = card_id.split("_", 2);
    var display_card = document.createElement("div");
    display_card.className = "p-6";
    card_parent.appendChild(display_card);
    if (format_type_index[0] == "page") {
      const display_element = document.createElement("img");
      display_element.setAttribute("src", "./generated/" + card_id + ".svg");
      display_element.className = "shadow-xl rounded-3xl border-2 border-solid";
      display_element.style =
        "background-color: rgb(255 255 255); border-color: rgb(10 10 10);";
      display_card.appendChild(display_element);
    }
    if (format_type_index[0] == "interactive") {
      const display_element = document.createElement("canvas");
      display_element.setAttribute(
        "class",
        "emscripten shadow-xl rounded-md border-2 border-solid",
      );
      display_element.style = "border-color: rgb(10 10 10);";
      display_element.setAttribute("id", "canvas");
      display_element.setAttribute("oncontextmenu", "event.preventDefault()");
      display_card.appendChild(display_element);
      const import_path =
        "./generated/interactive_" + format_type_index[1] + "/interactive.js";
      await load_WASM(display_element, import_path);
    }
  }
})();
