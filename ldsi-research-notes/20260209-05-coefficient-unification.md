# Unification des Coefficients CLI vs Lib

**Date** : 2026-02-09
**Tags** : #bugfix #coefficients #cli
**Status** : Implemente, teste

## Probleme

Deux sources de verite pour les coefficients par defaut :

| Source | alpha | beta | gamma |
|--------|-------|------|-------|
| `LdsiCoefficients::default()` (lib) | 0.50 | 0.30 | 0.20 |
| CLI `--alpha/--beta/--gamma` defaults | 0.40 | 0.35 | 0.25 |

La commande `ldsi analyze` sans flags utilisait 0.40/0.35/0.25.
La commande `ldsi inject` (via `compute_ldsi(_, _, None)`) utilisait 0.50/0.30/0.20.

Meme outil, resultats differents. Bug.

## Fix

- `main.rs` : `default_value` aligne sur 0.50/0.30/0.20
- `CLAUDE.md` : documentation mise a jour, "single source of truth" documente
- Reference "historique" (0.40/0.35/0.25) supprimee

## Principe

La lib est la source de verite. Le CLI l'importe, ne la redefinit pas.

## Liens

- [[20260209-04-grid-search-results]] (les coefficients pourraient changer apres re-calibration)
