# ProductManager

*[English](#english) — [Français](#français)*

---

## English

A desktop application to manage product **expiration dates** and **promotions** by barcode. Written in Rust, it is used in production in a shop.

The starting problem: spotting products nearing their expiration date in time, and keeping track of promotion periods, without doing it by hand. ProductManager brings both into a single interface and sends automatic notifications before a product expires or a promotion ends.

> **Compatibility note:** the product catalogue import currently supports **LGPI exports only**. Other catalogue formats are not handled yet.

### Features

- **Two trackers in one**: a tab for expirations, a tab for promotions, backed by the same database.
- **Barcode entry** (13-digit EAN) with quantity and date.
- **Automatic product name resolution** from a CSV catalogue (barcode → label).
- **Geographic zone support**: each product's storage/geographic zone is read from the catalogue and shown in the list and printed listings.
- **Live search** by code or by product name.
- **Filters**: by deadline (1 month / 3 months / all) and by date range.
- **Urgency coloring**: red under 7 days, orange under 30.
- **Automatic notifications** through a scheduled checker: alerts for products expiring within 30 days and promotions ending the next day.
- **Print / export** of the listing: generates an HTML page opened in the browser, ready to print or save as PDF.

### Tech stack

- **[Rust](https://www.rust-lang.org/)** (2024 edition)
- **[iced](https://iced.rs/)** — graphical interface
- **[rusqlite](https://github.com/rusqlite/rusqlite)** / SQLite — local storage (single database, two tables)
- **[chrono](https://github.com/chronotope/chrono)** — deadline calculations
- **[notify-rust](https://github.com/hoodie/notify-rust)** — desktop notifications
- **[anyhow](https://github.com/dtolnay/anyhow)** — error handling

### Architecture

The project separates business logic, interface and checker into three independent parts:

```
src/
├── main.rs              Graphical interface (iced) — main binary
├── lib.rs               Library entry point, re-exports the modules
├── bin/checker.rs       Standalone checker — sends the notifications
├── expiration/          Expiration logic
│   ├── opendb.rs        Open / create the database
│   ├── writedb.rs       Write a product
│   ├── sort.rs          Read sorted by date
│   ├── remove.rs        Delete
│   ├── input.rs         Validate and convert dates (DD/MM/YYYY)
│   └── encoding.rs      Load the CSV catalogue, resolve code → (name, zone)
├── promotion/           Promotion logic (same pattern)
│   ├── writepromo.rs
│   ├── sortpromo.rs
│   └── removepromo.rs
└── print/               Printable listing generation
    └── htmlmaker.rs
```

The library (database logic) does not depend on the interface, which lets the checker reuse it without pulling in iced.

### Installation

Requires [Rust](https://www.rust-lang.org/tools/install) (2024 edition).

```bash
git clone https://github.com/Delta-Azura/productmanager.git
cd productmanager
cargo build --release
```

Binaries are produced in `target/release/`:
- `ProductManager` — the graphical application
- `checker` — the notification checker

### Usage

Run the interface:

```bash
cargo run --bin ProductManager
```

Run the checker manually (it notifies upcoming deadlines):

```bash
cargo run --bin checker
```

#### Automatic notifications

The checker is meant to be run once a day by the system scheduler.

**Windows** (Task Scheduler):

```
schtasks /create /tn "ProductManager" /tr "C:\path\to\checker.exe" /sc daily /st 09:00
```

**Linux** (systemd, user service) — create a timer triggering `checker` daily.

#### Product catalogue

Barcode → name resolution relies on an **LGPI CSV export** (`;` separator, Windows-1252 encoding) containing the columns `Code produit`, `Désignation` and `Zone Géo.`. Only the LGPI format is supported for now.

### License

Distributed under the **GNU General Public License v2**. See the [LICENSE](LICENSE) file.

---

## Français

Application de bureau pour gérer les **dates de péremption** et les **promotions** des produits d'un commerce, à partir de leur code-barres. Écrite en Rust, elle est utilisée en production dans un magasin.

Le problème de départ : repérer à temps les produits qui approchent de leur date de péremption, et suivre les périodes de promotion, sans le faire à la main. ProductManager centralise ces deux suivis dans une seule interface et envoie des notifications automatiques avant qu'un produit ne périme ou qu'une promotion ne se termine.

> **Compatibilité :** l'import du catalogue produits ne prend en charge que les **exports LGPI** pour l'instant. Les autres formats de catalogue ne sont pas encore gérés.

### Fonctionnalités

- **Deux suivis en un** : un onglet pour les péremptions, un onglet pour les promotions, sur la même base de données.
- **Saisie par code-barres** (EAN 13 chiffres) avec quantité et date.
- **Résolution automatique du nom du produit** à partir d'un catalogue CSV (code-barres → désignation).
- **Support des zones géographiques** : la zone de stockage / zone géographique de chaque produit est lue depuis le catalogue et affichée dans la liste et les listings imprimés.
- **Recherche** par code ou par nom de produit, en direct.
- **Filtres** : par échéance (1 mois / 3 mois / tout) et par intervalle de dates.
- **Coloration par urgence** : rouge à moins de 7 jours, orange à moins de 30.
- **Notifications automatiques** via un vérificateur planifié : alerte pour les produits périmant sous 30 jours et les promotions se terminant le lendemain.
- **Impression / export** du listing : génération d'une page HTML ouverte dans le navigateur, prête à imprimer ou à enregistrer en PDF.

### Stack technique

- **[Rust](https://www.rust-lang.org/)** (édition 2024)
- **[iced](https://iced.rs/)** — interface graphique
- **[rusqlite](https://github.com/rusqlite/rusqlite)** / SQLite — stockage local (base unique, deux tables)
- **[chrono](https://github.com/chronotope/chrono)** — calcul des échéances
- **[notify-rust](https://github.com/hoodie/notify-rust)** — notifications de bureau
- **[anyhow](https://github.com/dtolnay/anyhow)** — gestion d'erreurs

### Architecture

Le projet sépare la logique métier, l'interface et le vérificateur en trois parties indépendantes :

```
src/
├── main.rs              Interface graphique (iced) — binaire principal
├── lib.rs               Point d'entrée de la bibliothèque, ré-exporte les modules
├── bin/checker.rs       Vérificateur autonome — envoie les notifications
├── expiration/          Logique des péremptions
│   ├── opendb.rs        Ouverture / création de la base
│   ├── writedb.rs       Écriture d'un produit
│   ├── sort.rs          Lecture triée par date
│   ├── remove.rs        Suppression
│   ├── input.rs         Validation et conversion des dates (JJ/MM/AAAA)
│   └── encoding.rs      Chargement du catalogue CSV, résolution code → (nom, zone)
├── promotion/           Logique des promotions (même modèle)
│   ├── writepromo.rs
│   ├── sortpromo.rs
│   └── removepromo.rs
└── print/               Génération du listing imprimable
    └── htmlmaker.rs
```

La bibliothèque (logique base de données) ne dépend pas de l'interface, ce qui permet au vérificateur de la réutiliser sans embarquer iced.

### Installation

Nécessite [Rust](https://www.rust-lang.org/tools/install) (édition 2024).

```bash
git clone https://github.com/Delta-Azura/productmanager.git
cd productmanager
cargo build --release
```

Les binaires sont générés dans `target/release/` :
- `ProductManager` — l'application graphique
- `checker` — le vérificateur de notifications

### Utilisation

Lancer l'interface :

```bash
cargo run --bin ProductManager
```

Lancer le vérificateur manuellement (il notifie les échéances proches) :

```bash
cargo run --bin checker
```

#### Notifications automatiques

Le vérificateur est conçu pour être lancé une fois par jour par le planificateur du système.

**Windows** (planificateur de tâches) :

```
schtasks /create /tn "ProductManager" /tr "C:\chemin\vers\checker.exe" /sc daily /st 09:00
```

**Linux** (systemd, service utilisateur) — créer un timer déclenchant `checker` quotidiennement.

#### Catalogue de produits

La résolution code-barres → nom s'appuie sur un **export CSV LGPI** (séparateur `;`, encodage Windows-1252) contenant les colonnes `Code produit`, `Désignation` et `Zone Géo.`. Seul le format LGPI est pris en charge pour l'instant.

### Licence

Distribué sous licence **GNU General Public License v2**. Voir le fichier [LICENSE](LICENSE).