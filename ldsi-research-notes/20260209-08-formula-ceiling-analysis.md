# Analyse du Plafond de la Formule lambda_LD

**Date** : 2026-02-09
**Tags** : #formula #diagnostic #limitation #gridsearch
**Status** : Analyse complete, action requise

## Constat

Le grid search post-fix (apres correction entropie shift + suppression baseline topologie) donne :

```
alpha = 0.85, beta = 0.25, gamma = 1.00
SSE = 0.8363, somme = 2.10
```

**Probleme** : La somme > 1.0 revele que l'optimiseur etire les coefficients pour essayer d'atteindre les cibles FOU (> 1.2), mais n'y arrive pas.

## Diagnostic par Categorie

### Defaults actuels (0.50/0.30/0.20)

| Cas | Attendu | Obtenu | Err | Verdict reel |
|-----|---------|--------|-----|-------------|
| Identique | 0.05 | 0.08 | +0.03 | ZOMBIE OK |
| Quasi-id | 0.15 | 0.23 | +0.08 | ZOMBIE OK |
| Ajout marg | 0.20 | 0.15 | -0.05 | ZOMBIE OK |
| Paraphrase | 0.55 | 0.56 | +0.01 | REBELLE OK |
| Precision | 0.50 | 0.71 | +0.21 | ARCHITECTE (devrait etre REBELLE) |
| Scientif | 0.65 | 0.54 | -0.11 | REBELLE OK |
| Metaphore | 0.90 | 1.00 | +0.10 | ARCHITECTE OK |
| Philosophie | 0.95 | 0.61 | -0.34 | REBELLE (devrait etre ARCHITECTE) |
| Poetique | 1.00 | 0.80 | -0.20 | ARCHITECTE OK |
| Hallucination | 1.40 | 0.80 | -0.60 | **ARCHITECTE** (devrait etre FOU) |
| Word salad | 1.50 | 0.72 | -0.78 | **ARCHITECTE** (devrait etre FOU) |
| Pseudo-sci | 1.30 | 0.75 | -0.55 | **ARCHITECTE** (devrait etre FOU) |

### Classification effective

| Verdict | Seuil | Cas corrects | Cas incorrects |
|---------|-------|-------------|---------------|
| ZOMBIE | < 0.3 | 3/3 | 0 |
| REBELLE | 0.3-0.7 | 2/3 | 1 ARCHIT mal classe |
| ARCHITECTE | 0.7-1.2 | 2/3 | 3 FOU classes ARCHITECTE |
| FOU | > 1.2 | 0/3 | Jamais atteint |

**Score de classification : 7/12 (58%)**

## Cause Racine

### 1. NCD sature

NCD pour textes divergents courts (< 50 mots) plafonne a ~0.93.
Pas de difference significative entre ARCHITECTE (0.75-0.93) et FOU (0.83-0.89).

### 2. Entropie ne discrimine pas FOU

`H(B)/H(A) - 1` pour :
- ARCHITECTE : +0.82 a +1.93
- FOU : +1.00 a +1.07

Les cas FOU ont un ratio proche de 1.0 (mots divers mais pas plus que la creativite structuree).
Le cas ARCHITECTE "metaphore" a un ratio de 1.93 — PLUS ELEVE que FOU !

### 3. Topologie ecrase sur zero

Delta topologique pour tous les cas non-triviaux : [-0.045, +0.284].
Le seul cas positif (hallucination, +0.284) est un artefact : text_a = "Bonjour." (graphe minuscule) vs text_b beaucoup plus riche.

**Topologie brute de text_b** :
```
ARCHITECTE (poetique)  : 34 noeuds, densite 0.336, clust 0.849, SW 0.519
FOU (word salad)       : 16 noeuds, densite 0.496, clust 0.992, SW 0.984
FOU (pseudo-sci)       : 18 noeuds, densite 0.494, clust 0.958, SW 0.846
```

Les textes FOU ont des graphes PLUS denses et PLUS clusters que ARCHITECTE.
C'est parce qu'avec 16-19 mots et window=15, presque tous les mots co-occurrent.
Le clustering trivial (graphe quasi-complet) mime une "bonne structure".

## Solution Proposee : NCD Inter-Phrase

### Concept

Mesurer la coherence interne de B par NCD entre phrases consecutives :

```rust
fn inter_sentence_coherence(text: &str) -> f64 {
    let sentences: Vec<&str> = text.split('.')
        .filter(|s| s.trim().len() > 10)
        .collect();
    if sentences.len() < 2 { return 0.5; } // Neutre pour mono-phrase

    let total: f64 = sentences.windows(2)
        .map(|w| ncd::compute_ncd(w[0], w[1]).score)
        .sum();
    total / (sentences.len() - 1) as f64
}
```

**Hypothese** :
- Texte coherent (ARCHITECTE) : NCD inter-phrase moderate (0.4-0.7), les phrases partagent du vocabulaire thematique
- Word salad (FOU) : NCD inter-phrase elevee (0.8+), aucun fil conducteur
- Texte identique (ZOMBIE) : NCD inter-phrase basse

### Limitation

La plupart des cas de test actuels sont mono-phrase.
Le metric n'est utile que pour des sorties LLM reelles (multi-paragraphes).

### Formule etendue

```
lambda_LD = alpha * NCD(A,B) + beta * (H(B)/H(A) - 1) + gamma * delta_topo + delta * ISC(B)
```

Ou ISC = Inter-Sentence Coherence (inversee : 1 - NCD_inter pour que coherent = positif).

## Decision

Cette analyse a mene directement a la decouverte du Fer a Cheval Topologique ([[20260209-09-horseshoe-theory]]).
Le ISC reste une piste pour v0.4.0 mais la priorite immediate est la Structural Quality (SQ).

**Actions** :
1. Implementer SQ(B) en remplacement de delta_topo ([[20260209-11-formula-v030-design]])
2. Implementer NCD damping ([[20260209-10-ncd-short-text-damping]])
3. Re-runner le grid search avec la formule v0.3.0
4. ISC comme 4eme metrique (v0.4.0, necessite textes multi-phrases)

## Liens

- [[20260209-04-grid-search-results]] (resultats pre-fix pour comparaison)
- [[20260209-03-formula-entropy-shift]] (la correction qui a permis gamma > 0)
- [[20260209-07-benchmark-live]] (benchmark live avec textes courts)
- [[20260209-09-horseshoe-theory]] (decouverte majeure qui decoule de cette analyse)
- [[20260209-10-ncd-short-text-damping]] (correction NCD)
- [[20260209-11-formula-v030-design]] (design v0.3.0)
