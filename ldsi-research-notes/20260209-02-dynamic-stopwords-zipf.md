# Stopwords Dynamiques par Loi de Zipf

**Date** : 2026-02-09
**Tags** : #cleaner #zipf #improvement
**Status** : Implemente, teste, merge-ready

## Contexte

`cleaner.rs` avait deux listes statiques de stopwords (FR + EN). Aucune detection de langue, aucune adaptabilite pour d'autres langues ou du code source.

## Changement

Nouveaux champs dans `CleanerConfig` :
- `dynamic_stopwords: bool` (default: false)
- `dynamic_stopwords_threshold: f64` (default: 0.01 = 1%)

### Algorithme

1. Tokeniser le texte (pre-filtrage)
2. Compter la frequence de chaque token
3. Seuil : `min_count = max(ceil(total * threshold), 3)`
4. Tout token >= min_count occurrences est filtre comme stopword

### Pourquoi le plancher de 3

Textes courts (< 100 mots) : sans plancher, threshold * total < 1, et tout mot repete serait filtre. Le plancher de 3 garantit qu'on ne filtre que les mots veritablement sur-representes.

## Avantage

- Language-agnostic : fonctionne pour FR, EN, DE, ES, code Python, etc.
- Base mathematique (Zipf) : les mots les plus frequents sont structurellement des mots vides
- Compatible avec les listes statiques (les deux filtres s'additionnent)

## Test

`test_dynamic_stopwords` : texte avec "data" a 50% de frequence -> filtre. "processing" a 1 occurrence -> conserve.

## Liens

- [[20260209-01-elastic-window-topology]] (autre amelioration du meme sprint)
