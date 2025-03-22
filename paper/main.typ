#import "template.typ": arkheion, arkheion-appendices

#show: arkheion.with(
  title: "Shapemaker: Créations audiovisuelles procédurales musicalement synchrones",
  authors: (
    (name: "Gwenn Le Bihan", email: "gwenn.lebihan@etu.inp-n7.fr", affiliation: "ENSEEIHT"),
  ),
  date: "22 Mars 2025",
  keywords: ("audiovisuel", "procédural", "SVG", "Rust", "WASM", "WebMIDI", "VST"),
)

#let citeauthor(label) = cite(label, style: "chicago-author-date")
#let imagefigure(path, caption) = figure(image(path, width: 100%), caption: caption)

#show link: underline

#align(center, pad(y: 2em, image("./dna-analysis-machine.png", width: 75%)))


#raw(lang: "rust", 
  read("../src/examples.rs").replace("crate::*", "shapemaker::*")
)

#pagebreak()

#outline()

=  Introduction

== À la recherche d'une impossible énumération des formes



#grid(
  columns: (1fr, 1.5fr),
  gutter: 2em,
  imagefigure("./majus.png", [MAJUS #citeauthor(<vasarely-majus>)]),
  [
    Fascinée depuis longtemps par les œuvres du plasticien et artiste Op-Art _Victor Vasarely_, j'ai été saisie par une de ses périodes, la période "Planetary Folklore", pendant laquelle il a expérimenté à travers plusieurs œuvres autour de l'idée d'un alphabet universel employant des séries combinaisons simples de formes et couleurs. D'apparence très simple, ces combinaisons sont d'une manières assez fascinantes uniques, d'où l'idée d'alphabet @planetary-folklore-period. 

    En particulier, un tableau, MAJUS, implémente à la fois ce concept, et est également une transcription d'une fugue de Bach.
  ]
)

Avec cette idée dans la tête, je me mets à gribouiller une ébauche d'"alphabet des formes", qui, naïvement, chercher à énumérer toutes les formes construisibles à partir de formes simples, que l'on peut superposer, pivoter et translater.

#grid(
  columns: (1fr, 1fr),
  gutter: 1em,
  imagefigure("./alphabetdesformes.png", "Un “alphabet” incomplet"),
  imagefigure("./alphabetdesformes.svg", "Une vectorisation")
)

Principalement par simple intérêt esthétique, je vectorise cette page via Illustrator. Vectoriser signifie convertir une image bitmap, représentée par des pixels, en une image vectorielle, qui est décrite par une série d'instructions permettant de tracer des vecteurs (d'où le nom), leur ajouter des attributs comme des couleurs, des règles de remplissage (Even-Odd, Non-Zero, etc.), des effets de dégradés, etc.

Un aspect intéréssant est que, parmi les différents formats d'image vectorielles existant, le _SVG_, pour _Scalable Vector Graphics_, est indéniablement le plus populaire, et est un standard ouvert décrivant un format texte.

Il est donc très facile de programmatiquement générer des images vectorielles à travers ce format.

== Une approche procédurale ?

#figure(caption: "Exemples d'œuvres résultant d'une procédure de génération semi-aléatoire, basée sur une grille de 8 “points d'ancrages”", grid(
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

Il existe cependant un moyen de "faire tourner du code Rust" dans un navigateur Web: la compilation vers WebAssembly (WASM), un langage assembleur pour le web @wasm, qui est une cible de compilation pour quelques des langages compilés plus modernes, comme Go @gowasm or Rust @rustwasm

En exportant la _crate_ shapemaker en bibliothèque Javascript via wasm-bindgen @wasmbindgen, il est donc possible d'exoser à une balise #raw("<script>", lang: "html") les fonctions de la bibliothèque, et brancher donc celles-ci à des _callbacks_ donnés par l'API WebMIDI:

#grid(
  columns: (1fr, 1fr),
  gutter: 2em,
  figure(caption: "Exposition de fonctions à WASM depuis Rust", text(size: 0.7em, [
    ```rust
    #[wasm_bindgen]
    pub fn render_image(opacity: f32, color: Color) -> Result<(), JsValue> {
        let mut canvas = /* ... */

        *WEB_CANVAS.lock().unwrap() = canvas;
        render_canvas_at(String::from("body"));

        Ok(())
    }
    ```
  ])),
  figure(caption: "Utilisation des fonctions exposées dans un script Javascript", text(size: 0.7em, [
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
  ]))
)

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
