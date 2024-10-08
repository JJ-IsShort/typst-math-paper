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

var card_parent = document.createElement("div");
document.body.appendChild(card_parent);
var format = ["page_1", "page_2", "interactive_1"];
(async () => {
  for (
    let format_section = 0;
    format_section < format.length;
    format_section++
  ) {
    const card_id = format[format_section];
    const format_type_index = card_id.split("_", 2);
    var display_card = document.createElement("div");
    card_parent.appendChild(display_card);
    if (format_type_index[0] == "page") {
      const display_element = document.createElement("img");
      display_element.setAttribute("src", "./generated/" + card_id + ".svg");
      display_card.appendChild(display_element);
    }
    if (format_type_index[0] == "interactive") {
      const display_element = document.createElement("canvas");
      display_element.setAttribute("class", "emscripten");
      display_element.setAttribute("id", "canvas");
      display_element.setAttribute("oncontextmenu", "event.preventDefault()");
      display_card.appendChild(display_element);
      const import_path =
        "./generated/interactive_" + format_type_index[1] + "/interactive.js";
      await load_WASM(display_element, import_path);
    }
  }
})();
