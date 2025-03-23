#import "template.typ": arkheion, arkheion-appendices
#import "utils.typ": include-function, cut-around, cut-between

#import "@preview/diagraph:0.3.2"
#show raw.where(lang: "dot"): it => diagraph.render(it.text)
#show raw.where(lang: "mermaid"): it => diagraph.render(
  it.text.replace("graph TD", "digraph {").replace("-->", "->") + "}",
)


#let imagefigure(path, caption) = figure(
  image(path, width: 100%),
  caption: caption,
)

#let diagram(caption: "", content) = figure(
  caption: caption,
  kind: image,
  content,
)

#show link: underline

#show: arkheion.with(
  title: "Shapemaker: Créations audiovisuelles procédurales musicalement synchrones",
  authors: (
    (
      name: "Gwenn Le Bihan",
      email: "gwenn.lebihan@etu.inp-n7.fr",
      affiliation: "ENSEEIHT",
    ),
  ),
  date: [#datetime.today().day() Mars 2025],
  keywords: (
    "audiovisuel",
    "procédural",
    "SVG",
    "Rust",
    "WASM",
    "WebMIDI",
    "VST",
  ),
)


#align(center, pad(y: 1.7em, image("./dna-analysis-machine.png", width: 100%)))

#include-function(
  "../src/examples.rs",
  "dna_analysis_machine",
  lang: "rust",
  transform: it => "use shapemaker::*\n\n" + it,
)

#pagebreak()

#outline()

= Introduction

== À la recherche d'une impossible énumération des formes



#grid(
  columns: (1fr, 1.5fr),
  gutter: 2em,
  imagefigure("./majus.png", [MAJUS @vasarely-majus]),
  [
    Fascinée depuis longtemps par les œuvres du plasticien et artiste Op-Art _Victor Vasarely_, j'ai été saisie par une de ses périodes, la période "Planetary Folklore", pendant laquelle il a expérimenté à travers plusieurs œuvres autour de l'idée d'un alphabet universel employant des séries combinaisons simples de formes et couleurs. D'apparence très simple, ces combinaisons sont d'une manières assez fascinantes uniques, d'où l'idée d'alphabet @planetary-folklore-period.

    En particulier, un tableau, MAJUS, implémente à la fois ce concept, et est également une transcription d'une fugue de Bach.
  ],
)

Avec cette idée dans la tête, je me mets à gribouiller une ébauche d'"alphabet des formes", qui, naïvement, chercher à énumérer toutes les formes construisibles à partir de formes simples, que l'on peut superposer, pivoter et translater.

#grid(
  columns: (1fr, 1fr),
  gutter: 1em,
  imagefigure("./alphabetdesformes.png", "Un “alphabet” incomplet"),
  imagefigure("./alphabetdesformes.svg", "Une vectorisation"),
)

Principalement par simple intérêt esthétique, je vectorise cette page via Illustrator. Vectoriser signifie convertir une image bitmap, représentée par des pixels, en une image vectorielle, qui est décrite par une série d'instructions permettant de tracer des vecteurs (d'où le nom), leur ajouter des attributs comme des couleurs, des règles de remplissage (Even-Odd, Non-Zero, etc.), des effets de dégradés, etc.

Un aspect intéréssant est que, parmi les différents formats d'image vectorielles existant, le _SVG_, pour _Scalable Vector Graphics_, est indéniablement le plus populaire, et est un standard ouvert décrivant un format texte.

Il est donc très facile de programmatiquement générer des images vectorielles à travers ce format.

== Une approche procédurale ?

#figure(
  caption: "Exemples d'œuvres résultant d'une procédure de génération semi-aléatoire, basée sur une grille de 8 “points d'ancrages”",
  grid(
    columns: (1fr, 1fr, 1fr),
    ..(
      "designing-a-font",
      "drone-operating-system",
      "HAL-9000",
      "japan-sledding-olympics",
      "lunatic-green-energy",
      // "measuring-spirits",
      "phone-cameras",
      "reflections",
      "spline-optimisation",
      "weaving",
    ).map(artwork => grid.cell(
      image("../examples/gallery/" + artwork + ".svg", width: 100%),
    ))
  ),
)

