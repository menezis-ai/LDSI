# Fenetre Elastique avec Decroissance Inverse

**Date** : 2026-02-09
**Tags** : #topology #formula #improvement
**Status** : Implemente, teste, merge-ready

## Contexte

`WINDOW_SIZE = 5` etait une constante arbitraire dans `core/topology.rs`. Les aretes de co-occurrence avaient toutes un poids uniforme (`u32 = 1`), ignorant la distance entre tokens.

## Changement

- `WINDOW_SIZE = 5` remplace par `MAX_WINDOW = 15`
- Type d'arete : `DiGraph<String, u32>` -> `DiGraph<String, f64>`
- Poids d'arete : `weight = 1.0 / (distance as f64 + 1.0)`
  - d=1 (adjacent) : w=0.50
  - d=5 : w=0.167
  - d=14 : w=0.0625

## Justification

- La coherence locale n'est pas fixe (3 mots vs 20 mots pour une idee)
- Un window trop large (>20) cree un graphe quasi-complet, perte de pouvoir discriminant
- La decroissance inverse est la solution la plus simple sans NLP lourd
- Les associations long-range sont capturees mais faiblement ponderees

## Impact

- Densite graphe augmente (~0.10 -> ~0.29 pour 40 mots uniques) mais reste < 0.5
- 117 tests verts, aucune regression
- Nouveau test : `test_decay_weighting` valide w_close > w_far

## Liens

- [[20260209-03-formula-entropy-shift]] (topology baseline +0.5 aussi modifie)
- [[20260209-04-grid-search-results]] (impact sur calibration)
