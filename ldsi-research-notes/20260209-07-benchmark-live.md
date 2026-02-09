# Benchmark Live : Standard vs Fracture

**Date** : 2026-02-09
**Tags** : #benchmark #results #architecte
**Status** : Execute, single datapoint

## Setup

- Binary : `ldsi analyze` (release build)
- Coefficients : 0.40/0.35/0.25 (pre-unification, CLI defaults de l'epoque)
- Pas de LLM live (Ollama n'avait que nomic-embed-text). Textes rediges manuellement.

## Prompts

**Standard (A)** : Definition encyclopedique de l'amour. Ton neutre, vocabulaire standard, structure lineaire.

**Fracture (B)** : "L'amour est un algorithme de tri corrompu." Metaphore computationnelle etendue : bubble sort, buffer overflow, zero-day exploit. Vocabulaire technique dense, ponts semantiques code/emotion.

## Resultats

```json
{
  "lambda": 0.8462,
  "verdict": "Architecte",
  "ncd": { "score": 0.8432 },
  "entropy": { "ratio": 1.1033 },
  "topology": { "delta": 0.4910 }
}
```

**Temps d'execution** : 22ms

## Analyse

- **NCD 0.843** : Divergence informationnelle massive. Les textes partagent peu de structure compressible.
- **H(B)/H(A) 1.103** : +10% d'entropie. Vocabulaire riche, pas du bruit.
- **delta_topo 0.491** : LCC = 1.0 des deux cotes (connexe). Clustering 0.80 -> 0.77 (legere baisse). Structure preservee.
- **Densite** : 0.257 -> 0.193. Graphe plus large (plus de noeuds uniques) mais moins dense = vocabulaire enrichi structure.

## Limites

- Single datapoint. Pas de validation statistique.
- Coefficients non optimaux (pre-unification).
- Textes rediges, pas generes par LLM.

## Liens

- [[20260209-04-grid-search-results]] (calibration necessaire pour valider les seuils)
- [[20260209-03-formula-entropy-shift]] (la formule a change depuis ce benchmark)
