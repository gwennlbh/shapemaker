#import "template.typ": arkheion, arkheion-appendices

#show: arkheion.with(
  title: "Shapemaker: Créations audiovisuelles procédurales musicalement synchrones",
  authors: (
    (name: "Gwenn Le Bihan", email: "gwenn.lebihan@etu.inp-n7.fr", affiliation: "ENSEEIHT"),
  ),
  date: "22 Mars 2025",
  keywords: ("audiovisuel", "procédural", "SVG", "Rust", "WASM", "WebMIDI", "VST"),
)
#set cite(style: "chicago-author-date")
#show link: underline

#pad(y: 2em, 
  figure(
    image("./dna-analysis-machine.png", width: 75%)
  )
)


#raw(lang: "rust", 
  read("../src/examples.rs").replace("crate::*", "shapemaker::*")
)

#pagebreak()

#outline()

= 💭 Introduction

== 🔬 À la recherche d'une impossible énumération des formes

#figure(image("./alphabetdesformes.png", width: 80%), caption: "Un “alphabet” incomplet")

#figure(image("./alphabetdesformes.svg", width: 80%), caption: "Une vectorisation sur Adobe Illustrator")

== ✨ Une approche procédurale ?

#pad(4em, grid(
  columns: (1fr, 1fr, 1fr),
  ..( "designing-a-font", 
  "drone-operating-system", 
  "HAL-9000", 
  "japan-sledding-olympics", 
  "lunatic-green-energy", 
  // "measuring-spirits", 
  "phone-cameras", 
  "reflections", 
  "spline-optimisation", 
  "weaving").map(artwork => grid.cell(image("../examples/gallery/" + artwork + ".svg", width: 100%)) )
))

== 🌍 Excursion dans le monde physique

=== 👯 Interprétation collective

#link("https://shapemaker.gwen.works/soon.noredir")

== 🎵 Lien musical

= 💞 Une _crate_ Rust avec un API sympathique

= ➰ Render loop et hooks

= 🪡 Sources de synchronisation

== 🎹 Temps réel: WASM et WebMIDI

== 📈 Amplitudes de _stems_

== 📊 Export MIDI

== 📂 Fichier de projet

== 🦑 Dépôt de "sondes" dans le logiciel de MAO

= ⚡ Performance

= 📔 Conclusion


// Add bibliography and create Bibiliography section
#bibliography("bibliography.bib")
