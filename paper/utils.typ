#let cut-lines = (
  starts,
  ends,
  content,
  keep_delimiting: false,
) => {
  let lines = content.split(regex("\r?\n"))
  let predicate = pred => if type(pred) == str {
    it => it.trim() == str
  } else if type(pred) == function {
    pred
  } else if type(pred) == regex {
    it => it.find(pred) != none
  } else {
    panic("cut-between predicates must be strings or functions")
  }
  let start_index = lines.position(predicate(starts))

  if start_index == none {
    none
  } else {
    let lines_from_start = lines.slice(if keep_delimiting {
      start_index
    } else {
      calc.max(start_index + 1, 0)
    })

    lines_from_start
      .slice(
        0,
        lines_from_start.position(predicate(ends))
          + if keep_delimiting { 1 } else { 0 },
      )
      .join("\n")
  }
}

#let cut-between = (starts, ends, content) => cut-lines(
  starts,
  ends,
  content,
  keep_delimiting: false,
)
#let cut-around = (starts, ends, content) => cut-lines(
  starts,
  ends,
  content,
  keep_delimiting: true,
)


#let include-function = (
  filepath,
  name,
  lang: none,
  transform: it => it,
) => {
  let start_pattern = if lang == "rust" {
    regex("^pub fn " + name)
  } else if lang == "python" {
    regex("^def " + name)
  } else if lang == none {
    panic("specify a source language")
  } else {
    panic(lang + " is not supported for now. Use cut-between directly.")
  }

  let end_pattern = if lang == "rust" {
    regex("^\}")
  } else if lang == "python" {
    regex("^# end") // TODO pass next line to cut-between
  } else {
    none
  }

  let contents = cut-around(
    start_pattern,
    end_pattern,
    read(filepath),
  )

  if contents == none {
    [
      Woops! function #name not in #filepath .\_.
      Searched for a line beginning with #start_pattern in:

      #raw(lang: lang, read(filepath))
    ]
  } else {
    raw(lang: lang, transform(contents))
  }
}