L'étape prochaine dans cette démarche était évidemment donc de générer procéduralement ces formes. Afin d'avoir des résultats intéréssants, et devant l'évidente absurdité d'un projet d'énumération _complète_ de _toutes les formes_, on préfèrera des générations procédurales dites "semi-aléatoires", dans le sens où certains aspects du résultat final sont laissés à l'aléatoire, comme le placement des formes élémentaires, tandis que de d'autres, comme la palette de couleurs, sont des décisions de l'artiste.

Le modèle initialement choisi dans les premières ébauches de Shapemaker est le suivant:

#figure(
  caption: "Vocabulaire visuel des premières ébauches: grille de placement à 9 points, formes et couleurs",
  grid(
    columns: (1fr, 1fr, 1fr),
    gutter: 3em,
    grid.cell(image("./grid.svg"), align: center),
    grid.cell(image("./shapeshed.svg"), align: center),
    grid.cell(image("./colorshed.svg"), align: center)
  ),
)

L'idée est donc de limiter la part d'aléatoire à des choix dans des ensembles prédéfinis d'éléments, que ce soit dans le choix des couleurs, des placements ou des formes élémentaires.

Cette méthode amène donc l'artiste à définir, d'une certaine manière, son _propre langage visuel_, où les éléments de langage sont les couleurs, formes, placements et post-traitements (flou, rotations, etc) utilisables.

La part aléatoire engendre _une_ infinité réduite d'œuvres, qui naissent dans les confins du langage visuel devisé par l'artiste.

== Excursion dans le monde physique

#figure(
  caption: [Planches d'impression (merci à Relais Copies @relaiscopies)],
  stack(
    image("./street/workshop.jpeg"),
    // image("./street/stack.jpeg")
  ),
)

Bien évidemment, les décisions dans le processus créatif ne s'arrêtent pas au choix du vocabulaire visuel utilisé par le processus de génération.

Étant donné la simplicité avec laquelle l'on peut générer de grandes quantités d'œuvres à partir d'un même langage, le _choix d'en sélectionner les meilleures_ influe évidemment sur la série exposée et/ou partagée.

C'est dans cette optique que j'ai réalisé une série d'impressions de 30 générations, dont certaines ont été légèrement retouchées après génération.



=== Interprétation collective

Avec 30 œuvres abstraites sans nom, je me suis posé la question de comment les nommer. J'aurais pu les nommer au gré de ma propre imagination, mais j'ai trouvé intéréssant le faire de laisser cette décision au grand public, qui tomberait né à né avec ces manifestations de pseudo-hasard virtuel.

Le choix du nom d'une œuvre, en particulier quand elle est aussi abstraite et dénuée de contexte explicite, peut se faire parmi une potentielle infinité de titres, du littéral, au descriptiviste au poétique.

Les œuvres possèdent toutes un QR code amenant sur une page web qui permet de (re)nommer l'œuvre, en y apposant optionnellement son nom, en l'adoptant jusqu'à ce que lea prochain·e n'en prenne la garde.

J'ai donc laissé le public trouver ces œuvres, cachées à travers la ville, dans l'esprit des fameux _Spaces Invaders_ de Paris @spaceinvadersparis (qui d'ailleurs étendent leur colonisation bien au-délà de Paris, allant même jusqu'à l'ISS @spaceinvadersiss).


#let work = (slug, caption, with-context: false, screenshot: true) => figure(
  caption: caption,
  grid(
    gutter: 0.5em,
    columns: if screenshot {
      (if with-context { 2fr } else { 1fr }, 3fr)
    } else {
      1fr
    }
    ,
    if screenshot {
      grid.cell(rowspan: 2, image("./street/" + slug + "-screenshot.png"))
    },
    image("./street/" + slug + ".jpeg"),
    if with-context {
      image("./street/" + slug + "-context.jpeg")
    },
  ),
)


