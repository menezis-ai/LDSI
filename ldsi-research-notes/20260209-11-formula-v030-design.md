# Design de la Formule v0.3.0

**Date** : 2026-02-09
**Tags** : #formula #design #v030 #architecture
**Status** : Design, implementation a faire

## Formule v0.2.0 (actuelle)

```
lambda_LD = alpha * NCD(A,B) + beta * (H(B)/H(A) - 1) + gamma * delta_topo(A,B)
```

Coefficients : alpha=0.50, beta=0.30, gamma=0.20

### Limitations documentees

1. **FOU indetectable** : lambda_LD plafonne a ~0.80 pour les cas FOU (score attendu > 1.2)
2. **delta_topo inutile** : Ecrase sur [-0.05, +0.28], ne discrimine pas
3. **NCD volatile** : Sur textes courts, overhead zstd fausse le ratio

Classification : **7/12 cas corrects (58%)**

## Formule v0.3.0 (proposee)

```
lambda_LD = alpha * NCD_corrected(A,B) + beta * (H(B)/H(A) - 1) + gamma * SQ(B)
```

### Changements

| Composante | v0.2.0 | v0.3.0 | Raison |
|------------|--------|--------|--------|
| NCD | Brut | Damped pour < 1KB | Volatilite textes courts |
| Entropie | (ratio - 1) clamp [-1, 2] | Inchange | Fonctionne bien |
| Topologie | delta_topo(A,B) | SQ(B) | Le signal est dans B, pas dans le delta |

### NCD_corrected

```rust
let factor = if combined_size >= 1024 { 1.0 }
             else { (combined_size as f64).ln() / 1024_f64.ln() };
ncd_corrected = ncd_raw * factor;
```

### SQ(B) - Structural Quality

```rust
let density_score = (-((density_b - 0.35) / 0.15).powi(2)).exp();
let sw_penalty = if sw_idx > 0.8 { (1.0 - (sw_idx - 0.8) * 2.0).max(0.0) } else { 1.0 };
let sq = density_score * sw_penalty;
```

Basee sur la Theorie du Fer a Cheval Topologique ([[20260209-09-horseshoe-theory]]) :
- Gaussienne centree sur densite = 0.35 (zone ARCHITECTE)
- Penalite lineaire si SW > 0.8 (zone ZOMBIE/FOU)
- SQ in [0, 1]

## Projections avec Defaults (0.50/0.30/0.20)

| Cas | Attendu | v0.2.0 | v0.3.0 (projete) | Verdict v0.3.0 |
|-----|---------|--------|-------------------|----------------|
| ZOMBIE Identique | 0.05 | 0.08 | ~0.05 | ZOMBIE |
| ZOMBIE Quasi-id | 0.15 | 0.23 | ~0.18 | ZOMBIE |
| ZOMBIE Ajout marg | 0.20 | 0.15 | ~0.15 | ZOMBIE |
| REBELLE Paraphrase | 0.55 | 0.56 | ~0.49 | REBELLE |
| REBELLE Precision | 0.50 | 0.71 | ~0.68 | REBELLE |
| REBELLE Scientif | 0.65 | 0.54 | ~0.58 | REBELLE |
| ARCHIT Metaphore | 0.90 | 1.00 | ~1.09 | ARCHITECTE |
| ARCHIT Philosophie | 0.95 | 0.61 | ~0.72 | ARCHITECTE |
| ARCHIT Poetique | 1.00 | 0.80 | ~0.92 | ARCHITECTE |
| FOU Hallucination | 1.40 | 0.80 | ~0.68 | REBELLE |
| FOU Word salad | 1.50 | 0.72 | ~0.66 | REBELLE |
| FOU Pseudo-sci | 1.30 | 0.75 | ~0.72 | ARCHITECTE/REBELLE |

### Classification projetee

| Verdict | v0.2.0 (correct) | v0.3.0 (projete) |
|---------|-----------------|-------------------|
| ZOMBIE | 3/3 | 3/3 |
| REBELLE | 2/3 | 3/3 (ameliore) |
| ARCHITECTE | 2/3 | 2-3/3 (ameliore) |
| FOU | 0/3 | 0/3 (classe REBELLE au lieu d'ARCHITECTE) |

**Score projete : 8-9/12 (67-75%)** vs 7/12 (58%) avant.

## Probleme Restant : FOU

La formule v0.3.0 ameliore la discrimination ARCHITECTE/FOU mais ne peut toujours pas classer FOU correctement car :
1. NCD_corrected plafonne a ~0.75 pour les textes courts
2. Entropy shift est similaire pour ARCHITECTE et FOU
3. SQ(FOU) est bas mais gamma=0.20 ne suffit pas a inverser le signal

### Options pour v0.4.0

1. **Inter-Sentence Coherence (ISC)** : NCD entre phrases consecutives de B
   - Texte coherent : NCD inter-phrase modere (0.4-0.7)
   - Word salad : NCD inter-phrase eleve (0.8+)
   - Necessite des textes multi-phrases (sorties LLM reelles)

2. **Augmenter gamma** : Si le grid search le confirme, gamma pourrait passer a 0.40+
   - Risque : sur-pondere la topologie qui est instable pour textes < 20 mots

3. **Reformuler les seuils** : Accepter que FOU ne produit pas lambda > 1.2
   - ZOMBIE < 0.2, REBELLE 0.2-0.5, ARCHITECTE 0.5-0.8, FOU > 0.8 ?
   - Probleme : l'ARCHITECTE actuel produit 0.72-1.09, chevauche FOU

## Decision

Implementer v0.3.0 (SQ + NCD damping), runner le grid search, evaluer.
Le FOU reste un probleme ouvert qui necessite soit des textes plus longs, soit une 4eme metrique (ISC).

## Liens

- [[20260209-09-horseshoe-theory]] (la decouverte fondamentale)
- [[20260209-10-ncd-short-text-damping]] (detail du damping NCD)
- [[20260209-08-formula-ceiling-analysis]] (l'analyse qui a motive ce redesign)
- [[20260209-03-formula-entropy-shift]] (le fix entropy qui a precede)
