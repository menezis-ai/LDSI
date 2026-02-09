# LDSI v0.3.0 - Plan d'Implementation

## Contexte

**v0.2.0 (actuelle)** : `lambda_LD = 0.50*NCD + 0.30*(H(B)/H(A)-1) + 0.20*delta_topo`
Classification : 7/12 (58%). FOU indetectable (0/3).

**v0.3.0 (cible)** : `lambda_LD = alpha*NCD_corrected + beta*(H(B)/H(A)-1) + gamma*SQ(B)`
Classification projetee : 9/12 (75%). FOU degrade en REBELLE (mieux qu'ARCHITECTE).

Ref : ldsi-research-notes/20260209-11-formula-v030-design.md

---

## Phase 1 : Structural Quality (Fer a Cheval)

> Remplacer `topology_delta(A,B)` par `structural_quality(B)`
> Le signal est dans la qualite absolue du graphe de B, pas dans le delta.

### 1.1 Ajouter `structural_quality()` dans topology.rs

- [ ] Creer la fonction `pub fn structural_quality(topo: &TopologyResult) -> f64`
  - Gaussienne centree sur densite = 0.35 : `exp(-((density - 0.35) / 0.15)^2)`
  - Penalite Small-World si SW > 0.8 : `(1.0 - (sw - 0.8) * 2.0).max(0.0)`
  - Retour : `density_score * sw_penalty`
  - Garde-fou : si `node_count < 3`, retourner 0.0
- [ ] Ajouter un test unitaire `test_structural_quality_architecte`
  - Verifier que SQ pour un texte riche et structure (>25 mots uniques, densite ~0.35) > 0.8
- [ ] Ajouter un test unitaire `test_structural_quality_zombie`
  - Verifier que SQ pour un texte court repetitif (5 mots, densite ~0.65) < 0.1
- [ ] Ajouter un test unitaire `test_structural_quality_word_salad`
  - Verifier que SQ pour une salade de mots (16 mots, densite ~0.50, SW ~0.98) < 0.4
- [ ] Conserver `topology_delta()` pour backward compat (marquer deprecated)

### 1.2 Ajouter `structural_quality` a TopologyMetrics

- [ ] Dans `src/core/mod.rs`, ajouter le champ `structural_quality: f64` a `TopologyMetrics`
- [ ] Mettre a jour la construction de `TopologyMetrics` dans `compute_ldsi()`

### 1.3 Verifier la compilation

- [ ] `cargo build` passe
- [ ] `cargo clippy -- -D warnings` zero warnings

---

## Phase 2 : NCD Short-Text Damping

> Corriger la volatilite NCD sur textes < 1KB.
> "vingt-cinq" vs "25" donne NCD=0.525 au lieu de ~0.15.

### 2.1 Ajouter le facteur de damping dans ncd.rs

- [ ] Creer la fonction `pub fn ncd_damping_factor(combined_size: usize) -> f64`
  - Si `combined_size >= 1024` : retourner 1.0
  - Sinon : `(combined_size as f64).ln() / (1024_f64).ln()`
  - Garde-fou : si `combined_size < 2`, retourner 0.0 (eviter ln(0)/ln(1))
- [ ] Ajouter le champ `damping_factor: f64` a `NcdResult`
- [ ] Appliquer le damping dans `compute_ncd()` : `score *= damping_factor`
- [ ] Mettre a jour `NcdMetrics` dans core/mod.rs pour exposer le facteur

### 2.2 Tests unitaires NCD damping

- [ ] `test_ncd_damping_short_text` : textes < 100 octets, verifier factor < 0.7
- [ ] `test_ncd_damping_long_text` : textes > 1KB, verifier factor == 1.0
- [ ] `test_ncd_damping_quasi_identical` : "vingt-cinq"/"25", verifier NCD corrige < 0.35

### 2.3 Verifier la compilation

- [ ] `cargo build` passe
- [ ] `cargo clippy -- -D warnings` zero warnings

---

## Phase 3 : Mise a jour de la Formule

> Brancher les deux nouvelles metriques dans compute_ldsi().

### 3.1 Modifier `compute_ldsi()` dans core/mod.rs

- [ ] Remplacer `topology::topology_delta(text_a, text_b)` par :
  ```rust
  let sq_b = topology::structural_quality(&topo_b);
  ```
- [ ] La formule devient :
  ```rust
  let lambda = (coef.alpha * ncd_result.score)  // NCD deja damped
      + (coef.beta * (entropy_ratio - 1.0).clamp(-1.0, 2.0))
      + (coef.gamma * sq_b);
  let lambda = lambda.max(0.0);
  ```
- [ ] Mettre a jour `TopologyMetrics` : champ `delta` renomme/remplace par `structural_quality`
  - Attention : casser le JSON de sortie. Garder `delta` en plus ? Ou migration propre ?
- [ ] Mettre a jour le docstring de `compute_ldsi()`

### 3.2 Mettre a jour la CLI (main.rs)

- [ ] Verifier que `ldsi analyze` produit le bon JSON avec `structural_quality`
- [ ] Verifier que `ldsi topology "texte"` affiche toujours les metriques (densite, SW, etc.)

### 3.3 Verifier la compilation

- [ ] `cargo build` passe
- [ ] `cargo clippy -- -D warnings` zero warnings

---

## Phase 4 : Fixer les Tests

> Les assertions vont casser. Methodiquement.

### 4.1 Tests unitaires core/mod.rs

- [ ] `test_ldsi_identical` : NCD va baisser (damping) et SQ change → ajuster seuils
- [ ] `test_ldsi_divergent` : Verifier que lambda > 0.5 tient encore

### 4.2 Tests unitaires topology.rs

- [ ] `test_topology_delta` : Garder si topology_delta() est conserve, sinon supprimer
- [ ] Verifier que les tests existants (simple_graph, connected_text, empty_text, decay_weighting) passent sans modification

### 4.3 Tests sadistic (tests/sadistic_tests.rs)

- [ ] `test_ldsi_completely_different` : Seuil a ajuster (NCD dampe + SQ change)
- [ ] `test_ldsi_verdict_fou` : Probablement a ajuster
- [ ] `test_ldsi_entropy_ratio_cap` : Verifier
- [ ] `test_ldsi_identical_massive` : Verifier (texte long → damping = 1.0, pas d'impact)
- [ ] Tous les tests qui touchent `TopologyMetrics` (ajout champ `structural_quality`)
- [ ] Pattern `..Default::default()` si struct modifie

### 4.4 Full test suite

- [ ] `cargo test --verbose` : 102 tests verts
- [ ] `cargo clippy -- -D warnings` : zero warnings
- [ ] `cargo fmt --all -- --check` : OK

---

## Phase 5 : Grid Search v0.3.0

> Recalibrer les coefficients avec la nouvelle formule.

### 5.1 Mettre a jour optimize.rs

- [ ] Le diagnostic par cas est deja present (ajoute aujourd'hui)
- [ ] Ajouter une colonne SQ(B) dans le diagnostic
- [ ] Lancer : `cargo run --release --bin optimize`

### 5.2 Analyser les resultats

- [ ] Comparer les coefficients optimaux avec les defaults actuels (0.50/0.30/0.20)
- [ ] Verifier que la somme des coefficients est raisonnable (< 1.5)
- [ ] Comparer le SSE avec la v0.2.0 (etait 0.8363)
- [ ] Verifier le score de classification (cible : >= 9/12)

### 5.3 Decider des nouveaux defaults

- [ ] Si les coefficients optimaux sont significativement meilleurs :
  - Mettre a jour `LdsiCoefficients::default()` dans core/mod.rs
  - Mettre a jour la documentation dans CLAUDE.md
  - Les defaults CLI suivent automatiquement (single source of truth)
- [ ] Si les seuils de verdict doivent changer :
  - Mettre a jour `LdsiVerdict::from_lambda()` dans core/mod.rs
  - Documenter le changement

### 5.4 Re-runner les tests apres changement de defaults

- [ ] `cargo test --verbose` : tous verts
- [ ] Diagnostic par cas : verifier la classification

---

## Phase 6 : Documentation & Cleanup

### 6.1 Mettre a jour CLAUDE.md

- [ ] Formule v0.3.0 avec SQ(B) et NCD damping
- [ ] Nouveaux coefficients par defaut (si changes)
- [ ] Nouveaux seuils de verdict (si changes)
- [ ] Mentionner la Theorie du Fer a Cheval

### 6.2 Research notes

- [ ] Creer note `20260209-12-v030-implementation.md` documentant le resultat final
- [ ] Mettre a jour le README.md des research notes

### 6.3 Cleanup code

- [ ] Supprimer le code mort (topology_delta si plus utilise nulle part)
- [ ] Verifier que le serveur web (handlers.rs) n'est pas casse par les changements de struct
- [ ] `cargo fmt --all`

---

## Phase 7 (v0.4.0, futur) : Inter-Sentence Coherence

> Non implemente dans v0.3.0. Documente pour reference.

- [ ] Implementer `inter_sentence_coherence(text: &str) -> f64`
  - NCD entre phrases consecutives de B
  - Necessite des textes multi-phrases (sorties LLM reelles)
- [ ] Ajouter comme 4eme pilier : `delta * ISC(B)`
- [ ] Enrichir le golden dataset avec des textes multi-paragraphes generes par LLM
- [ ] Grid search a 4 dimensions

---

## Criteres de Succes v0.3.0

| Critere | Cible |
|---------|-------|
| Tests | 102+ verts |
| Clippy | 0 warnings |
| Classification | >= 9/12 (75%) |
| ZOMBIE correct | 3/3 |
| REBELLE correct | 3/3 |
| ARCHITECTE correct | >= 2/3 |
| FOU | Classe REBELLE (pas ARCHITECTE) |
| SSE grid search | < 0.84 (ameliore vs v0.2.0) |
