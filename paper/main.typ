#import "template.typ": arkheion, arkheion-appendices

#show: arkheion.with(
  title: "Shapemaker: Créations audiovisuelles procédurales musicalement synchrones",
  authors: (
    (name: "Gwenn Le Bihan", email: "gwenn.lebihan@etu.inp-n7.fr", affiliation: "ENSEEIHT"),
  ),
  date: "22 Mars 2025",
)
#set cite(style: "chicago-author-date")
#show link: underline

#pad(y: 2em, 
  figure(
    image("./dna-analysis-machine.png", width: 75%)
  )
)

#columns(1, text(size: 0.75em, raw(read("../src/examples.rs").replace("use crate::*", "use shapemaker::*"), lang: "rust")))

#pagebreak()



// Add bibliography and create Bibiliography section
#bibliography("bibliography.bib")
