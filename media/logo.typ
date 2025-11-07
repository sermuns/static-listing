#let rusty-red = rgb(228, 58, 37)

#set page(
  height: 1em,
  width: 1em,
  margin: 0em,
  fill: rusty-red,
)
#set text(font: "Libertinus Sans")
#set place(center + horizon)

#place(dx: 1.4pt, dy: -1pt, image("directory.svg", height: 5.5pt))
#place(dx: -2pt, text(rusty-red.desaturate(50%).lighten(70%))[L])
