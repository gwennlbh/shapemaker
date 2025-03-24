// From https://github.com/mgoulao/arkheion, slightly tweaked parce que le Français.
#let arkheion(
  title: "",
  abstract: none,
  keywords: (),
  authors: (),
  custom_authors: none,
  date: none,
  logo: none,
  body,
) = {
  // Set the document's basic properties.
  set document(author: authors.map(a => a.name), title: title)
  set page(
    margin: (left: 25mm, right: 25mm, top: 25mm, bottom: 30mm),
    numbering: "1",
    number-align: center,
  )
  show raw: set text(size: 0.85em, font: "Martian Mono", weight: "bold")
  set text(font: "New Computer Modern", lang: "fr")
  show math.equation: set text(weight: 400)
  show math.equation: set block(spacing: 0.65em)
  set math.equation(numbering: "(1)")
  set heading(numbering: "1.1 ")
  // Écriture inclusive >:3
  show "·": sym.dot.op
  // show heading: set text(font: "Martian Mono")

  // Set run-in subheadings, starting at level 4.
  show heading: it => {
    // H1 and H2
    if it.level == 1 {
      pad(
        bottom: 10pt,
        it,
      )
    } else if it.level == 2 {
      pad(
        bottom: 8pt,
        it,
      )
    } else if it.level > 3 {
      text(11pt, weight: "bold", it.body + " ")
    } else {
      it
    }
  }

  pad(
    x: 0%,
    y: 30%,
    {
      if logo != none {
        pad(
          top: 1em,
          align(center)[
            #image(logo, width: 80%)
          ],
        )
      }

      if logo == none {
        line(length: 100%, stroke: 2pt)
      }
      // Title row.
      pad(
        bottom: 4pt,
        top: 4pt,
        align(center)[
          #block(text(weight: 500, 1.75em, title))
          #v(1em, weak: true)
        ],
      )
      if logo == none {
        line(length: 100%, stroke: 2pt)
      }

      // Author information.
      if custom_authors != none {
        custom_authors
      } else {
        pad(
          top: 0.5em,
          x: 2em,
          grid(
            columns: (1fr,) * calc.min(3, authors.len()),
            gutter: 1em,
            ..authors.map(author => align(center)[
              #if author.keys().contains("orcid") {
                link("http://orcid.org/" + author.orcid)[
                  #pad(
                    bottom: -8pt,
                    grid(
                      columns: (8pt, auto, 8pt),
                      rows: 10pt,
                      [],
                      [*#author.name*],
                      [
                        #pad(left: 4pt, top: -4pt, image("orcid.svg", width: 8pt))
                      ],
                    ),
                  )
                ]
              } else {
                grid(
                  columns: auto,
                  rows: 2pt,
                  [*#author.name*],
                )
              }
              #author.email \
              #author.affiliation
            ]),
          ),
        )
      }

      align(center)[#date]

      // Abstract.
      if abstract != none {
        pad(
          x: 3em,
          top: 1em,
          bottom: 0.4em,
          align(center)[
            #heading(
              outlined: false,
              numbering: none,
              text(0.85em, smallcaps[Introduction]),
            )
            #set par(justify: true)
            #set text(hyphenate: false)

            #abstract
          ],
        )
      }

      // Keywords
      if keywords.len() > 0 {
        [*_Mots clés_* #h(0.3cm)] + keywords.map(str).join(" · ")
      }
    },
  )

  // Main body.
  set par(justify: true)
  set text(hyphenate: false)

  body
}

#let monospace = body => {
  text(font: "Martian Mono", size: 0.7em, body)
}

#let arkheion-appendices(body) = {
  counter(heading).update(0)
  counter("appendices").update(1)

  set heading(
    numbering: (..nums) => {
      let vals = nums.pos()
      let value = "ABCDEFGHIJ".at(vals.at(0) - 1)
      if vals.len() == 1 {
        return value
      } else {
        return value + "." + nums.pos().slice(1).map(str).join(".")
      }
    },
  )
  [#pagebreak() #body]
}
