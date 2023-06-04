use lazy_static::lazy_static;
use serde::Serialize;

#[derive(Serialize, Copy, Clone, Debug)]
pub struct Repo {
    pub(crate) name: &'static str,
    pub(crate) about: &'static str,
    pub(crate) language: Language,
}

#[derive(Serialize, Copy, Clone, Debug, PartialEq)]
pub enum Language {
    RUST,
    JAVA,
    CSHARP,
    C,
    CPP,
    PYTHON,
    CSS,
    HTML,
    MARKDOWN,
    EV,
    NULL,
}

lazy_static! {
    #[derive(Serialize, Copy, Clone, Debug)]
    pub static ref REPOS: Vec<Repo> = vec![
        Repo {
            name: "Ev-Web-Server",
            about: "Web server i made in rust, its supposed to be homework but i figured it would fit well enough here",
            language: Language::RUST,
        },
        Repo {
            name: "Ev-Web-Server-Front-End",
            about: "Front end for the web server (https://github.com/Evryon75/Ev-Web-Server)",
            language: Language::CSS,
        },
        Repo {
            name: "Evryon75",
            about: "Personal Readme",
            language: Language::MARKDOWN,
        },
        Repo {
            name: "Pin-Bot-v2",
            about: "The improved pin manager bot",
            language: Language::RUST,
        },
        Repo {
            name: "T-item-Browser",
            about: "Overkill school project ?",
            language: Language::CSS,
        },
        Repo {
            name: "Clarissa",
            about: "Operating system made in Rust",
            language: Language::RUST,
        },
        Repo {
            name: "Collatz-conjecture",
            about: "I was bored and this seemed like an interesting thing to code up",
            language: Language::RUST,
        },
        Repo {
            name: "Ev-Interpreter",
            about: "An interpreter for the Ev programming language, my own language!",
            language: Language::RUST,
        },
        Repo {
            name: "Ev-Lang-Documentation",
            about: "Documentation for the Ev programming language",
            language: Language::HTML,
        },
        Repo {
            name: "FizzBuzz-in-C",
            about: "Segmentation fault",
            language: Language::C,
        },
        Repo {
            name: "Neural-Network",
            about: "A neural network made in pyton from scratch",
            language: Language::PYTHON,
        },
        Repo {
            name: "FizzBuzz",
            about: "This is my approach with fizzbuzz",
            language: Language::RUST,
        },
        Repo {
            name: "EvCalc",
            about: "A calculator in the Ev programming language",
            language: Language::EV,
        },
        Repo {
            name: "Sussy-Squares",
            about: "Overkill school project. Controls: W A S D + SPACE vs ARROW KEYS + ENTER. Keyboards without N Key Rollover that dont scan keys indipendently from one another might struggle.",
            language: Language::JAVA,
        },
        Repo {
            name: "Lorenz-System-in-Unity",
            about: "I made the Lorenz system of equations in Unity",
            language: Language::CSHARP,
        },
        Repo {
            name: "Conway-s-Game-of-Life",
            about: "Although the grid is limited, it is large enough for a glider gun.",
            language: Language::JAVA,
        },
        Repo {
            name: "Red-Square-Jumping",
            about: "First real Unity project",
            language: Language::CSHARP,
        },
        Repo {
            name: "Pin-Manager_Discord-Bot",
            about: "This bot checks the number of pinned messages in a channel, and if they have reached the limit the channel is locked and archived. A new channel will be created to replace the old one.",
            language: Language::JAVA,
        },
        Repo {
            name: "Homing-Object",
            about: "A dot that follows another user-controlled dot (arrow keys) on a 2D plane at a consistent speed, in every orientation.",
            language: Language::JAVA,
        },
        Repo {
            name: "FxTris",
            about: "Modern tetris in JavaFX",
            language: Language::JAVA,
        },
        Repo {
            name: "3D-Engine",
            about: "I followed a tutorial for this after learning linear algebra.",
            language: Language::CPP,
        },
    ];
}