#work("paramount", ["Paramount"])
#work("reflets-citadins", ["Reflets Citadins", nommée par _Enide_])
#work(
  "lenvolée-du-cerf-volant",
  ["l'envolée du Cerf-Volant", nommée par _Nicolas C._],
)

Certaines ont été souvent renommées, beaucoup ont été volées, et certaines restent encore inconquises.

#work("danse-le-ciel", ["Danse le ciel"], with-context: true)
#work("bridging", [_Sans titre_], with-context: true)

== Lien musical

#figure(
  caption: [Frames d'une _story_ Instagram montrant une première esquisse de vidéo],
  stack(
    dir: ltr,
    ..range(7).map(it => image(
      "./blackmirrorlike/frame-" + str(it) + ".png",
      width: 14%,
    )),
  ),
)

À force de générer des centaines de petites images géométriques, il m'est venu à l'idée de les transformer en frames d'une _vidéo_.

Afin d'évaluer à quoi pourrait ressembler une telle chose, j'ai commencé par simplement faire une boucle, écrasant un même fichier .png à un intervalle de temps régulier, fichier ouvert dans XnView @xnview, qui permet de se re-charger automatiquement quand le fichier affiché change.

Bien évidemment, surtout s'il s'agit d'une vidéo synchronisée à sa bande son, il ne suffit pas de générer une frame aléatoire chaque seconde. Il faut pouvoir _réagit à des moments et rythmes clés du morceau_.

= Une _crate_ Rust avec un API sympathique

#diagram(
  caption: [Pipeline],
  scale(80%, reflow: true)[
    ```dot
    digraph G {
      rankdir="LR";
      compound=true;
      node[shape="record"];

      subgraph cluster_0 {
        label = "Render loop"
        style = "filled"
        color = "#f0f0f0"

       
        // Create a more circular arrangement using rank constraints
        { rank=same; "next frame"; rasterize; }
        { rank=same; hooks; "render to SVG"; }
        { rank=same; canvas; }
        
        // Set specific weights to encourage circular layout
        "next frame" -> hooks [weight=2];
        hooks -> canvas [weight=2];
        canvas -> "render to SVG" [weight=2];
        "render to SVG" -> rasterize [weight=2];
        rasterize -> "next frame" [weight=2];
        
        // Add some balancing invisible edges
        "next frame" -> canvas [style=invis, weight=0.5];
        hooks -> "render to SVG" [style=invis, weight=0.5];
        canvas -> rasterize [style=invis, weight=0.5];
        "render to SVG" -> "next frame" [style=invis, weight=0.5];
        rasterize -> hooks [style=invis, weight=0.5];
      }

      syncdata[label="sync data"];

      audioin[label="stems .wav + BPM"]
      midi[label="MIDI export"]
      flp[label=".flp project file"]

      midi -> syncdata
      audioin -> syncdata
      flp -> syncdata

      syncdata -> "next frame"

      usercode[label="user code"];
      usercode -> hooks 

      "rasterize" -> "video encoder"
      syncdata -> audio -> "video encoder"
    }
    ```
  ]
)

#diagram(
  caption: [Organisation des sous-modules],
  raw(
    lang: "mermaid",
    cut-between(
      it => it == "```mermaid",
      it => it == "```",
      read("../src/README.md"),
    ),
  ),
)


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

