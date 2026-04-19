// Copyright (C) 2026 Jorge Andre Castro
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 or the License, or
// (at your option) any later version.
use std::env;
use std::fs;
use std::path::PathBuf;


/// Le script de compilation pour `rp2040-linker`.
/// 
/// Ce script automatise l'injection du fichier `memory.x` dans le processus 
/// de liaison (linkage) du projet utilisateur.
fn main() {
    // Récupère le dossier de sortie spécifique au build actuel (géré par Cargo)
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR non défini"));
    let memory_x_path = out.join("memory.x");
    // 1. Écrit le fichier memory.x contenu dans la crate vers le dossier de build.
    // Cela permet d'avoir une configuration mémoire "standard" sans effort.
    fs::write(&memory_x_path, include_bytes!("memory.x"))
        .expect("Impossible d'écrire le fichier memory.x");

    // 2. Ajoute le dossier 'out' au chemin de recherche du linker.
    // C'est ce qui permet au linker de trouver automatiquement notre memory.x.
    println!("cargo:rustc-link-search={}", out.display());

    // 3. Instruction de surveillance pour Cargo :
    // On ne recompile que si le script de build ou le fichier memory.x change.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");

    //4. Détection de conflit :
    // On n'affiche l'avertissement QUE si on est dans un projet utilisateur.
    //Si on compile la crate rp2040-linker elle-même, on ignore.
    let pkg_name = env::var("CARGO_PKG_NAME").unwrap_or_default();
    let project_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    
    if pkg_name != "rp2040-linker" && project_dir.join("memory.x").exists() {
        println!("cargo:warning=⚠️ Un fichier memory.x a été détecté à la racine de votre projet. Il pourrait entrer en conflit avec rp2040-linker.");
    }
}