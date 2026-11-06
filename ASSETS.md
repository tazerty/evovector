# Asset attribution

Every non-code asset in this repository (icons, images, fonts, example art, colour profiles, presets) is listed here with its author, source and licence. `cargo xtask assets` (part of `cargo xtask ci`) fails if an asset file is missing from this table.

**Policy (mandatory):** EvoVector contains **no Adobe iconography, images, artwork, presets, swatch/brush/symbol/pattern libraries or ICC profiles**. Every asset is either original work by EvoVector contributors, or third-party material under an open licence (OSI open source, public domain / CC0, or Creative Commons that allows redistribution), and it is attributed below. Screenshots of Adobe software are never committed. The one exception is the Upstream name, wordmark and logos in `docs/brand/`: Upstream trademarks, not open source, usable only under [`docs/brand/LICENSE-brand.txt`](docs/brand/LICENSE-brand.txt).

Generated-in-code art is original and has no file to list. This covers the default swatches, brushes, symbols, patterns, graphic styles, image-trace presets and the vector tool cursors (`crates/ui-egui/src/cursors.rs`).

**Font files are never committed here.** Fonts shared by the Building Apps live in [tazerty/evo-fonts](https://github.com/tazerty/evo-fonts) (rules: evorules [`standards/fonts.md`](amont/evo-rules/blob/main/standards/fonts.md)). The small Latin UI/document fonts below predate that rule and stay.

**Optional build input: evo-fonts.** Builds made with `EVO_FONTS_DIR=<evo-fonts checkout>` (all official releases) embed the Japanese fonts listed in its [`ATTRIBUTION.md`](https://github.com/tazerty/evo-fonts/blob/main/ATTRIBUTION.md): BIZ UDPGothic Regular and Bold, Shippori Mincho Regular and BIZ UDMincho Regular (web builds: BIZ UDPGothic Regular only), all under the SIL Open Font License 1.1. They are not files in this repository; release packages carry each one's `OFL-<family>.txt`. Builds without `EVO_FONTS_DIR` embed none of them.

| Asset | Author | Source | Licence | Notes |
|---|---|---|---|---|
| `docs/brand/upstream-logo-white.png` | Upstream team (project owner) | https://evora.st | Upstream trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/upstream-logo-white.svg` | Upstream team (project owner) | https://evora.st | Upstream trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/upstream-logo.png` | Upstream team (project owner) | https://evora.st | Upstream trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/upstream-logo.svg` | Upstream team (project owner) | https://evora.st | Upstream trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/upstream-mark-black.png` | Upstream team (project owner) | https://evora.st | Upstream trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/upstream-mark-black.svg` | Upstream team (project owner) | https://evora.st | Upstream trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/upstream-mark.png` | Upstream team (project owner) | https://evora.st | Upstream trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `docs/brand/upstream-mark.svg` | Upstream team (project owner) | https://evora.st | Upstream trademark, see docs/brand/LICENSE-brand.txt (not open source) | Used in README only; not shipped in the app |
| `assets/evora/sinopia-pro-mark-128.png` | Evora (fork tazerty/evovector, patch EVORA-SIN-03 E1) | Original geometric mark drawn procedurally (indigo #4F46E5 rounded tile, white cubic curve with its anchors and handles), 128 px, 4×4 supersampled | MIT OR Apache-2.0 | Web brand mark (`crates/ui-egui/src/brand.rs`) in place of the app icon in Evora builds |
| `assets/app-icon/LICENSE.txt` | (licence/readme text) | EvoVector | MIT OR Apache-2.0 |  |
| `assets/app-icon/README.md` | (licence/readme text) | EvoVector | MIT OR Apache-2.0 |  |
| `assets/app-icon/hicolor/128x128/apps/ai.storyteller.evovector.png` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/16x16/apps/ai.storyteller.evovector.png` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/24x24/apps/ai.storyteller.evovector.png` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/256x256/apps/ai.storyteller.evovector.png` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/32x32/apps/ai.storyteller.evovector.png` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/48x48/apps/ai.storyteller.evovector.png` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/512x512/apps/ai.storyteller.evovector.png` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/64x64/apps/ai.storyteller.evovector.png` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/hicolor/scalable/apps/ai.storyteller.evovector.svg` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/evovector-1024.png` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/evovector-macos-512.png` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/evovector-small.svg` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/evovector.icns` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/evovector.ico` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `assets/app-icon/evovector.svg` | Project owner (Upstream team) | Original artwork drawn in Upstream for EvoVector, vectorised and rendered by `packaging/icons.sh` | MIT OR Apache-2.0 (`assets/app-icon/LICENSE.txt`) | App icon (engraved dragon) |
| `packaging/macos/dmg/background.svg` | Xus Badia (contributed in #493) | Original artwork for EvoVector's DMG window; links the app icon `assets/app-icon/evovector-small.svg` | MIT OR Apache-2.0 (licensed by the contributor under the project licence) | DMG background source. Text in Inter and JetBrains Mono (OFL-1.1, `assets/fonts`), not embedded; paper grain generated by an SVG `feTurbulence` filter |
| `packaging/macos/dmg/background.tiff` | Xus Badia (contributed in #493) | Rendered from `packaging/macos/dmg/background.svg` by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 | 1x + 2x DMG background; includes the app icon (see its rows), text rendered from OFL-1.1 fonts and LittleCMS's built-in sRGB profile ("No copyright, use freely") |
| `packaging/macos/dmg/DS_Store` | Xus Badia (layout, #493) | Written by `packaging/macos/dmg/generate.py` | MIT OR Apache-2.0 | Finder view settings for the DMG (window, icon size and positions, background alias); no machine data |
| `assets/fonts/Inter-Medium.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/Inter-Regular.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/Inter-SemiBold.ttf` | Rasmus Andersson / The Inter Project Authors | https://github.com/rsms/inter | OFL-1.1 (`assets/fonts/OFL-Inter.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/JetBrainsMono-Regular.ttf` | The JetBrains Mono Project Authors | https://github.com/JetBrains/JetBrainsMono | OFL-1.1 (`assets/fonts/OFL-JetBrainsMono.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Bold.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-It.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Regular.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSans3-Semibold.ttf` | Paul D. Hunt / Adobe (released as open source) | https://github.com/adobe-fonts/source-sans | OFL-1.1 (`assets/fonts/OFL-SourceSans3.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/SourceSerif4-Regular.ttf` | Frank Grießhammer / Adobe (released as open source) | https://github.com/adobe-fonts/source-serif | OFL-1.1 (`assets/fonts/OFL-SourceSerif4.txt`) | Open-source typeface, not product iconography or artwork |
| `assets/fonts/OFL-Inter.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-JetBrainsMono.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-SourceSans3.txt` | (licence text) | upstream project | — |  |
| `assets/fonts/OFL-SourceSerif4.txt` | (licence text) | upstream project | — |  |
| `assets/icons/LICENSE-lucide.txt` | (licence text) | upstream project | — |  |
| `assets/icons/align-center-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-center-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-end-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-end-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-center.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-end.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-horizontal-justify-start.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-start-horizontal.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-start-vertical.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-center.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-end.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/align-vertical-justify-start.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/arrow-left-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/arrow-up-down.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/blend.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/brush.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chart-column.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-down.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-left.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevron-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevrons-left.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/chevrons-right.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/circle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/clipboard-paste.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/cloud.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/combine.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/copy.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/dc-actions.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-bottom.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-center.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-hcenter.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-left.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-right.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-top.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-al-vcenter.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-align.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-artboard.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-key.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-alignto-selection.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-anchor.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-anchor-smooth.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-appearance.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arc.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-down.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-extend.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-fill.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 | The Selection and Direct Selection arrows' fill, painted black or white under their outline |
| `assets/icons/dc-arrow-tip.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-arrow-up.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-artboard-options.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-artboards.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-blend.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-bloat.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-butt.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-round.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cap-square.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-center-hide.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-center-show.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-clear.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-color-guide.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-crystallize.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-cube.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dash-align.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dash-exact.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dir-off.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dir-on.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-direct.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-bottom.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-hcenter.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-hspace.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-left.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-right.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-top.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-vcenter.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-dist-vspace.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-behind.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-inside.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-draw-normal.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-ellipse.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-fill-none.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-flare.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-folder.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-free-transform.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-fx.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-gamut.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-freeform.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-linear.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-radial.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-across.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-along.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grad-stroke-within.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-gradient.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-graphic-styles.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-grid-view.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-group-select.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-bevel.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-miter.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join-round.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-join.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-knife.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-line-cut.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-line.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-list-view.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-live-bucket.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-live-select.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mask-none.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-measure.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mesh.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-mirror-cut.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-fill.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-item.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-new-stroke.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-options.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-center.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-all.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-center.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-left.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-justify-right.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-left.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-ltr.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-right.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-para-rtl.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-path-eraser.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pathfinder.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-perspective.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-crop.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-divide.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-exclude.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-intersect.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-merge.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-minus-back.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-minus-front.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-outline.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-trim.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pf-unite.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-place-symbol.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-polar-grid.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-polygon.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-pucker.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-puppet.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rearrange.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rect-cut.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rect-grid.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reference-point.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-remove-brush.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reshape.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-reverse.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rotate-view.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rounded-rect.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rule-evenodd.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-rule-nonzero.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-scallop.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-screen-mode.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-selection.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-shape-builder.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-shear.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-smooth.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-center.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-inside.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke-outside.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-stroke.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-swap.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-swatch-kinds.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-symbol-sprayer.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-touch-type.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-transform-panel.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-transparency.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-twirl.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-area.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-path.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-type-vertical.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-width-profile.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-width.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-wrinkle.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-zoom-large.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/dc-zoom-small.svg` | EvoVector contributors | Original work, drawn for EvoVector | MIT OR Apache-2.0 |  |
| `assets/icons/ellipsis.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eraser.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eye-off.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/eye.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/file-plus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/flip-horizontal-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/flip-vertical-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/folder-open.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/frame.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/grid-3x3.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/group.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/hand.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/hexagon.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/history.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/house.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/image.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/info.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/lasso.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/layers.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/layout-grid.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/library.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/link-2-off.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/link.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/external-link.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) | Help → app page link |
| `assets/icons/git-branch.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Help → GitHub link |
| `assets/icons/globe.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Help → website link |
| `assets/icons/message-circle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Discord button and Help → Discord link |
| `assets/icons/lock-open.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/lock.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/map.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/maximize-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/menu.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/minus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/monitor.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/move-diagonal-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/move.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/paint-bucket.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/paintbrush.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/palette.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pen-tool.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pen-tool-add.svg` | Lucide Icons and Contributors; badge by EvoVector contributors | https://lucide.dev (lucide-static v1.49.0 `pen-tool`; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Lucide's pen-tool with a plus badge |
| `assets/icons/pen-tool-delete.svg` | Lucide Icons and Contributors; badge by EvoVector contributors | https://lucide.dev (lucide-static v1.49.0 `pen-tool`; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) | Lucide's pen-tool with a minus badge |
| `assets/icons/pencil.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pilcrow.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pin.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/pipette.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/plus.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/printer.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/redo-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-3d.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-ccw.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/rotate-cw.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/ruler.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/save.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/scaling.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/scissors.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/search.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/settings.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/shapes.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/share-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/slice.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/sparkles.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spline-pointer.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spline.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/spray-can.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/square-dashed.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/square.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-exclude.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-intersect.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-subtract.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squares-unite.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/squircle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/star.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/sun.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/swatch-book.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/text-cursor-input.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/tornado.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/trash-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/triangle.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/type.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/undo-2.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/ungroup.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/wand-sparkles.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/waves.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/x.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `assets/icons/zoom-in.svg` | Lucide Icons and Contributors | https://lucide.dev (lucide-static v1.49.0; header kept in file) | ISC; derived from Feather, also MIT (`assets/icons/LICENSE-lucide.txt`) |  |
| `docs/images/dusk-poster.png` | EvoVector contributors | Rendered by EvoVector from `examples/dusk-poster.evovector` | MIT OR Apache-2.0 |  |
| `examples/dusk-poster.evovector` | EvoVector contributors | Original artwork built through the EvoVector command API | MIT OR Apache-2.0 |  |
| `examples/dusk-poster.svg` | EvoVector contributors | SVG export of the above | MIT OR Apache-2.0 |  |
| `docs/images/shot-1-neon.png` | EvoVector contributors | Screenshot of EvoVector itself (EvoVector/Lucide UI icons only), editing `examples/neon-drive.evovector` | MIT OR Apache-2.0 |  |
| `docs/images/shot-2-ribbons.png` | EvoVector contributors | Screenshot of EvoVector itself, editing `examples/ribbons.evovector` | MIT OR Apache-2.0 |  |
| `docs/images/shot-3-sheet.png` | EvoVector contributors | Screenshot of EvoVector itself, editing `examples/feature-sheet.evovector` | MIT OR Apache-2.0 |  |
| `docs/images/shot-4-bezier.png` | EvoVector contributors | Screenshot of EvoVector itself, editing `examples/feature-sheet.evovector` | MIT OR Apache-2.0 |  |
| `docs/images/shot-5-perspective.png` | EvoVector contributors | Screenshot of EvoVector itself, editing `examples/perspective-city.evovector` | MIT OR Apache-2.0 |  |
| `docs/images/art-neon-drive.png` | EvoVector contributors | Rendered by EvoVector from `examples/neon-drive.evovector` | MIT OR Apache-2.0 |  |
| `docs/images/art-ribbons.png` | EvoVector contributors | Rendered by EvoVector from `examples/ribbons.evovector` | MIT OR Apache-2.0 |  |
| `docs/images/art-pathfinder.png` | EvoVector contributors | Rendered by EvoVector from `examples/feature-sheet.evovector` (artboard 1) | MIT OR Apache-2.0 |  |
| `docs/images/art-mesh.png` | EvoVector contributors | Rendered by EvoVector from `examples/feature-sheet.evovector` (artboard 2) | MIT OR Apache-2.0 |  |
| `docs/images/art-repeat.png` | EvoVector contributors | Rendered by EvoVector from `examples/feature-sheet.evovector` (artboard 3) | MIT OR Apache-2.0 |  |
| `docs/images/art-envelope.png` | EvoVector contributors | Rendered by EvoVector from `examples/feature-sheet.evovector` (artboard 4) | MIT OR Apache-2.0 |  |
| `examples/neon-drive.evovector` | EvoVector contributors | Original artwork built through the EvoVector command API | MIT OR Apache-2.0 |  |
| `examples/ribbons.evovector` | EvoVector contributors | Original artwork built through the EvoVector command API | MIT OR Apache-2.0 |  |
| `examples/feature-sheet.evovector` | EvoVector contributors | Original artwork built through the EvoVector command API | MIT OR Apache-2.0 |  |
| `examples/perspective-city.evovector` | EvoVector contributors | Original artwork built through the EvoVector command API | MIT OR Apache-2.0 |  |