#figure(
  caption: "Exposition de fonctions à WASM depuis Rust, et utilisation de celles-ci dans un script Javascript",
  grid(
    columns: (1fr, 1fr),
    gutter: 2em,
    text(
      size: 0.75em,
      [
        ```rust
        #[wasm_bindgen]
        pub fn render_image(opacity: f32, color: Color) -> Result<(), JsValue> {
            let mut canvas = /* ... */

            *WEB_CANVAS.lock().unwrap() = canvas;
            render_canvas_at(String::from("body"));

            Ok(())
        }
        ```
      ],
    ),
    text(
      size: 0.75em,
      [
        ```js
        import init, { render_image } from "./shapemaker.js"

        void init()

        navigator.requestMIDIAccess().then((midi) => {
          Array.from(midi.inputs).forEach((input) => {
            input[1].onmidimessage = (msg) => {
              const [cmd, ...args] = [...msg.data]
              if (cmd !== 144) return

              const [pitch, velocity] = args
              const octave = Math.floor(pitch / 12) - 1

              render_image(velocity / 128, colors[octave])
            }
          })
        })
        ```
      ],
    ),
  ),
)

Au final, on peut arriver à une performance live interactive @pianowasmdemo intéréssante, et assez réactive pour ne pas avoir de latence (et donc de désynchronisation audio/vidéo) perceptible.

Les navigateurs Web supportant nativement le format SVG, qui se décrit notamment comme incluable directement dans le code HTML d'une page web @svginhtml, il est possible de simplement générer le code SVG, et de laisser le navigateur faire le rendu, ce qui s'avère être une solution très performante.

== Amplitudes de _stems_

```rs
let mut reader = hound::WavReader::open(path.clone())
  .map_err(|e| format!("Failed to read stem file: {}", e))
  .unwrap();

let spec = reader.spec();

let sample_index_to_frame = |sample: usize| {
  (sample / spec.channels / spec.sample_rate * self.fps) as usize
};

let mut amplitude_db: Vec<f32> = vec![];
let mut current_amplitude_sum: f32 = 0.0;
let mut current_amplitude_buffer_size: usize = 0;
let mut latest_loaded_frame = 0;

for (i, sample) in reader.samples::<i16>().enumerate() {
  let sample = sample.unwrap();
  if sample_index_to_frame(i) > latest_loaded_frame {
    amplitude_db
        .push(current_amplitude_sum / current_amplitude_buffer_size as f32);
    current_amplitude_sum = 0.0;
    current_amplitude_buffer_size = 0;
    latest_loaded_frame = sample_index_to_frame(i);
  } else {
    current_amplitude_sum += sample.abs() as f32;
    current_amplitude_buffer_size += 1;
  }
}

let stem = Stem {
  amplitude_max: *amplitude_db.iter().max().unwrap(),
  amplitude_db,
  duration_ms: (reader.duration() / spec.sample_rate * 1000.0) as usize,
};

// Write loaded stem to a CBOR cache file
Stem::save_to_cbor(&stem, &cached_stem_path);
```

== Export MIDI

#raw(
  lang: "rust",
  cut-around(
    it => it.trim().starts-with("// Add notes"),
    it => it == "    }",
    read("../src/synchronization/midi.rs"),
  ),
)

```
Commit 7ae7a14a90f16f664edee3f433ade9b8c5019ffa

⚗️ Figure out a POC to get notes from MIDI file into note[ms][stem_name]

And the conversion from MIDI ticks to milliseconds does not drift at
all, after 6 mins on a real-world track (see research_midi/source.mid),
it's still fucking _spot on_, to the FUCKING CENTISECOND (FL Studio
can't show me more precision anyways).

So beautiful.

aight, imma go to sleep now
```

== Fichier de projet

#include-function(
  "../research/adapters/flstudio/adapter.py",
  "main",
  lang: "python",
)

== Dépôt de "sondes" dans le logiciel de MAO

#include-function(
  "../src/vst/beacon.rs",
  "connect_to_beacon",
  lang: "rust",
)

#include-function(
  "../src/vst/beacon.rs",
  "register_probe",
  lang: "rust",
)

= Performance

= Conclusion


// Add bibliography and create Bibiliography section
// #bibliography("bibliography.yaml", style: "./ieee-with-locations.csl")
#bibliography("bibliography.yaml")
