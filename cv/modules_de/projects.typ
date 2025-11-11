// Imports
#import "@preview/brilliant-cv:2.0.6": cvSection, cvEntry
#let metadata = toml("../metadata.toml")
#let cvSection = cvSection.with(metadata: metadata)
#let cvEntry = cvEntry.with(metadata: metadata)


#cvSection("Projekte")

#cvEntry(
  title: [Solo Projekt],
  society: [Microweb],
  date: [],
  location: [],
  description: list(
    [Webserver mit meiner Homepage, meinem Blog, Sandbox-Umgebung und vielem mehr!],
    [Code verfügbar unter #link("https://github.com/Microwonk/varcher")[GitHub]. Besuche #link("https://nicolas-frey.com")[hier]],
  ),
  tags: ("Rust", "Axum", "Postgres", "Leptos", "Tailwind"),
)

#cvEntry(
  title: [Solo Projekt],
  society: [Varcher],
  date: [],
  location: [],
  description: list(
    [Solo-Projekt zum Erlernen der Vulkan-API, mehrere Renderer],
    [Code verfügbar unter #link("https://github.com/Microwonk/microweb")[GitHub]],
  ),
  tags: ("C++", "Vulkan", "Rendering"),
)