#import "template.typ": arkheion, arkheion-appendices, monospace
#import "utils.typ": include-function, cut-around, cut-between, dedent

#import "@preview/diagraph:0.3.2"
#show raw.where(lang: "dot"): it => diagraph.render(it.text)
#show raw.where(lang: "mermaid"): it => diagraph.render(
  it.text.replace("graph TD", "digraph {").replace("-->", "->") + "}",
)


#let imagefigure(path, caption) = figure(
  image(path, width: 100%),
  caption: caption,
)

#let diagram(caption: "", size: 100%, content) = figure(
  caption: caption,
  kind: image,
  scale(size, content, reflow: true),
)

#let breakout(content) = block(
  inset: 1em,
  fill: luma(240),
  radius: 4pt,
  width: 100%,
  pad(x: 1em, align(center, text(size: 1.1em, content))),
)

#let codesnippet(caption: "", content, lang: "rust", size: 1em) = {
  let snip = text(
    size: size,
    block(
      inset: 1.5em,
      fill: luma(240),
      radius: 4pt,
      width: 100%,
      // Figure itself is already non breakable, afaik
      breakable: caption != "",
      if type(content) == str {
        raw(
          lang: lang,
          content,
        )
      } else {
        content
      },
    ),
  )

  if caption != "" {
    figure(caption: caption, align(left, snip))
  } else {
    snip
  }
}

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
  // keywords: (
  //   "audiovisuel",
  //   "procédural",
  //   "DSP",
  //   "SVG",
  //   "Rust",
  //   "WASM",
  //   "MIDI",
  //   "VST",
  // ),
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

Pour implémenter cette génération, il faut donner donc un moyen à l'artiste de décrire sa procédure de génération.

Ainsi, Shapemaker est une bibliothèque réutilisable, ou _crate_ dans l'écosystème Rust @rustcrates.

La création d'un procédé de génération est conceptualisée par un canvas, composé de une ou plusieurs couches ou _layers_ d'objets. Ces objets sont _colorés_ (possèdent une information sur la manière dont il faut les remplir: bleu solide, hachures cyan, etc.), et peuvent également subir des filtres et transformations #footnote[Avec un peu de recul, le terme d'objet texturé est plus approprié, mais le code n'a pas encore changé]. Ils sont aussi _placés_ dans l'espace du canvas: le canvas possède une information de _région_, un intervalle 2D de points valables. Les objets se placent dans cette région, en stockant dans leur structure les coordonnées de _points_ marquant leur positionnement dans l'espace (coins pour un #raw(lang: "rust", "Object::Rectangle"))

#diagram(
  caption: [Modèle objet du Canvas],
  size: 90%,
  ```dot
  digraph {
    // rankdir="LR";
    node [shape="record"];

    Canvas -> Layer [label="1+"]
    region2 [label="Region"]
    Layer -> region2
    Canvas -> Region [label=".world_region"]
    point2 [label="Point"]
    Region -> point2 [label="RegionIterator"]
    Layer -> ColoredObject [label="0+"]
    Object -> "Object::Rectangle,\nObject::Circle,\n…" -> Point
    ColoredObject -> Object
    ColoredObject -> Fill
    ColoredObject -> Transform
    ColoredObject -> Filter
    Fill -> "Fill::Solid,\nFill::Hatches,\n…" -> Color
    Transform -> "Transform::Rotate,\nTransform::Translate,\n…"
    Filter -> "Filter::Blur,\nFilter::Glow,\n…"
  }
  ```,
)

Ce modèle mental permet de travailler plus efficacement car il est bien plus proche de la manière dont on a tendance à penser l'art visuel: sur Illustrator par exemple, ce sont des objets, organisés en plusieurs couches, qui possèdent des attributs dictant leur remplissage.

Les concepts de transformations et de filtres sont également très proche de ce qu'on peut retrouver dans des logiciels de création d'images raster, comme Photoshop.



#grid(
  columns: (1fr, 1fr),
  gutter: 1em,
  [
    La bibliothèque fournit une grande quantité de fonctions utiles pour redimensionner des régions, en prendre le milieu.

    La partie purement géométrique de la bibliothèque, définissans `Point`, `Region` et leurs opérations utiles associées (itérer les points d'une région, calculer le milieu d'une région, etc.), sont regroupées dans `shapemaker::geometry`.

    Les définitions des objets et de tout leurs aspects visuels (`Fill`, `Transform`, `Filter`, `Color`, `Object`, `ColoredObject`) sont regroupées dans `shapemaker::objects`.

    Il y a également `shapemaker::random` qui regroupe des fonctions de génération aléatoire, permettant d'introduire facilement et de manière plus ou moins granulaire, une part d'aléatoire dans le processus de génération: `Region.random_point()`, `Color::random()`, etc.

    Enfin, `shapemaker::rendering` implémente le rendu d'un canvas et de tout ce qu'il contient en SVG
  ],
  diagram(
    caption: [Dépendances entre les modules de la bibliothèque],
    size: 60%,
    raw(
      lang: "mermaid",
      cut-between(
        it => it == "```mermaid",
        it => it == "```",
        read("../src/README.md"),
      ),
    ),
  ),
)



