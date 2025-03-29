use clap::{Parser, Subcommand};
use novascript::compiler::Compiler;
use novascript::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Crea un nuovo progetto NovaScript
    New {
        /// Nome del progetto
        name: String,
        
        /// Template da utilizzare
        #[arg(short, long, default_value = "basic")]
        template: String,
    },
    
    /// Esegue un programma NovaScript
    Run {
        /// Percorso del file da eseguire
        file: PathBuf,
        
        /// Argomenti da passare al programma
        #[arg(last = true)]
        args: Vec<String>,
    },
    
    /// Avvia un server di sviluppo con hot reloading
    Dev {
        /// Percorso del file entry point
        file: PathBuf,
        
        /// Porta su cui avviare il server
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
    },
    
    /// Compila un programma NovaScript in WebAssembly
    Build {
        /// Percorso del file da compilare
        file: PathBuf,
        
        /// Directory di output
        #[arg(short, long, default_value = "dist")]
        out_dir: String,
        
        /// Compila in modalità release
        #[arg(short, long)]
        release: bool,
    },
    
    /// Esegue i test
    Test {
        /// Pattern di test da eseguire
        #[arg(short, long)]
        pattern: Option<String>,
        
        /// Modalità watch
        #[arg(short, long)]
        watch: bool,
    },
    
    /// Formatta il codice NovaScript
    Fmt {
        /// File o directory da formattare
        #[arg(default_value = ".")]
        path: String,
        
        /// Controlla il formato senza modificare i file
        #[arg(short, long)]
        check: bool,
    },
    
    /// Controlla gli errori e lo stile del codice
    Lint {
        /// File o directory da controllare
        #[arg(default_value = ".")]
        path: String,
        
        /// Correggi automaticamente i problemi quando possibile
        #[arg(short, long)]
        fix: bool,
    },
    
    /// Controlla i tipi nel codice
    Check {
        /// File o directory da controllare
        #[arg(default_value = ".")]
        path: String,
    },
    
    /// Installa un pacchetto
    Install {
        /// Nome del pacchetto
        package: String,
        
        /// Installa come dipendenza di sviluppo
        #[arg(short, long)]
        dev: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::New { name, template } => {
            println!("Creazione nuovo progetto '{}' con template '{}'...", name, template);
            novascript::commands::new::execute(name, template)?;
        },
        
        Commands::Run { file, args } => {
            println!("Esecuzione '{}'...", file.display());
            novascript::commands::run::execute(file, args)?;
        },
        
        Commands::Dev { file, port } => {
            println!("Avvio server di sviluppo su porta {}...", port);
            novascript::commands::dev::execute(file, *port).await?;
        },
        
        Commands::Build { file, out_dir, release } => {
            println!("Compilazione '{}'...", file.display());
            let compiler = Compiler::new();
            compiler.compile(file, out_dir, *release)?;
        },
        
        Commands::Test { pattern, watch } => {
            println!("Esecuzione test{}{}...", 
                pattern.as_ref().map_or("".to_string(), |p| format!(" con pattern '{}'", p)), 
                if *watch { " in modalità watch" } else { "" });
            novascript::commands::test::execute(pattern.as_deref(), *watch)?;
        },
        
        Commands::Fmt { path, check } => {
            println!("Formattazione '{}'{}...", path, 
                if *check { " (solo controllo)" } else { "" });
            novascript::commands::fmt::execute(path, *check)?;
        },
        
        Commands::Lint { path, fix } => {
            println!("Linting '{}'{}...", path,
                if *fix { " con correzione automatica" } else { "" });
            novascript::commands::lint::execute(path, *fix)?;
        },
        
        Commands::Check { path } => {
            println!("Controllo tipi '{}'...", path);
            novascript::commands::check::execute(path)?;
        },
        
        Commands::Install { package, dev } => {
            println!("Installazione pacchetto '{}'{}...", package,
                if *dev { " come dipendenza di sviluppo" } else { "" });
            novascript::commands::install::execute(package, *dev)?;
        },
    }
    
    Ok(())
}