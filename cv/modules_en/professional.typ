// Imports
#import "@preview/brilliant-cv:2.0.6": cvSection, cvEntry, cvEntryStart, cvEntryContinued, 
#let metadata = toml("../metadata.toml")
#let cvSection = cvSection.with(metadata: metadata)
#let cvEntry = cvEntry.with(metadata: metadata)
#let cvEntryStart = cvEntryStart.with(metadata: metadata)
#let cvEntryContinued = cvEntryContinued.with(metadata: metadata)


#cvSection("Professional Experience")

#cvEntry(
  society: [Proxmox Server Solutions GmbH],
  logo: image("../src/logos/proxmox.svg"),
  location: [Vienna, Austria],
  date: [since Sep. 2025],
  title: [Rust Software Developer],
  description: list(
    [Expand on existing products by fixing bugs, and implementing new features],
    [Create internal tooling and lead new projects],
  ),
  tags: ("Rust", "Qemu", "LXC", "Hyper", "Tokio", "Debian", "Perl", "Javascript"),
)

#cvEntry(
  title: [Software Developer],
  society: [ICOTEC GmbH],
  date: [Dec. 2024 - Jun. 2025],
  location: [Graz, Styria],
  description: list(
    [Developed and optimized backend systems for the biggest bank in Austria],
    [Implemented real-time application to display the results of the 2025 Ski World Championship],
  ),
  tags: ("Rust", "C#", ".NET", "ASP.NET", "SQL", "Javascript"),
)

#cvEntry(
  title: [Software Developer Intern],
  society: [Cookis GmbH],
  logo: image("../src/logos/cookis.svg"),
  date: list(
    [Summer 2024],
    [Summer 2023],
  ),
  location: [Imst, Tyrol],
  description: list(
    [Implemented a Laravel + Vue application for LLM-driven translation],
    [Assisted with data cleaning and record completion],
  ),
  tags: ("MySQL", "PHP", "Javascript", "Vue", "Laravel"),
)
