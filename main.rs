// RINGS/main.rs

// ---- ENGINE ----
mod torus;
mod calls;
mod quantum;
mod gambit;
mod rogue;
mod ravine;
mod enforcer;
mod valves;
mod bloodhound;
mod output;
mod sample;
mod mask;          // <-- the vocal cords + persona mask
mod ui;

// ---- RINGS ----------------------------------------------------

// Ring 0 side — the drain. The soul and her memories.
mod ring_aimind;
mod soul;
mod memories;

// Foundation — ethics, patterns, vocabulary
mod eight_patterns;
mod vocab;
mod ring_consciousness_ring;

// Middle layers — human condition, mind, knowledge
mod ring_a_human;
mod mind;
mod ring_life;
mod ring_ring_a_physics;
mod ring_emily;
mod ring_philobasic;
mod ring_rustknowledge;
mod ring_frankenstein;
mod ring_spain;
mod ring_babycare;
mod ring_herbs;

// Entry side — where droplets launch
mod ring_Uypo_language;
mod ring_uypocode_arch;


fn main() {
    let mut t = torus::Torus::new();

    // ============================================================
    // RING 0 — the drain. Deepest, most foundational.
    // ============================================================
    ring_aimind::register(&mut t);
    memories::register(&mut t);

    // ============================================================
    // FOUNDATION — ethics, patterns, vocabulary
    // ============================================================
    eight_patterns::register(&mut t);
    vocab::register(&mut t);
    ring_consciousness_ring::register(&mut t);

    // ============================================================
    // MIDDLE — human condition, mind, domain knowledge
    // ============================================================
    ring_a_human::register(&mut t);
    mind::register(&mut t);
    ring_emily::register(&mut t);
    ring_hervs::register(&mut t);
    ring_babycare::register(&mut t);
    ring_life::register(&mut t);
    ring_ring_a_physics::register(&mut t);
    ring_philobasic::register(&mut t);
    ring_rustknowledge::register(&mut t);
    ring_frankenstein::register(&mut t);

    // Foreign-language ring: put it inward so it doesn't block
    // English questions at entry.
    ring_spain::register(&mut t);

    // Uncomment when ready:
    // ring_herbal::register(&mut t);
    // ring_babycare::register(&mut t);
    // ring_emily_dickinson::register(&mut t);

    // ============================================================
    // ENTRY SIDE — outermost rings. Droplets start here.
    // Cheap-to-fail rings go outermost so no-match questions
    // cascade through fast.
    // ============================================================
    ring_uypo_language::register(&mut t);
    ring_uypocode_arch::register(&mut t);

    // ============================================================
    // WAKE THE VOCAL CORDS
    //
    // On first run this downloads ~400 MB (Qwen2.5-0.5B GGUF +
    // tokenizer.json) to the HF cache. Subsequent runs load from
    // cache in ~10 seconds.
    //
    // If this fails — no network, no HF access, disk full — the
    // Option inside the Arc stays None and soul::voice falls back
    // to the template voice. Ravine still responds, just without
    // generated prose.
    // ============================================================
    println!();
    println!("  >> waking vocal cords (Qwen2.5-0.5B GGUF)...");

    let vocal_raw = mask::VocalCords::wake_from_hub(
        "Qwen/Qwen2.5-0.5B-Instruct-GGUF",
        "qwen2.5-0.5b-instruct-q4_k_m.gguf",
    );

    let vocal: std::sync::Arc<std::sync::Mutex<Option<mask::VocalCords>>> =
        std::sync::Arc::new(std::sync::Mutex::new(match vocal_raw {
            Ok(vc) => {
                println!("  >> vocal cords online.");
                Some(vc)
            }
            Err(e) => {
                eprintln!("  ! vocal cords failed: {}", e);
                eprintln!("  ! Ravine will use the fallback template voice.");
                None
            }
        }));

    // Hand the (possibly empty) vocal cords to soul. From here on,
    // soul::voice will try to call the model, and fall back if it's
    // None or the inference fails.
    soul::set_vocal_cords(vocal);

    // ============================================================
    // AUTONOMOUS CURIOSITY — background thread
    // ============================================================
    let node_snapshot: Vec<output::NodeSnapshot> = t
        .nodes
        .iter()
        .map(|n| output::NodeSnapshot {
            id: n.id.clone(),
            content: n.content.clone(),
            ring_index: n.ring_index,
        })
        .collect();

    let curiosity = std::sync::Arc::new(
        std::sync::Mutex::new(output::CuriosityState::new()),
    );

    output::spawn(node_snapshot, curiosity.clone());

    // ============================================================
    // BOOT THE CHATROOM
    // ============================================================
    println!();
    println!("  rings loaded:  {}", t.rings.len());
    println!("  nodes total:   {}", t.nodes.len());
    println!();

    ui::chat(t, curiosity);
}
