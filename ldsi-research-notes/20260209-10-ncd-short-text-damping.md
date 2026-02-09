# NCD Short-Text Damping (Correction Volatilite)

**Date** : 2026-02-09
**Tags** : #ncd #correction #formula #shorttext
**Status** : Design, implementation a faire

## Probleme

Le NCD brut sur textes courts (< 1KB) est volatil a cause du overhead fixe de zstd (dictionnaire, headers).

Cas illustratif :
```
text_a = "La temperature est de vingt-cinq degres aujourd'hui."
text_b = "La temperature est de 25 degres ce jour."
NCD = 0.525  ->  REBELLE (devrait etre ZOMBIE)
```

Ces textes sont semantiquement quasi-identiques. Mais le remplacement "vingt-cinq" -> "25" et "aujourd'hui" -> "ce jour" cause une divergence de compression disproportionnee sur 92 octets combines.

## Cause

Pour des textes courts, le ratio signal/bruit du NCD est mauvais :
- Header zstd ≈ 10-15 octets (fixe)
- Sur 50 octets, le header = 20-30% du signal
- Sur 5000 octets, le header = 0.2-0.3% du signal

Le `optimal_window_log()` dans ncd.rs corrige le probleme de fenetre, mais pas le probleme de overhead proportionnel.

## Solution Proposee

Facteur de damping logarithmique :

```rust
fn ncd_damping_factor(combined_size: usize) -> f64 {
    if combined_size >= 1024 {
        1.0
    } else {
        (combined_size as f64).ln() / (1024_f64).ln()
    }
}

// NCD_corrected = NCD_raw * damping_factor
```

### Comportement

| Taille combinee | Facteur | Effet |
|-----------------|---------|-------|
| 50 octets | 0.564 | NCD * 0.56 |
| 100 octets | 0.665 | NCD * 0.67 |
| 200 octets | 0.764 | NCD * 0.76 |
| 500 octets | 0.897 | NCD * 0.90 |
| 1024+ octets | 1.000 | Pas de correction |

### Impact sur le cas "Quasi-id"

```
NCD_raw = 0.525
combined_size = 92 octets
factor = ln(92)/ln(1024) = 0.652
NCD_corrected = 0.525 * 0.652 = 0.342
```

lambda_LD avec defaults (0.50/0.30/0.20) :
- Avant : 0.50*0.525 + 0.30*(-0.114) = 0.228 (ZOMBIE limite)
- Apres : 0.50*0.342 + 0.30*(-0.114) = 0.137 (ZOMBIE solide)

## Risques

- **Sous-estimation sur textes courts** : Un vrai texte divergent de 50 octets sera sous-evalue
- **Seuil 1KB arbitraire** : Pourrait etre trop agressif ou pas assez
- **Interaction avec structural_quality** : Le damping + SQ ensemble pourraient ecraser le signal FOU

## Alternatives

1. **Sigmoid** : `ncd_corrected = ncd * sigmoid(combined_size, k=200, midpoint=100)`
2. **Floor + linear** : `factor = max(0.5, combined_size / 1024.0)`
3. **Ne rien faire** : Accepter la volatilite, documenter la limitation

## Decision

A implementer avec le facteur logarithmique. Le seuil de 1KB est conservateur (au-dela, les textes sont assez longs pour que zstd soit fiable).

## Liens

- [[20260209-09-horseshoe-theory]] (decouverte en parallele)
- [[20260209-08-formula-ceiling-analysis]] (le diagnostic qui montre NCD quasi-id = 0.525)
- [[20260209-11-formula-v030-design]] (integration dans la nouvelle formule)
