// Imports
#import "@preview/brilliant-cv:2.0.6": cvSection, cvEntry
#let metadata = toml("../metadata.toml")
#let cvSection = cvSection.with(metadata: metadata)
#let cvEntry = cvEntry.with(metadata: metadata)


#cvSection("Projects")

#cvEntry(
  title: [Solo Project],
  society: [Microweb],
  date: [],
  location: [],
  description: list(
    [Webserver containing my homepage, blog, sandbox environment and much more!],
    [Code available on #link("https://github.com/Microwonk/varcher")[GitHub]. Visit #link("https://nicolas-frey.com")[here]],
  ),
  tags: ("Rust", "Axum", "Postgres", "Leptos", "Tailwind"),
)

#cvEntry(
  title: [Solo Project],
  society: [Varcher],
  date: [],
  location: [],
  description: list(
    [Solo project to learn the Vulkan API, multiple renderers],
    [Code available on #link("https://github.com/Microwonk/microweb")[GitHub]],
  ),
  tags: ("C++", "Vulkan", "Rendering"),
)

// #cvEntry(
//   title: [Open Sourcerer],
//   society: [Various FOSS projects],
//   date: [],
//   location: [],
//   description: list(
//     [Fix bugs, implement new features and join discussion],
//     [Collaborate with other volunteers],
//   ),
// )