= Rendu en images

Maintenant que l'on a cette structure, il est bien évidemment essentiel de pouvoir la rendre en un fichier image exploitable, en PNG par exemple.

L'idée est d'exploiter le standard SVG et tout l'écosystème existant autour pour éviter d'avoir à ré-implémenter un moteur de rasterisation à la main: SVG possède déjà énormément de fonctionnalités, et faire ainsi nous permet de fournir un "escape hatch" et de fournir à Shapemaker des fragments de code SVG pour des cas spécifiques que la bibliothèque ne couvrirait pas, à travers `Object::RawSVG`, qui prend en argument un arbre SVG brut.

Ce processus de rendu est réalisé via l'implémentation d'un trait, une sorte d'équivalent des interfaces dans les langages orientés objet @rusttraits:

#codesnippet(
  lang: "rust",
  cut-around(
    it => it.trim().starts-with("pub trait SVGRenderable"),
    it => it == "}",
    read("../src/rendering/renderable.rs"),
  ),
)

Ce _trait_ est ensuite implémenté par la plupart des structures de `shapemaker::graphics`:

/ Canvas: rendu de toutes ses `Layer`, en prenant garde à les ordonner correctement pour que les premières couches soit déssinées par dessus les dernières
/ Layer: rendu de l'ensemble des `ColoredObject` qu'elle contient, en les regroupant dans un groupe SVG #raw(lang: "svg", "<g>")
/ ColoredObject: rendu de l'`Object` qu'il contient, en appliquant les transformations et filtres
/ Object: dépend de la variante: `Object::Rectangle` est rendu comme un #raw(lang: "svg", "<rect>"), `Object::Circle` est rendu comme un #raw(lang: "svg", "<circle>"), etc.
/ Fill: dépend de la variante: simple attribut SVG `fill` pour `Fill::Solid`, utilisation de #raw(lang: "svg", "<pattern>") pour `Fill::Hatches`, etc.
/ Transform: attribut SVG `transform`
/ Filter: définition d'un #raw(lang: "svg", "<filter>") avec les attributs correspondants
/ Color: utilise le `ColorMapping` donné pour réaliser sa variante en une valeur de couleur SVG (notation hexadécimale)

#diagram(
  caption: [Objets rendables en SVG],
  size: 60%,
  ```dot
  digraph {
    // rankdir="LR";
    node [shape="record", style="filled", fillcolor="#e0e000"];

    Canvas -> Layer
    region2 [label="Region", style="solid"]
    Layer -> region2
    Canvas -> Region
    point2 [label="Point", style="solid"]
    Region -> point2
    Layer -> ColoredObject
    Point[style="solid"]
    Object -> "Object::Rectangle,\nObject::Circle,\n…" -> Point
    ColoredObject -> Object
    ColoredObject -> Fill
    ColoredObject -> Transform
    ColoredObject -> Filter
    Fill -> "Fill::Solid,\nFill::Hatches,\n…" -> Color
    Transform -> "Transform::Rotate,\nTransform::Translate,\n…"
    Filter -> "Filter::Blur,\nFilter::Glow,\n…"
  }
  ```,
)

#grid(
  columns: (1fr, 1fr),
  gutter: 2em,
  [
    Les arguments `cell_size` et `object_sizes` permettent de réaliser en valeur concrètes (pixels) les valeurs de taille abstraites: la distance unitaire entre deux points est définie par `cell_size`, et les tailles des objets, qui, par choix, n'est pas contrôlable finement, sont définies par `object_sizes`.
  ],
  codesnippet(
    lang: "rust",
    size: 0.9em,
    cut-around(
      it => it.trim().starts-with("pub struct ObjectSizes"),
      it => it == "}",
      read("../src/graphics/objects.rs"),
    ),
  ),
)

En suite, pour convertir en PNG, on utilise une autre bibliothèque, _resvg_, qui implémente presque complétement la spécification SVG 1.1, et l'implémente même mieux que Firefox, Safari et Chrome @resvg. L'arbre SVG que l'on a construit est sérialisé en string, puis parsé par _resvg_, qui le transforme en un arbre de rendu, qui est ensuite rasterisé en une pixmap#footnote[Matrice plate de pixels RGBA], qui est finalement écrit dans un fichier PNG.

#diagram(
  caption: [Rendu d'un canvas SVG en PNG],
  ```dot
  digraph {
    rankdir="LR";
    node [shape="record"];
    "svg tree" -> "svg string"
    "svg string" -> "usvg tree"
    "usvg tree" -> "pixmap"
    pixmap -> "png file"
  }
  ```,
)

