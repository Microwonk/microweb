// Imports
#import "@preview/brilliant-cv:2.0.6": cvSection, cvSkill, hBar
#let metadata = toml("../metadata.toml")
#let cvSection = cvSection.with(metadata: metadata)


#cvSection("Fähigkeiten")

#cvSkill(
  type: [Sprachen],
  info: [Deutsch (Muttersprache) #hBar() Englisch (C1)],
)

#cvSkill(
  type: [Technologie Stack],
  info: [Rust #hBar() Axum #hBar() Java #hBar() C\# #hBar() SQL #hBar() PHP #hBar() Javascript #hBar() Git #hBar() Linux #hBar() Docker],
)
