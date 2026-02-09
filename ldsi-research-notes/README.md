# LDSI Research Notes

Zettelkasten-style research notes for the Lyapunov-Dabert Stability Index project.

## Structure

- `YYYYMMDD-XX-slug.md` : Note atomique (une idee = une note)
- Tags : `#formula`, `#topology`, `#entropy`, `#ncd`, `#calibration`, `#injector`, `#benchmark`
- Links : `[[YYYYMMDD-XX-slug]]` pour les connexions inter-notes

## Index

### 2026-02-09 : Session de Hardening

| # | Note | Tags | Status |
|---|------|------|--------|
| 01 | [[20260209-01-elastic-window-topology]] | #topology #window #decay | Implemente |
| 02 | [[20260209-02-dynamic-stopwords-zipf]] | #cleaner #zipf #stopwords | Implemente |
| 03 | [[20260209-03-formula-entropy-shift]] | #formula #entropy #bugfix | Implemente |
| 04 | [[20260209-04-grid-search-results]] | #calibration #gridsearch | Complete |
| 05 | [[20260209-05-coefficient-unification]] | #bugfix #coefficients #cli | Implemente |
| 06 | [[20260209-06-injector-mock-tests]] | #injector #testing #wiremock | Implemente |
| 07 | [[20260209-07-benchmark-live]] | #benchmark #results | Complete |
| 08 | [[20260209-08-formula-ceiling-analysis]] | #formula #diagnostic #limitation | Analyse |
| 09 | [[20260209-09-horseshoe-theory]] | #topology #discovery #horseshoe | Decouverte |
| 10 | [[20260209-10-ncd-short-text-damping]] | #ncd #correction #shorttext | Design |
| 11 | [[20260209-11-formula-v030-design]] | #formula #design #v030 | Design |

### Graphe de Dependances

```
01-elastic-window ─────────────────────┐
02-dynamic-stopwords                    │
03-formula-entropy-shift ──► 04-grid-search ──► 08-ceiling-analysis
05-coefficient-unification                              │
06-injector-mock-tests                                  │
07-benchmark-live                                       ▼
                                              09-horseshoe-theory
                                              ┌────────┼────────┐
                                              ▼        ▼        ▼
                                      10-ncd-damping  11-v030  (implementation)
```

### Decouverte Cle : Theorie du Fer a Cheval

Le ZOMBIE et le FOU se ressemblent structurellement (densite elevee, SW > 0.8).
Seul l'ARCHITECTE a une structure "qui respire" (densite ~0.35, SW ~0.5-0.65).
Voir [[20260209-09-horseshoe-theory]] pour le detail.