Le passage par une string svg est évidemment une perte de performance, qui est discutée #ref(<perf-svgstring>, form: "page")


= Render loop et hooks

On peut maintenant rasteriser un canvas. Passer à l'étape vidéo donc à réaliser cette opération sur chaque _frame_ de la vidéo finale. Cependant, la vidéo devant se synchroniser au son, la tâche est rendu plus difficile: en effet, il ne suffit pas d'exposer à l'artiste une fonction `render_frame`, qui prendrait en argument le numéro de frame actuel et permettrait de définir le canvas pour chaque frame: on a besoin de moyen de _réagir_ à des moments clés de la musique.

Pour donner les moyens à l'artiste d'exprimer cela, on utilise un concept assez commun en programmation, les _hooks_, nommés ainsi car, essentiellement, ils permettent à du code utilisateur de s'imiscer dans certains moments de l'exécution d'une bibliothèque @hooks.

Dans notre cas, on va donner les hooks suivants:

/ each_beat: Appelé sur chaque nouveau temps fort de la musique
/ on_note: Appelé à chaque début de note jouée, par un ou des instruments en particulier à préciser
/ at_timestamp: Appelé une fois, à un instant précis de la vidéo
/ ...: et pleins d'autres

Les hook stockent simplement deux fonctions: `when` pour savoir si le hook doit être exécuté à in instant précis, et `render_function` qui contient les actions à effectuer à cet instant.

#codesnippet(
  size: 0.85em,
  cut-around(
    it => it.trim().starts-with("pub struct Hook"),
    it => it == "}",
    read("../src/video/engine.rs"),
  )
    + "\n\n"
    + cut-around(
      it => it.trim().starts-with("pub type HookCondition"),
      it => it.trim().ends-with(";"),
      read("../src/video/engine.rs"),
    )
    + "\n\n"
    + cut-around(
      it => it.trim().starts-with("pub type RenderFunction"),
      it => it.trim().ends-with(";"),
      read("../src/video/engine.rs"),
    ).replace("anyhow::Result", "Result"),
)

Un hook reçoit notamment une référence mutable au Canvas #raw(lang: "rust", "&mut Canvas") car il _modifie le canvas de la frame en cours_. Le moteur de rendu vidéo ne possède en fait qu'un seul canvas, qui est successivement modifié au long de la vidéo.

Le générique #raw(lang: "rust", "<C>") existe car l'artiste peut définir des données additionelles à stocker dans le contexte, pratique pour stocker des données à travers la vidéo, au delà de l'exécution d'un unique hook#footnote[Par exemple, "quelle a été la dernière ligne de parole affichée? il faut passer à la prochaine"]

On met également à disposition une méthode `with_hook`, qui rajoute un hook à la liste, permettant de facilement les définir:


#codesnippet(
  include-function(
    "../src/video/engine.rs",
    "with_hook",
    lang: "rust",
    is_method: true,
    transform: it => (
      "impl Video<C> {\n    ...\n"
        + it.replace("<AdditionalContext>", "<C>")
        + "\n}"
    ),
  ),
)

Voici par exemple la définition du hook `on_note`:

#codesnippet(
  size: 0.9em,
  include-function(
    "../src/video/engine.rs",
    "on_note",
    lang: "rust",
    is_method: true,
    transform: it => (
      "impl Video<C> {\n    ...\n"
        + it.replace("<AdditionalContext>", "<C>")
        + "\n}"
    ),
  ),
)

Le moteur de rendu vidéo est donc une boucle qui, à chaque frame, regarde dans l'ensemble des _hooks_ enregistrés, lesquels doivent être exécutés, les exécute, puis rasterise le canvas en une frame qui est ensuite donnée à l'encodeur vidéo:

#diagram(
  caption: [Pipeline],
  size: 60%,
  ```dot
  digraph G {
    rankdir="LR";
    compound=true;
    node[shape="record"];

    subgraph cluster_0 {
      label = "Render loop"
      style = "filled"
      color = "#f0f0f0"

      // Set specific weights to encourage circular layout
      "next frame" -> hooks [weight=2, label="Trigger"];
      hooks -> canvas [weight=2, label="Modify"];
      canvas -> frame [weight=2, label="Render"];
      frame -> "next frame" [weight=2];
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
    usercode -> hooks  [label="Specifies"]

    frame -> video
    syncdata -> audio -> video
  }
  ```,
)

La boucle de rendu en elle-même itère sur *les instants, ms par ms, et non pas les frames*. C'est important pour garder la vidéo en synchronisation avec le son. J'avais initialiement fait la boucle sur les frames, et la vidéo se décalait progressivement.

