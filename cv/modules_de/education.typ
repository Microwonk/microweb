// Imports
#import "@preview/brilliant-cv:2.0.6": cvSection, cvEntry, hBar
#let metadata = toml("../metadata.toml")
#let cvSection = cvSection.with(metadata: metadata)
#let cvEntry = cvEntry.with(metadata: metadata)


#cvSection("Abschlüsse")

#cvEntry(
  title: [Computer Wissenschaften and Software Development],
  society: [IT Kolleg Imst],
  date: [2022 - 2024],
  location: [Imst, Tirol],
  description: list(
    [Diplomarbeit: Entwicklung einer App für den österreichischen L/L-17 Führerschein],
    [Kurse: OOP und Software Architektur #hBar() Kryptografie und Blockchain #hBar() Netzwerktechnik, Datenzentren und Virtualisierung/Containerisierung],
  ),
)

#cvEntry(
  title: [Digitale Kunst and IT],
  society: [BORG Landeck],
  date: [2018 - 2022],
  location: [Landeck, Tirol],
  description: [Kurse: Digitale Kunst #hBar() Video- und Bildbearbeitung #hBar() Programmieren],
)
