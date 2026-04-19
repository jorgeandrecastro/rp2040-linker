[![Crates.io](https://img.shields.io/crates/v/rp2040-linker.svg)](https://crates.io/crates/rp2040-linker)
[![License: GPL-2.0-or-later](https://img.shields.io/badge/License-GPL_2.0_or_later-blue.svg)](https://opensource.org/licenses/GPL-2.0)
# rp2040-linker

**Configuration automatique du linker et du boot pour le RP2040 (Raspberry Pi Pico) en Rust.**

Cette crate permet de gérer sans effort la mise en page de la mémoire et le second étage du bootloader (BOOT2) pour les applications Rust no_std. Plus besoin de configurer manuellement votre fichier **memory.x**.

✅ Testé avec succès sur Raspberry Pi Pico et RP2040 Zero.

----

## 🚀 Points Forts

* **Zéro Configuration :** Injecte automatiquement un `memory.x` optimisé pour le RP2040 (2 Mo Flash, 264 Ko RAM).
* **Prêt pour le Boot :** Gère la section `.boot2` à l'adresse correcte (`0x10000000`).
* **Placement Garanti :** Utilise `INSERT BEFORE .text` pour s'assurer que le bootloader est placé exactement là où le BootROM l'attend.
* **Compatibilité Totale :** Fonctionne avec `cortex-m-rt`, `embassy-rp`, et `rp2040-hal`.

----

## 📦 Installation

**Ajoutez ceci à votre fichier `Cargo.toml`:** 

````toml
[dependencies]
rp2040-linker = "0.1.1"
🛠 Utilisation
Dans votre fichier main.rs, importez simplement la crate pour activer l'automatisation :
````

````rust
#![no_std]
#![no_main]

// La ligne magique indispensable : elle gère la mémoire et les blocs de boot
use rp2040_linker as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Votre code pour RP2040 ici...
    loop {}
}

````
**Configuration requise dans .cargo/config.toml**
Assurez-vous que votre projet utilise les arguments du linker de cortex-m-rt :

````
Ini, TOML
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
rustflags = [
  "-C", "link-arg=-Tlink.x",
  "-C", "link-arg=--nmagic",
]

[build]
target = "thumbv6m-none-eabi"
````

----

# 🏗 Cartographie Mémoire
La crate définit la structure suivante :
````
Section   Origine      Taille         Description

BOOT2  0x10000000   256 octets    Second étage du bootloader
FLASH  0x10000100   2047 Ko       Code application et données
RAM    0x20000000   264 Ko        Mémoire SRAM système
````
----

# ⚡Workflow : Flash Rapide (flash.sh)
Pour ceux qui utilisent picotool directement, voici un script d'automatisation pour compiler, convertir et flasher votre RP2040 en une seule commande.

**📄 flash.sh**
Créez ce fichier à la racine de votre projet : 

````
#!/bin/bash
# On force le chemin vers Cargo pour les environnements sudo
export PATH="$HOME/.cargo/bin:$PATH"

echo "🦀 Étape 1 : Compilation en mode release..."
cargo build --release || { echo "❌ Échec de la compilation"; exit 1; }

# NOTE : Remplacez 'votre_projet' par le nom de votre binaire dans Cargo.toml
BINARY_NAME="votre_projet"
TARGET_PATH="target/thumbv6m-none-eabi/release/$BINARY_NAME"

echo "📦 Étape 2 : Conversion ELF vers UF2 pour RP2040..."
picotool uf2 convert -t elf "$TARGET_PATH" firmware.uf2 --family rp2040

echo "⚡ Étape 3 : Flashage du périphérique (nécessite sudo)..."
sudo picotool load firmware.uf2 -x

echo "✅ Terminé ! Votre Pico redémarre... "

````
**Comment l'utiliser :**

1. Rendez le script exécutable : chmod +x flash.sh

2. Lancez le flash : ./flash.sh

----

# 🏗️ Pourquoi utiliser cette crate ?

Le démarrage d'un RP2040 est un processus rigoureux. Le processeur cherche un bloc de démarrage spécifique appelé BOOT2 (Second Stage Bootloader) de 256 octets exactement à l'adresse 0x10000000. Sans une organisation mémoire parfaite, votre programme Rust ne pourra jamais s'exécuter.

rp2040-linker simplifie radicalement ce processus :

Infrastructure Invisible : Elle génère et injecte dynamiquement le script de liaison (memory.x) nécessaire lors de la compilation.

Placement BOOT2 Garanti : Grâce à l'instruction INSERT BEFORE .text, elle force le placement du bloc de boot avant votre code applicatif, assurant la compatibilité avec le BootROM.

Vérification de Cible : Elle empêche les erreurs de compilation accidentelles si vous ne ciblez pas l'architecture ARM (thumbv6m-none-eabi).

----

# 🔄 Gestion des conflits (Priorité & Shadowing)

**Que se passe-t-il si j'ai déjà un fichier memory.x à la racine de mon projet ?**

Le linker de Rust (rust-lld) utilise le premier script de mémoire qu'il trouve. Si un fichier local est présent, il risque de "masquer" (shadowing) celui de la crate, ce qui peut empêcher le démarrage du RP2040.

**🛡️ Détection et Isolation**
Détection automatique : La crate vérifie la présence d'un fichier memory.x local dans votre projet.

Isolation intelligente : Le mécanisme ignore la crate rp2040-linker elle-même lors de sa propre compilation (via CARGO_PKG_NAME). Cela évite les faux positifs pour les contributeurs de la bibliothèque.

Alerte ciblée : L'avertissement ne se déclenche que lorsqu'un projet tiers (votre firmware) possède son propre fichier.

⚠️ Avertissement (Warning)
Si un conflit est détecté, un message s'affiche durant la compilation :

⚠️ Un fichier memory.x a été détecté à la racine de votre projet. Il pourrait entrer en conflit avec rp2040-linker.

**✅ Résolution**
Pour laisser la crate gérer entièrement votre mémoire (mode "Zéro-Config"), il est fortement recommandé de supprimer le fichier memory.x local. Cela garantit que la configuration optimisée de la crate passe en priorité absolue.

Pourquoi est-ce important ?
Cela permet de maintenir un flux de compilation (pipeline CI/CD) propre tout en protégeant les développeurs contre les comportements imprévisibles du linker.

----

# 🛡️ Sécurité & Fiabilité

En centralisant la gestion de la mémoire dans cette crate, vous évitez les erreurs humaines de copier-coller entre vos différents projets. Vous bénéficiez d'une base solide, testée et conforme aux spécifications techniques de Raspberry Pi.

----

# 🔄 Gestion des conflits (Note technique)
Note : La détection automatique du fichier memory.x local fonctionne principalement lors du développement de la crate ou lors d'une utilisation via un chemin local (path = "...").

En raison de l'isolation de sécurité de Cargo, lorsqu'elle est installée via crates.io, la crate peut ne pas "voir" les fichiers de votre projet parent. Par mesure de sécurité, assurez-vous toujours de supprimer tout fichier memory.x manuel à la racine de votre projet pour garantir que celui de la crate soit utilisé et ne pas avoir des errurs liées innatendues due à un muavais memory.x qui traine.

----

# 🛡 Licence
Ce projet est sous licence GPL-2.0-or-later.

----

# 🦅 À propos

Développé par Jorge Andre Castro

