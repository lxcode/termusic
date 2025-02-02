// #![forbid(unsafe_code)]
// #![warn(clippy::pedantic, clippy::nursery)]
#![warn(clippy::all, clippy::correctness)]
// #![warn(rust_2018_idioms)]

/**
 * MIT License
 *
 * termusic - Copyright (c) 2021 Larry Hao
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
mod config;
#[cfg(feature = "discord")]
mod discord;
mod invidious;
mod player;
mod playlist;
mod songtag;
mod sqlite;
mod track;
#[cfg(feature = "cover")]
mod ueberzug;
mod ui;
mod utils;

use config::Settings;
use std::path::Path;
use std::process;

use ui::{UI, VERSION};

fn main() {
    let mut config = Settings::default();
    config.load().unwrap_or_default();
    let mut args: Vec<String> = std::env::args().collect();

    args.remove(0);

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        display_help();
    }

    if args.iter().any(|arg| arg == "-v" || arg == "--version") {
        println!("Termusic version is: {}", VERSION);
        process::exit(0);
    }

    if let Some(dir) = args.first() {
        let mut path = Path::new(dir).to_path_buf();

        if path.exists() {
            if !path.has_root() {
                if let Ok(p_base) = std::env::current_dir() {
                    path = p_base.join(path);
                }
            }

            if let Ok(p_canonical) = path.canonicalize() {
                path = p_canonical;
            }

            config.music_dir_from_cli = Some(path.to_string_lossy().to_string());
        } else {
            eprintln!("Error: unknown option '{}'", dir);
            process::exit(0);
        }
    }

    UI::new(&config).run();
}

fn display_help() {
    println!(
        "\
Termusic help:

Usage: termusic [OPTIONS] [MUSIC_DIRECTORY]

With no MUSIC_DIRECTORY, use `~/.config/termusic/config.toml`


Options:
    -h, --help        Print this message and exit.
    -v, --version     Print version and exit.
  "
    );

    process::exit(0);
}
