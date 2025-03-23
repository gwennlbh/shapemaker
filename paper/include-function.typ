#let include-function = (
    filepath,
    name,
    lang: "rust",
    transform: it => it
) => {
    let lines = read(filepath).split(regex("\r?\n"))
    let pattern = regex("^pub fn " + name)
    let function_start_index = lines.position(
        it => it.starts-with(pattern) 
    )
    
    if function_start_index == none {
        [
            Woops! function #name not in #filepath .\_.
            Searched for a line beginning with #pattern in:

            #raw(lang: lang, lines.join("\n"))
        ]
    } else {
        let lines_from_function = lines.slice(function_start_index)
        raw(
            lang: lang,
            transform(
                lines_from_function.slice(
                    0, 
                    lines_from_function.position(it => it == "}")
                ).join("\n") + "\n}"
            )
        )
    }
}
