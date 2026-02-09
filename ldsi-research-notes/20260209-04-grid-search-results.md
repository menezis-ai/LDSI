# Resultats du Grid Search (Pre et Post Formula Fix)

**Date** : 2026-02-09
**Tags** : #calibration #grid-search #coefficients
**Status** : Pre-fix execute, post-fix a lancer

## Grid Search Pre-Fix (ancienne formule)

**Dataset** : 12 cas (3 ZOMBIE, 3 REBELLE, 3 ARCHITECTE, 3 FOU)
**Pas** : 0.05 sur alpha, beta, gamma [0, 1]

### Resultat

```
Alpha (NCD)    : 0.90
Beta (Entropy) : 0.10
Gamma (Topo)   : 0.00
Error (SSE)    : 1.0882
```

### Interpretation

- **Topology eliminee** (gamma=0) : delta_topo ~= 0.5 partout (baseline +0.5), pas discriminant
- **NCD domine** (90%) : seule metrique avec une plage dynamique suffisante (0.17 -> 0.93)
- **Entropie marginale** (10%) : le ratio H(B)/H(A) avec plancher a 1.0 n'ajoute que du bruit

### Diagnostic detaille (defaults 0.50/0.30/0.20)

| Label | Expected | Got | NCD | H(B)/H(A) | dTopo | Gap |
|-------|----------|-----|-----|-----------|-------|-----|
| ZOMBIE-identique | 0.05 | 0.542 | 0.167 | 1.000 | 0.500 | +0.49 |
| REBELLE-paraphrase | 0.55 | 0.995 | 0.722 | 1.661 | 0.500 | +0.45 |
| ARCHI-philo | 0.95 | 1.052 | 0.747 | 1.821 | 0.463 | +0.10 |
| FOU-salade | 1.50 | 1.157 | 0.831 | 2.000 | 0.497 | -0.34 |

**Conclusion** : Formule surestime ZOMBIE/REBELLE, sous-estime FOU. Probleme structurel, pas de coefficients.

## Grid Search Post-Fix

A lancer apres validation des tests sur la formule corrigee.

## Liens

- [[20260209-03-formula-entropy-shift]] (les fixes appliques)
- [[20260209-05-coefficient-unification]] (prerequis : defaults unifies)