#codesnippet(```rust
let render_ms_range = self.start_rendering_at..self.duration_ms();

let mut frames_to_encode: Vec<(Time, String)> = vec![];

for _ in render_ms_range.into_iter() {
  context.ms += 1_usize;
  context.frame = self.fps * context.ms / 1000;
```)

On exécute bien les hooks à chaque itération de la boucle, mais par contre on ne rend une nouvelle frame que quand le numéro de frame change:

#codesnippet(
  dedent(
    cut-around(
      it => it
        .trim()
        .starts-with("if context.frame != previous_rendered_frame"),
      it => it.trim().ends-with("}"),
      read("../src/video/encoding.rs"),
    ),
  ),
)

La rastérisation est l'encodage sont réalisés après la fin de la boucle de rendu pour pouvoir paralléliser la rastérisation, voir #ref(<perf-parallelrasterize>).


= Sources de synchronisation

On a pu voir dans les exemples de code précédents que les hooks reçoivent deux arguments essentiels dans leur fonctions: le _canvas_, discuté précédemment, et un _contexte_.

Ce contexte, en plus de quelques informations déposées par la boucle de rendu (milliseconde actuelle, numéro de frame actuel, etc), contient surtout _des informations musicales sur l'instant présent_, comme les notes actuellement jouées, les amplitudes instantanées de chaque piste, etc.

Afin d'obtenir ces information, il faut analyser quelque chose: la question est donc, de quels fichiers ou signaux tirer parti pour construire ces informations?

Les sous-sections suivantes traites des différentes approches explorées:

/ Amplitudes de _stems_: utilisation des signaux audio bruts depuis des exports piste par piste du morceau
/ Analyser de fichiers MIDI: utilisation d'un standard stockant des informations de notes jouées.
/ Analyse de fichiers .flp: utilisation des fichiers de projet de FL Studio, un logiciel de production musicale. C'est l'équivalent d'un fichier source en programmation
/ Sondes dans le logiciel de MAO#footnote[MAO: Musique Assistée par Ordinateur]: utilisation de plugins VST pour envoyer des informations de synchronisation potentiellement arbitraire, directement depuis le logiciel de production musicale. //
/ Temps réel: utilisation de signaux MIDI en "live", solution contournant le problème de la synchronisation et toute la partie rendu vidéo et rastérisation. Plutôt prévue pour un autre cas d'usage, les utilisations en concert et installations live

Dans chacun de ces cas, l'objectif est de pouvoir inférer depuis ces ressources les informations suivantes:

- Le BPM#footnote[Beats per minute, aussi appelé tempo] du morceau, avec éventuellement des évolutions au cours du morceau
- D'éventuels marqueurs temporels permettant de réagir à des changements de phrases musicales (par exemple, la classique construction _build-up_ / _drop_ / _break_ en EDM#footnote[Electronic Dance Music]), sans avoir à harcoder un timestamp dans le code de la vidéo: ces marqueurs sont placés dans le logiciel de production musicale (cf #ref(<flstudiomarkers>), #ref(<flstudiomarkers>, form: "page"))
- Pour chaque instrument, et à chaque instant:
  - Les notes jouées: pitch#footnote[hauteur] et vélocité#footnote[intensité avec laquelle la note a été jouée]
  - Des éventuelles évolutions de paramètres influant sur le timbre de l'instrument (ouverture d'un filtre passe bas pour un synthétiseur, pédale de sustain pour un piano, etc)


== Amplitudes de _stems_

Cette approche consiste à demander à l'artiste de fournir un fichier audio par piste du morceau de musique. On entend "piste" ici assez vaguement, plus le nombre de fichiers est grand, plus il est possible de réagir à des changements d'amplitudes individuels. En général, une piste correspond un-à-un à un instrument.

=== Accessibilité

Exporter un projet en fichiers audios piste-par-piste, des _stems_, est une pratique plutôt courante, par exemple lors de concours de remix @remixconteststems, pour fournir aux participant·e·s les éléments du morceau séparés et ainsi faciliter la création d'un remix.

On pourrait faciliter encore plus l'usage en, par exemple, proposant de faire de la séparation de source par réseaux neuronaux si l'artiste ne peut pas ou ne souhaite pas faire un export en stems @sourcesep. Cette approche serait d'autant plus utile car l'on n'a pas le besoin ici d'une qualité sonore sur les pistes séparées, étant donné que l'on ne s'en sert qu'à des fins d'analyse pour de la synchronisation.



=== Performance

Néanmoins, ce processus de lire dans une structure de donnée les amplitudes à chaque instant reste assez coûteux, que ce soit en temps de calcul ou en mémoire.

=== Faisabilité

De plus, la correspondance signal $mapsto$ note jouée est beaucoup moins évidente qu'elle n'en paraît. Un signal peut être décomposé en amplitude et fréquence, mais une note possède deux caractéristiques bien plus utiles aux musicien·ne·s:

/ Vélocité $cancel(arrow.l.bar)$ amplitude: Les amplitudes d'un signal sont très variables, et il est difficile de déterminer un seuil de déclenchement efficace, en prenant en compte la présence d'effets (en particulier l'echo ou la réverbération).
/ Pitch $arrow.l.bar$ fréquence: Pour obtenir le pitch d'une note, il faut effectuer une analyse fréquentielle du signal. Ceci pourrait à priori ne pas être trop complexe, mais n'a pas été tenté étant donné les difficultés soulevées par le point précédent. Il est en plus très difficile de séparer plusieurs notes d'un accord.


== Export MIDI

Cette méthode consiste d'une certaine manière à prendre le problème "à l'envers" par rapport à la méthode précédente: on part d'information _sur les notes jouées_, desquelles on peut dériver les amplitudes, depuis la vélocité.

=== Faisabilité

Le format MIDI @midispec permet de spécifier:

- Pour chaque piste: les notes jouées (pitch et vélocité)
- Pour le morceau dans sa globalité, le BPM

Bien que l'on puisse assez facilement inférer une sorte d'amplitude simulée à partir des vélocités, le problème inverse se pose: si l'on veut animer un objet en prenant en compte les échos, par exemple, MIDI ne peut pas nous aider.

Mais pour de nombreux usages, le résultat final paraît beaucoup plus "en réaction avec la musique" qu'avec une approche par amplitudes réelles, certainement grâce à la précision apportée par le fait d'utiliser les évènements de notes jouées "à la source".

==== Ticks MIDI

Pour l'implémentation, rien de bien compliqué, on rajoute les notes une à une dans notre structure de données en partant des évènements MIDI:

#codesnippet(
  lang: "rust",
  dedent(
    cut-around(
      it => it.trim().starts-with("match message"),
      it => it == "                }",
      read("../src/synchronization/midi.rs"),
    ),
  ),
)


…Sauf que les coordonnées temporelles MIDI sont en _deltas de ticks MIDI_. Les ticks sont indépendant du BPM, et les deltas sont des simples différences du nombre de ticks passés entre deux évènements.

La durée d'un tick est aussi dépendante du _PPQ_, ou _Pulse per quarter_, qui correspond à la résolution temporellle d'un fichier MIDI, c'est l'équivalent des FPS en vidéos ou de la fréquence d'échantillonage en audio @midippq.

#codesnippet(
  include-function(
    "../src/synchronization/midi.rs",
    "midi_tick_to_ms",
    lang: "rust",
  ),
)

Pour passer de ticks à des millisecondes réelles, il faut réifier ces ticks en lisant le BPM, *qui peut changer au cours du morceau*. Les changements de BPM sont des évènements MIDI parmi le stream plat du fichier.

#codesnippet[
  ```rust
  // Convert deltas to absolute ticks
  let mut track_no = 0;
  for track in midifile.tracks.iter() {
      track_no += 1;
      let mut absolute_tick = 0;
      for event in track {
          absolute_tick += event.delta.as_int();
          timeline
              .entry(absolute_tick)
              .or_default()
              .insert(track_names[&track_no].clone(), *event);
      }
  }

  // Convert ticks to ms
  let mut absolute_tick_to_ms = HashMap::<u32, usize>::new();
  let mut last_tick = 0;
  for (tick, tracks) in timeline.iter().sorted_by_key(|(tick, _)| *tick) {
      for event in tracks.values() {
          if let TrackEventKind::Meta(MetaMessage::Tempo(tempo)) = event.kind {
              now.tempo = tempo.as_int() as usize;
          }
      }
      let delta = tick - last_tick;
      last_tick = *tick;
      now.ms += midi_tick_to_ms(delta, now.tempo, now.ticks_per_beat as usize);
      absolute_tick_to_ms.insert(*tick, now.ms);
  }
  ```
]



=== Performance

L'inférence d'amplitudes à partir des vélocités est assez coûteuse. La raison de ce coût n'a pas encore été étudiée.

=== Accessibilité

Malheureusement, là où l'export d'un projet musical en stems se résume à un simple clic dans un menu, l'export en MIDI est souvent plus complexe. Par exemple, sur FL Studio, il demande à créer _une copie du projet, avec toutes les pistes converties en "instruments MIDI"_, ce qui est fastidieux:

#imagefigure(
  "./flstudiomidimacro.png",
  [
    Dialoge d'avertissement lors de l'utilisation de la macro "Prepare for MIDI export" dans FL Studio
  ],
)

=== Conclusion

Cette méthode, malgré l'aspect fastidieux de sa mise en place, est une amélioration nette par rapport à l'approche par amplitude:

#codesnippet[
  #monospace[
    Commit #link("https://github.com/gwennlbh/shapemaker/commit/7ae7a14a90f16f664edee3f433ade9b8c5019ffa")[7ae7a14a90f16f664edee3f433ade9b8c5019ffa]
  ]

  ```
  ⚗️ Figure out a POC to get notes from MIDI file into note[ms][stem_name]

  And the conversion from MIDI ticks to milliseconds does not drift at
  all, after 6 mins on a real-world track (see research_midi/source.mid),
  it's still fucking _spot on_, to the FUCKING CENTISECOND (FL Studio
  can't show me more precision anyways).

  So beautiful.

  aight, imma go to sleep now
  ```]

== Fichier de projet

Étant donné l'aspect fastidieux de la solution précédente, il est intéressant de se pencher sur les fichiers de projet des logiciels de production musicale, afin de _remonter totalement à la source du morceau de musique_: le fichier qui est ouvert par l'artiste, celui sur lequel iel travaille.

Malheureusement, les logiciel libres sont très loin derrière les standards de l'industrie en terme de production musicale, et il est ajourd'hui assez irréaliste de penser pouvoir produire de la musique avec des alternatives libres qui possède des formats de fichier de projet ouverts.

On doit donc se tourner vers de la rétro-ingénierie, et avoir une implémentation d'un "adapteur" pour chaque logiciel de production musicale que l'on souhaite supporter.

=== FL Studio

Il existe une bibliothèque Python, pyflp @pyflp, qui permet de parser les fichiers de projets FL Studio, et d'en extraire la quasi totalité.

#codesnippet(
  include-function(
    "../research/adapters/flstudio/adapter.py",
    "main",
    lang: "python",
    transform: it => "import pyflp\n\n" + it.replace("\n# end", ""),
  ),
)

Cependant, l'auteur·ice de la bibliothèque n'a malheureusemnet plus le temps de la maintenir @pyflp3.12, et, étant donné l'évolution de FL Studio, le parser est voué à progressivement ne plus supporter les dernières versions du logiciel.

Étant donné que je suis utilisatrice de FL Studio, je n'a pas cherché de potentielles solutions pour d'autres logiciels de MAO.

==== Performance

Étant donné que l'adapter est en Python, l'intégrer proprement dans Shapemaker consisterai à éventuellement utiliser une solution de FFI#footnote[Foreign Function Interface, permettant d'appeler des fonctions écrites dans un autre langage de programmation] comme PyOxide @pyo3, ce qui demanderait également beaucoup de travail d'adaptation.

== Dépôt de "sondes" dans le logiciel de MAO

#grid(
  columns: (3fr, 1fr),
  gutter: 1em,
  [

    Cette dernière solution, dont l'implémentation est encore en cours, consiste à donner la possibilité aux artistes d'exposer directement des signaux depuis leur logiciel, en les exfiltrant à Shapemaker à travers un VST#footnote[Virtual Studio Technology, un standard de plugins audio] @vst dédié.

    L'avantage de cette approche est qu'elle est agnostique au logiciel de MAO: en effet, VST est _le_ standard de plugins audio, supporté par tout les logiciels.

    C'est via cette technologie que les artistes peuvent jouer des instruments virtuels, allant des pianos physiquement simulés @pianoteq, en passant par vocaloïdes#footnote[simuateurs de parole chantée, cas à application musicale de la synthèse vocale] (comme par exemple Hatsune Miku @mikudayooo), aux synthétiseurs additifs, soustractifs, à wavetables (dont un exemple très populaire est Serum @serum).

    C'est aussi cette technologie qui est utilisée pour appliquer des effets aux signaux audio créés par les instruments (on parle de VST _effets_, contrairement aux VST _générateurs_), allant des modélisations de pédales d'effets de guitare ou de compresseurs analogiques à tube, aux simulation de compression digitale de signaux ("bitcrushing"), aux égaliseurs fréquentiels.

  ],
  imagefigure(
    "./flstudioprobe.png",
    [Un VST Shapemaker servant de sonde, dans une chaîne d'effets sur FL Studio],
  ),
)


#breakout[
  Il est donc possible de recevoir du signal, *autant audio que MIDI*, en entrée d'un VST.
]

Autre possibilité, qui s'avère utile parmis nos objectifs: les VSTs peuvent exposer à l'hôte (le logiciel de MAO) des paramètres changeables, ce qui permet de faire évoluer le timbre d'un instrument, l'intensité d'une réverbération, etc. Faire varier des paramètres au cours du temps est un aspect essentiel de la musique, en particulier électronique, qui contribue à "donner vie" à un morceau.

On peut donc également exposer des paramètres sur notre VST-sonde, qui peuvent servir à automatiser des changements de couleurs, de formes, etc, en suivant une évolution dans le timbre d'un instrument, par exemple, depuis la source directement (il suffit d'envoyer le signal d'automatisation au VST-sonde, en plus de l'instrument lui-même).

On exfiltre ensuite ces données hors du logiciel vers un "beacon", via un simple API WebSocket, qui permet une communication instantanée beaucoup plus performante que des requêtes HTTP, et est plus approprié à l'envoie de potentiellement plusieurs miliers de points de données par secondes: en effet, le VST-sonde s'imiscant dans la chaîne de traitement audio, il ne doit pas la ralentir considérablement, sous peine de rendre le logiciel de MAO inutilisable

#codesnippet(
  caption: "Implémentation de la fonction permettant à une probe de se signaler auprès du beacon",
  [
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
  ],
)

Enfin, on utilise la crate _nih-plug_ @nihplug pour exporter la partie VST de notre code en un plugin VST, chargeable dans un logiciel de MAO

#diagram(
  caption: [Exfiltration de données depuis la chaîne de traitement du logiciel de MAO],
  size: 80%,
  ```dot
    digraph G {
      rankdir="LR";
      compound=true;
      node[shape="record"];

      subgraph cluster_host {
        label = "Logiciel de MAO"

        subgraph cluster_track {
          label = "Pour chaque piste"
          midi -> instrument -> effects -> probe
          midi -> probe
          automation -> instrument
          automation -> probe
        }
      }

      subgraph cluster_shapemaker {
        label = "Shapemaker"
  wip[label="(en développement)", shape="plaintext"]
        beacon -> wip
      }

        probe -> beacon [label="ws://"]

    }
  ```,
)


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

= Performance

Les premiers prototypes de Shapemaker avait une implémentation sérielle, ou le code Rust ne s'occupait seulement de la partie génération de formes et sérialisation en SVG. Chaque frame SVG étaient sauvegardées dans un fichier, puis converti en PNG en ligne de commande via ImageMagick. Les frames étaient ensuite concaténées en une vidéo via FFmpeg, également en ligne de commande.

#diagram(
  caption: [Pipeline de rendu, premier prototype],
  size: 85%,
  ```dot
  digraph {
    rankdir="LR";
    node [shape="record"];
    subgraph cluster_each_frame {
      label = "Chaque frame"
      subgraph cluster_rust {
        label = "Rust"
        canvas -> "Frame 0037.svg"
      }
      "Frame 0037.svg" -> "Frame 0037.png" [label="$ magick convert"]
    }
    "Frame 0037.png" -> "video.mp4" [label="$ ffmpeg"]
  }
  ```,
)

Un des plus gros gains de performance a été d'éliminer le plus d'I/O#footnote[Input/Output] possible, et notamment aussi d'éviter un encodage/décodage PNG en passant des pixmap (matrices de pixels) directement


#diagram(
  caption: [Pipeline de rendu sans #emph[shell-out]s#footnote[Invoquer un programme en ligne de commande (dans un shell), au lieu de faire tourner du code dans le programme courant]],
  size: 85%,
  ```dot
  digraph {
    rankdir="LR";
    node [shape="record"];
    subgraph cluster_rust {
      label = "Rust"
      subgraph cluster_each_frame {
        label = "Chaque frame"
        canvas -> "Frame 0037.svg"
        "Frame 0037.svg" -> "Frame 0037.png" [label="resvg"]
      }
    "Frame 0037.png" -> "video.mp4" [label="libx264"]
    }
  }
  ```,
)

L'inconvénient est que, pour la partie encoding vidéo, il n'existe pas encore vraiment d'encodeur H.264#footnote[Codec vidéo, très souvent utilisé pour les fichiers MP4, par exemple] en pur Rust, la plupart des solutions étant des bindings#footnote[bibliothèque utilisant des FFIs pour donner un accès idiomatique à une bibloithèque provenant d'un autre langage de programmation] vers des bibliothèques C, notamment ffmpeg.

Cela rend l'installation de la bibliothèque beaucoup plus complexe, notamment sur Windows (les logiciels de production musicale sont très rares à fonctionner correctement sur Linux, surtout quand on prend en compte que les VSTs doivent eux aussi fonctionner sur Linux):

#codesnippet(
  caption: "Erreur recontrée pendant la compilation des bindings Rust à libx264",
  ```
       Compiling ffmpeg-sys-next v7.1.0
  error: failed to run custom build command for `ffmpeg-sys-next v7.1.0`
  note: To improve backtraces for build dependencies, set the CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG=true environment variable to enable debug information generation.

  Caused by:
    process didn't exit successfully: `C:\Users\…\projects.local\shapemaker\target\debug\build\ffmpeg-sys-next-d2108b58b450b79e\build-script-build` (exit code: 101)
    --- stdout
    Could not find ffmpeg with vcpkg: Could not look up details of packages in vcpkg tree could not read status file updates dir: The system cannot find the path specified. (os error 3)
  ```,
)

Malgré plusieurs guides contradictoires d'installation, utiliser _vcpkg_ @vcpkg pour installer ffmpeg a fini par fonctionner

Une fois cette optimisation faite, qui a *divisé par 10* le temps de rendu, on peut se pencher sur le détail de la boucle de rendu pour identifier les potentiels gains de performance


#diagram(
  caption: [Détail de la boucle de rendu],
  scale(90%, reflow: true)[
    ```dot
    digraph G {
      compound=true;
      splines="ortho";
      node[shape="record"];

      hooks -> canvas;
      subgraph cluster_tosvg {
        label = "SVG string rendering [0.2ms]"
        subgraph g_svg {
          rank=same;
          canvas -> render_to_svg [label="0.1ms"]
          render_to_svg -> stringify_svg [label="0.1ms"]
        }
      }
      subgraph cluster_rasterize {
        label = "Encode frame [167ms]"
        subgraph g_rasterize {
          rank=same;
          stringify_svg -> "svg string"
          "svg string" -> "usvg tree" [label="48ms"]
          "usvg tree" -> pixmap [label="11ms"]
          pixmap -> "hwc frame" [label="108ms"]
        }
      }

      canvas -> "svg string" [weight=10, style=invis]
    }
    ```
  ],
)

#figure(
  caption: "Durées d'exécution par tâche, pour une vidéo de test de 5 secondes",
  table(
    columns: 3,
    inset: 0.75em,
    [*Tâche*], [*Durée [ms]*], [*\#*],
    ..csv("../results.csv").slice(1).flatten()
  ),
)

== Rastérisation parallèle <perf-parallelrasterize>

Si la partie `render_to_svg` n'est pas parallélisable car il faut bien faire exécuter tout les hooks dans l'ordre, la rastérisation des SVG sortants, elle, est bien parallélisable. Malheureusement, le gain de performance n'a pas été significatif.

== Encodage H.264 parallèle?

Si l'on est bien capable de donner à l'encodeur nos frames dans le désordre, tout en lui indiquant le timestamp de chaque frame, l'encodeur ne supporte pas de recevoir les frames dans le désordre:

#align(center)[

]

Il est donc impossible de paralléliser l'encodage

== Pixmap et frames HWC: 100ms de standards

L'encodage vidéo étant fait par une bibliothèque totalement séparée de celle s'occupant de la rastérisation SVG, il y a un risque d'incompatibilité entre les formats de pixmap utilisés par les deux bibliothèques, ce qui est le cas ici.

En effet, les SVG rasterisés sont stockées dans un array plat de valeurs RGBA @pixmapvecu8:

#align(center)[
  ```
  [R, G, B, A, R, G, B, A, …]
  ```
]

Tandis que la bibliothèque utilisée, _video-rs_, attend une matrice HWC, ou height-width-channels, de pixels RGB @videorshwc, @videorshcwframe, @array3rust:

#align(center)[
  ```
  [
    [ [R, G, B], [R, G, B], … ],
    [ [R, G, B], [R, G, B], … ],
    …
  ]
  ```
]

Il est donc nécéssaire de convertir entre ces deux formats, ce qui est lent car demande de copier les données.

Une solution serait de passer à une bibiothèque plus bas niveau et voir s'il est possible de donner directement les données de pixmap à l'encodeur, sans conversion, ou tout du moins sans avoir à copier les données.

Une autre solution est de faire proposer une contribution à la bibiothèque de rendu utilisée par _resvg_, _tiny_skia_#footnote[Tiny-skia est notamment utilisé par Typst @typsttinyskia @typsttinyskiacargotoml, l'alternative moderne à LaTeX sur laquelle ce papier a été typeset], pour pouvoir instrumentaliser les lectures et écritures à sa pixmap, et ainsi écrire dans la représentation voulue par libx264 directement.

== SVG vers string vers SVG <perf-svgstring>

Comme on peut le remarquer, il y a un gain de performance assez conséquent de possible si l'on parvient à utiliser usvg, non seulement pour la rastérisation, mais également pour la construction de l'arbre SVG: sur une boule de rendu de 167 ms, *on passe 29% du temps à parser un arbre SVG sérialisé, alors que l'on vient de construire cette arbre*.

= Conclusion


#show: arkheion-appendices
= Marqueurs dans un logiciel de MAO

#imagefigure(
  "./flstudiomarkers.png",
  [
    Marqueurs dans FL Studio:
    #smallcaps([intro end, block 1, break 1, buildup 1, …])
    #linebreak()
    Fichier de projet pour _Onset_ de Postamble @onset
  ],
) <flstudiomarkers>

// Add bibliography and create Bibiliography section
// #bibliography("bibliography.yaml", style: "./ieee-with-locations.csl")
#bibliography("bibliography.yaml")
