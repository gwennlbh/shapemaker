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

=  Introduction

== À la recherche d'une impossible énumération des formes

#figure(image("./alphabetdesformes.png", width: 80%), caption: "Un “alphabet” incomplet")

#figure(image("./alphabetdesformes.svg", width: 80%), caption: "Une vectorisation sur Adobe Illustrator")

== Une approche procédurale ?

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

== Excursion dans le monde physique

=== Interprétation collective

#link("https://shapemaker.gwen.works/soon.noredir")

== Lien musical

= Une _crate_ Rust avec un API sympathique

= Render loop et hooks

= Sources de synchronisation

== Temps réel: WASM et WebMIDI

Il est possible de réagir en temps réel à des pressions de touches sur des appareils conçus pour la production musicale assistée par ordinateur (MAO): des claviers, des potentiomères pour ajuster des réglages affectant le timbre d'un son, des pads pour déclencher des sons et, par exemple, jouer des percussions, etc.

Ces appareils sont appelés "contrôleurs MIDI", du protocole standard qui régit leur communication avec l'ordinateur.

S'il est évidemment possible d'interagit avec ces contrôleurs depuis un programme natif (c'est après tout ce que font les logiciels de production musicale), j'ai préféré tenté l'approche Web, pour en faciliter l'accessibilité et en réduire le temps nécéssaire à la mise en place #footnote[
  Imaginez, votre ordinateur a un problème 5 minutes avant le début d'une installation live, et vous aviez prévu d'utiliser Shapemaker pour des visuels. En faisant du dispostif un site web, il suffit de brancher son contrôleur à l'ordinateur d'un·e ami·e, et c'est tout bon.
].

Comme pour de nombreuses autres technologies existant à la frontière entre le matériel et le logiciel, les navigateurs mettent à disposition des sites web une technologie permettant de communiquer avec les périphériques MIDI connectés à la machine: c'est l'API WebMIDI @webmidi.

Mais bien évidemment, tout le code de Shapemaker, tout ses capacités de génération de formes, sont implémentées en Rust.

Il existe cependant un moyen de "faire tourner du code Rust" dans un navigateur Web: la compilation vers WebAssembly (WASM) @wasm.

En exportant la _crate_ shapemaker en bibliothèque Javascript via wasm-bindgen @wasmbindgen, il est donc possible d'exoser à une balise #raw("<script>", lang: "html") les fonctions de la bibliothèque, et brancher donc celles-ci à des _callbacks_ donnés par l'API WebMIDI:

#figure(caption: "Exposition de fonctions à WASM depuis Rust", [
```rust
#[wasm_bindgen]
pub fn render_image(opacity: f32, color: Color) -> Result<(), JsValue> {
    let mut canvas = /* ... */

    *WEB_CANVAS.lock().unwrap() = canvas;
    render_canvas_at(String::from("body"));

    Ok(())
}
```
])

#figure(caption: "Utilisation des fonctions exposées dans un script Javascript", [
```js
import init, { render_image } from "./shapemaker.js"

void init()

navigator.requestMIDIAccess().then((midiAccess) => {
  Array.from(midiAccess.inputs).forEach((input) => {
    input[1].onmidimessage = (msg) => {
      const [cmd, ...args] = [...msg.data]
      if (cmd !== 144) return

      // Touche enfoncée
      const [pitch, velocity] = args

      // get octave from pitch
      const octave = Math.floor(pitch / 12) - 1

      if (velocity === 0) {
        fadeOutElement(frameElement(color))
      } else {
        render_image(velocity / 128, octave)
      }
    }
  })
})
```
])

Au final, on peut arriver à une performance live interactive @pianowasmdemo intéréssante, et assez réactive pour ne pas avoir de latence (et donc de désynchronisation audio/vidéo) perceptible.

Les navigateurs Web supportant nativement le format SVG, qui se décrit notamment comme incluable directement dans le code HTML d'une page web @svginhtml, il est possible de simplement générer le code SVG, et de laisser le navigateur faire le rendu, ce qui s'avère être une solution très performante.

== Amplitudes de _stems_

== Export MIDI

== Fichier de projet

== Dépôt de "sondes" dans le logiciel de MAO

= Performance

= Conclusion


// Add bibliography and create Bibiliography section
#bibliography("bibliography.yaml")
