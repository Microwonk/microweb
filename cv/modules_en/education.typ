// Imports
#import "@preview/brilliant-cv:2.0.6": cvSection, cvEntry, hBar,
#let metadata = toml("../metadata.toml")
#let cvSection = cvSection.with(metadata: metadata)
#let cvEntry = cvEntry.with(metadata: metadata)


#cvSection("Education")

#cvEntry(
  title: [Computer Science and Software Development],
  society: [IT Kolleg Imst],
  date: [2022 - 2024],
  location: [Imst, Tyrol],
  description: list(
    [Thesis: Developing an app for the austrian L/L-17 driving license],
    [Course: OOP and Software Architecture #hBar() Cryptography and Blockchain #hBar() Networking, Datacenters and Virtualization/Containerization],
  ),
)

#cvEntry(
  title: [Digital Art and IT],
  society: [BORG Landeck],
  date: [2018 - 2022],
  location: [Landeck, Tyrol],
  description: [Course: Digital art #hBar() video and image editing #hBar() Programming],
)
