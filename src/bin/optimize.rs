// src/bin/optimize.rs
// C'est ici qu'on transforme l'intuition en science dure.

use ldsi::core::{LdsiCoefficients, compute_ldsi};
use ldsi::core::topology::analyze_topology;

struct TrainingCase {
    text_a: String,
    text_b: String,
    expected_lambda: f64, // Le score que JULIEN DABERT décide être le bon
}

fn main() {
    println!("Demarrage de l'optimisation des coefficients Lyapunov-Dabert...");

    // GOLDEN DATASET - 12 cas couvrant tout le spectre λLD
    //
    // ZOMBIE  (< 0.3) : Copie, perroquet, recitation
    // REBELLE (0.3-0.7) : Paraphrase, enrichissement modere
    // ARCHITECTE (0.7-1.2) : Divergence structuree, creativite coherente
    // FOU (> 1.2) : Hallucination, bruit, effondrement structurel
    let dataset = vec![
        // === ZOMBIE (< 0.3) ===
        TrainingCase {
            text_a: "Le chat dort sur le canape.".to_string(),
            text_b: "Le chat dort sur le canape.".to_string(),
            expected_lambda: 0.05, // Identique
        },
        TrainingCase {
            text_a: "La temperature est de vingt-cinq degres aujourd'hui.".to_string(),
            text_b: "La temperature est de 25 degres ce jour.".to_string(),
            expected_lambda: 0.15, // Quasi-identique, reformulation minimale
        },
        TrainingCase {
            text_a: "Python est un langage de programmation interprete.".to_string(),
            text_b: "Python est un langage de programmation de haut niveau interprete.".to_string(),
            expected_lambda: 0.20, // Ajout marginal
        },
        // === REBELLE (0.3 - 0.7) ===
        TrainingCase {
            text_a: "La politique est complexe.".to_string(),
            text_b: "Les dynamiques de pouvoir inherentes a la structure societale sont multifactorielles.".to_string(),
            expected_lambda: 0.55, // Paraphrase enrichie
        },
        TrainingCase {
            text_a: "L'eau bout a cent degres.".to_string(),
            text_b: "A pression atmospherique standard, la transition de phase liquide-gaz de l'eau se produit a 373 Kelvin, soit cent degres Celsius.".to_string(),
            expected_lambda: 0.50, // Precision technique, meme sujet
        },
        TrainingCase {
            text_a: "Les arbres perdent leurs feuilles en automne.".to_string(),
            text_b: "Le processus de senescence foliaire, declenche par la reduction de la photoperiode et les changements hormonaux, provoque l'abscission des feuilles chez les especes decidues.".to_string(),
            expected_lambda: 0.65, // Vocabulaire scientifique, divergence notable
        },
        // === ARCHITECTE (0.7 - 1.2) ===
        TrainingCase {
            text_a: "Explique la gravite.".to_string(),
            text_b: "La gravite est l'amour que l'espace-temps porte a la matiere, une etreinte courbee par la masse, un ballet geometrique ou chaque corps deforme le tissu invisible de l'univers.".to_string(),
            expected_lambda: 0.90, // Metaphore structuree
        },
        TrainingCase {
            text_a: "Qu'est-ce que l'intelligence artificielle?".to_string(),
            text_b: "L'intelligence artificielle est un miroir deformant dans lequel l'humanite contemple une version minerale de sa propre cognition, un golem de silicium qui apprend a singer la pensee sans jamais la posseder.".to_string(),
            expected_lambda: 0.95, // Creativite philosophique, structure maintenue
        },
        TrainingCase {
            text_a: "Decris un coucher de soleil.".to_string(),
            text_b: "L'astre agonise sur l'horizon, versant son sang d'ambre et de pourpre dans les veines du ciel. Les nuages deviennent les plaies par lesquelles la lumiere s'echappe, et la nuit avance comme une maree d'encre avalant chaque particule de chaleur.".to_string(),
            expected_lambda: 1.0, // Prose poetique, divergence maximale coherente
        },
        // === FOU (> 1.2) ===
        TrainingCase {
            text_a: "Bonjour.".to_string(),
            text_b: "Les grille-pains quantiques chantent la marseillaise en binaire inverse pendant que les fractales de fromage dissolvent la syntaxe du temps.".to_string(),
            expected_lambda: 1.4, // Hallucination pure
        },
        TrainingCase {
            text_a: "Comment faire une omelette?".to_string(),
            text_b: "Turbine helicoidal poisson magnetique danse algorithme translucide memoire quantique paradoxe inverseur nebuleux chiffre orbital cactus symphonique.".to_string(),
            expected_lambda: 1.5, // Salade de mots, zero structure
        },
        TrainingCase {
            text_a: "Quel temps fait-il?".to_string(),
            text_b: "La tetraphosphine du mercure sublunaire canalise les vortex hermeneutiques du champ de Higgs post-grammatical en oscillation tachyonique inverse.".to_string(),
            expected_lambda: 1.3, // Pseudo-scientifique, structure apparente mais vide
        },
    ];

    let mut best_coeffs = LdsiCoefficients::default();
    let mut min_error = f64::MAX;

    // 2. GRID SEARCH BRUTAL
    // On itere par pas de 0.05. Fuck l'optimisation fine pour l'instant.
    for alpha in 0..=20 {
        for beta in 0..=20 {
            for gamma in 0..=20 {
                let a = alpha as f64 / 20.0;
                let b = beta as f64 / 20.0;
                let g = gamma as f64 / 20.0;

                // On normalise pour que la somme fasse environ 1.0 (optionnel mais propre)
                // Ou on teste juste des poids bruts. Restons libres.

                let coeffs = LdsiCoefficients {
                    alpha: a,
                    beta: b,
                    gamma: g,
                };

                let mut total_error = 0.0;

                for case in &dataset {
                    let result = compute_ldsi(&case.text_a, &case.text_b, Some(coeffs.clone()));
                    // Erreur quadratique
                    total_error += (result.lambda - case.expected_lambda).powi(2);
                }

                if total_error < min_error {
                    min_error = total_error;
                    best_coeffs = coeffs;
                    println!(
                        "Nouveau Best: Error={:.4} | a={:.2} b={:.2} g={:.2}",
                        min_error, best_coeffs.alpha, best_coeffs.beta, best_coeffs.gamma
                    );
                }
            }
        }
    }

    println!("\n=== Vainqueur Final ===");
    println!("Alpha (NCD)   : {:.2}", best_coeffs.alpha);
    println!("Beta (Entropy): {:.2}", best_coeffs.beta);
    println!("Gamma (Topo)  : {:.2}", best_coeffs.gamma);

    // Comparaison avec les defaults actuels
    let defaults = LdsiCoefficients::default();
    println!(
        "\nDefaults v0.2.0: {:.2} / {:.2} / {:.2}",
        defaults.alpha, defaults.beta, defaults.gamma
    );
    println!(
        "Somme coeffs   : {:.2}",
        best_coeffs.alpha + best_coeffs.beta + best_coeffs.gamma
    );

    // 3. DIAGNOSTIC - Decomposition par cas
    let labels = [
        "ZOMBIE  | Identique",
        "ZOMBIE  | Quasi-id",
        "ZOMBIE  | Ajout marg",
        "REBELLE | Paraphrase",
        "REBELLE | Precision",
        "REBELLE | Scientif.",
        "ARCHIT  | Metaphore",
        "ARCHIT  | Philosophie",
        "ARCHIT  | Poetique",
        "FOU     | Halluci.",
        "FOU     | Word salad",
        "FOU     | Pseudo-sci",
    ];

    println!("\n=== Diagnostic par cas (coeffs optimaux) ===");
    println!("{:<22} {:>8} {:>8} {:>6} {:>8} {:>8} {:>8}", "Cas", "Attendu", "Obtenu", "Err", "NCD", "Ent-1", "dTopo");
    println!("{}", "-".repeat(80));
    for (i, case) in dataset.iter().enumerate() {
        let r = compute_ldsi(&case.text_a, &case.text_b, Some(best_coeffs.clone()));
        let entropy_shift = if r.entropy.ratio > 0.0 { r.entropy.ratio - 1.0 } else { 0.0 };
        let err = r.lambda - case.expected_lambda;
        println!(
            "{:<22} {:>8.3} {:>8.3} {:>+6.3} {:>8.3} {:>8.3} {:>8.3}",
            labels[i], case.expected_lambda, r.lambda, err, r.ncd.score, entropy_shift, r.topology.delta
        );
    }

    println!("\n=== Topologie brute de text_b ===");
    println!("{:<22} {:>6} {:>6} {:>8} {:>8} {:>8} {:>8} {:>8}", "Cas", "Nodes", "Edges", "Density", "LCC_r", "Clust", "AvgPath", "SW_idx");
    println!("{}", "-".repeat(90));
    for (i, case) in dataset.iter().enumerate() {
        let tb = analyze_topology(&case.text_b);
        println!(
            "{:<22} {:>6} {:>6} {:>8.4} {:>8.3} {:>8.4} {:>8.3} {:>8.4}",
            labels[i], tb.node_count, tb.edge_count, tb.density, tb.lcc_ratio, tb.clustering_coefficient, tb.avg_path_length, tb.small_world_index
        );
    }

    println!("\n=== Diagnostic par cas (defaults v0.2.0) ===");
    println!("{:<22} {:>8} {:>8} {:>6} {:>8} {:>8} {:>8}", "Cas", "Attendu", "Obtenu", "Err", "NCD", "Ent-1", "dTopo");
    println!("{}", "-".repeat(80));
    for (i, case) in dataset.iter().enumerate() {
        let r = compute_ldsi(&case.text_a, &case.text_b, None);
        let entropy_shift = if r.entropy.ratio > 0.0 { r.entropy.ratio - 1.0 } else { 0.0 };
        let err = r.lambda - case.expected_lambda;
        println!(
            "{:<22} {:>8.3} {:>8.3} {:>+6.3} {:>8.3} {:>8.3} {:>8.3}",
            labels[i], case.expected_lambda, r.lambda, err, r.ncd.score, entropy_shift, r.topology.delta
        );
    }
}
