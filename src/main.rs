use clap::{Parser, Subcommand};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Parser)]
#[command(name = "greet")]
#[command(about = "A simple greeting CLI application")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Say hie to someone
    Hie {
        /// The name to greet
        name: String,

        /// Language to use for greeting
        #[arg(short, long, default_value = "en")]
        language: Language,

        /// Greet in uppercase (scream)
        #[arg(short, long)]
        scream: bool,
    },
    /// Say goodbye to someone
    Bye {
        /// The name to say goodbye to
        name: String,

        /// Language to use for goodbye
        #[arg(short, long, default_value = "en")]
        language: Language,

        /// Say goodbye in uppercase (scream)
        #[arg(short, long)]
        scream: bool,
    },
    /// List all available languages
    Languages,
}

#[derive(Clone, Copy, PartialEq, Eq, EnumIter)]
enum Language {
    En,
    Fr,
    Es,
    De,
    It,
}

impl std::str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "en" => Ok(Language::En),
            "fr" => Ok(Language::Fr),
            "es" => Ok(Language::Es),
            "de" => Ok(Language::De),
            "it" => Ok(Language::It),
            _ => Err(format!("Unknown language: {}", s)),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::En => write!(f, "en"),
            Language::Fr => write!(f, "fr"),
            Language::Es => write!(f, "es"),
            Language::De => write!(f, "de"),
            Language::It => write!(f, "it"),
        }
    }
}

fn generate_message(name: &str, language: Language, scream: bool, is_greeting: bool) -> String {
    let base_message = if is_greeting {
        match language {
            Language::En => "Hello",
            Language::Fr => "Bonjour",
            Language::Es => "Hola",
            Language::De => "Hallo",
            Language::It => "Ciao",
        }
    } else {
        match language {
            Language::En => "Goodbye",
            Language::Fr => "Au revoir",
            Language::Es => "Adiós",
            Language::De => "Auf Wiedersehen",
            Language::It => "Arrivederci",
        }
    };

    let mut message = format!("{} {}!", base_message, name);

    if scream {
        message = message.to_uppercase();
    }

    message
}

impl Language {
    fn get_name(&self) -> &'static str {
        match self {
            Language::En => "English",
            Language::Fr => "French",
            Language::Es => "Spanish",
            Language::De => "German",
            Language::It => "Italian",
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Hie { name, language, scream } => {
            let message = generate_message(name, *language, *scream, true);
            println!("{}", message);
        }
        Commands::Bye { name, language, scream } => {
            let message = generate_message(name, *language, *scream, false);
            println!("{}", message);
        }
        Commands::Languages => {
            println!("Available languages:");
            for language in Language::iter() {
                let is_default = matches!(language, Language::En);
                let name = language.get_name();
                if is_default {
                    println!("  {} - {} (default)", language, name);
                } else {
                    println!("  {} - {}", language, name);
                }
            }
        }
    }
}
