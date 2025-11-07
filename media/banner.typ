#import "lib.typ": *

#set page(
  width: 1073pt,
  height: 151pt,
  margin: 0em,
  fill: none,
  background: box(
    width: 100%,
    height: 100%,
    fill: rusty-red,
    radius: 10%,
  ),
)
#set text(
  font: "JetBrains Mono",
  size: 120pt,
  fill: darker,
  weight: 700,
  spacing: 10pt,
)
#set align(center + horizon)
#let sm = text.with(size: 0.4em, weight: 300)

#stack(
  dir: ltr,
  spacing: 40pt,
  image(bytes(directory-bytes), height: 60%),
  [ST#sm[atic] I#sm[ndex] L#sm[isting]],
  image(bytes(directory-bytes), height: 60%),
)
