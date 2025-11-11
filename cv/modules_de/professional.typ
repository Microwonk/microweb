// Imports
#import "@preview/brilliant-cv:2.0.6": cvSection, cvEntry
#let metadata = toml("../metadata.toml")
#let cvSection = cvSection.with(metadata: metadata)
#let cvEntry = cvEntry.with(metadata: metadata)


#cvSection("Berufliche Erfahrung")

#cvEntry(
  society: [Proxmox Server Solutions GmbH],
  logo: image("../src/logos/proxmox.svg"),
  location: [Wien, Österreich],
  date: [seit Sep. 2025],
  title: [Rust Software Entwickler],
  description: list(
    [Erweitern bestehender Produkte durch Bugfixes und Implementierung neuer Funktionen],
    [Interne Werkzeuge erstellen und neue Projekte leiten],
  ),
  tags: ("Rust", "Qemu", "LXC", "Hyper", "Tokio", "Debian", "Perl", "Javascript"),
)

#cvEntry(
  title: [Software Entwickler],
  society: [ICOTEC GmbH],
  date: [Dez. 2024 - Jun. 2025],
  location: [Graz, Steiermark],
  description: list(
    [Entwicklung und Optimierung von Backend-Systemen für die größte Bank Österreichs],
    [Implementierte Echtzeitanwendung zur Anzeige der Ergebnisse der Ski-Weltmeisterschaft 2025],
  ),
  tags: ("Rust", "C#", ".NET", "ASP.NET", "SQL", "Javascript"),
)

#cvEntry(
  title: [Software Entwickler Praktikum],
  society: [Cookis GmbH],
  logo: image("../src/logos/cookis.svg"),
  date: list(
    [Sommer 2024],
    [Sommer 2023],
  ),
  location: [Imst, Tirol],
  description: list(
    [Implementierung einer Laravel + Vue-Anwendung für LLM-gesteuerte Übersetzung],
    [Unterstützung bei der Datenbereinigung und Vervollständigung von Datensätzen],
  ),
  tags: ("MySQL", "PHP", "Javascript", "Vue", "Laravel"),
)

