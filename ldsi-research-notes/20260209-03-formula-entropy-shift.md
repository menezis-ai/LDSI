# Correction de la Formule : Entropy Shift et Topology Baseline

**Date** : 2026-02-09
**Tags** : #formula #entropy #topology #bugfix #calibration
**Status** : Implemente, tests en cours

## Probleme Identifie

Le diagnostic sur 12 cas de calibration a revele deux defauts structurels :

### 1. Plancher d'entropie

**Ancienne formule** : `beta * entropy_ratio.min(2.0)`

Pour textes identiques : H(B)/H(A) = 1.0, contribution = beta * 1.0 = 0.15.
Resultat : lambda minimum ~0.48 pour des textes identiques. Impossible d'atteindre ZOMBIE (< 0.3).

**Fix** : `beta * (entropy_ratio - 1.0).clamp(-1.0, 2.0)`

- ratio = 1.0 (identique) -> 0 de contribution
- ratio = 1.5 (vocabulaire enrichi) -> +0.5 * beta
- ratio = 0.7 (vocabulaire appauvri) -> -0.3 * beta (reduit lambda, zombie detecte)

### 2. Baseline topology +0.5

**Ancien** : `topology_delta` retournait `lcc_score * 0.5 + clustering_score * 0.3 + penalty + 0.5`

Le +0.5 faisait que delta_topo ~= 0.5 pour TOUS les cas (0.455 a 0.784 observe). L'optimiseur mettait gamma=0.00 car la metrique ne discriminait rien.

**Fix** : suppression du +0.5. Delta centre sur 0. Pas de changement structurel = 0 de contribution.

### 3. Plancher lambda

Ajout de `lambda.max(0.0)` car avec le shift negatif (ratio < 1) et topology negative, lambda pourrait devenir negatif, ce qui n'a pas de sens physique.

## Formule Corrigee

```
lambda_LD = alpha * NCD(A,B) + beta * clamp(H(B)/H(A) - 1, -1, 2) + gamma * delta_topo
lambda_LD = max(0, lambda_LD)
```

## Impact Attendu

| Cas | Ancien lambda | Nouveau lambda (estime) |
|-----|--------------|------------------------|
| Identique | 0.542 | ~0.084 |
| Paraphrase | 0.995 | ~0.559 |
| Architecte | 1.052 | ~0.719 |

Le spectre s'elargit vers le bas. Les seuils de verdict pourraient necessiter un re-calibrage.

## Liens

- [[20260209-04-grid-search-results]] (re-calibration necessaire)
- [[20260209-01-elastic-window-topology]] (topology aussi modifiee)
