// The project function defines how your document looks.
// It takes your content and some metadata and formats it.
// Go ahead and customize it to your liking!
#let project(title: "", authors: (), body) = {
  // Set the document's basic properties.
  set document(author: authors, title: title)
  // set page(numbering: "1", number-align: center)
  
  set page(width: 595.28pt, height: auto, margin: (top: 2.5cm, bottom: 2.5cm, left: 2.5cm, right: 2.5cm))

  set text(font: "Linux Libertine", size: 15pt, lang: "en")
  set par(justify: true, leading: 1em)

  body
}
