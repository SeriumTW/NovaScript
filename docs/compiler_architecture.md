# Architettura del Compilatore NovaScript

## Panoramica

Il compilatore NovaScript è una pipeline multi-stadio che trasforma il codice sorgente NovaScript in WebAssembly. Il compilatore è costruito con Rust per prestazioni, sicurezza della memoria ed eccellente supporto WebAssembly.

## Stadi del Compilatore

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│             │    │             │    │             │    │             │    │             │
│  Codice     │    │  Token      │    │    AST      │    │    IR       │    │   Modulo    │
│  Sorgente   │──▶│  (Lexer)    │──▶│  (Parser)    │──▶│ (Optimizer) │──▶│   WASM      │
│             │    │             │    │             │    │             │    │             │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
```

### 1. Analisi Lessicale (Lexer)

Il lexer trasforma il testo del codice sorgente in una sequenza di token, che sono le unità più piccole di significato nel linguaggio.

#### Dettagli Implementativi:

- Utilizza uno scanner scritto a mano con approccio a macchina a stati
- Tiene traccia delle posizioni nel sorgente per la segnalazione degli errori
- Gestisce i token di indentazione (INDENT, DEDENT) per la significatività degli spazi bianchi
- Riconosce parole chiave, identificatori, letterali e operatori

#### Tipi di Token:

```rust
enum TipoToken {
    // Parole chiave
    Let, Const, Fn, If, Else, For, While, Return, // ...

    // Letterali
    Identificatore, LiteraleNumerico, LiteraleStringa, LiteraleBooleano, LiteraleNull,

    // Operatori
    Plus, Minus, Star, Slash, Equals, EqualsEquals, // ...

    // Punteggiatura
    Comma, Dot, Colon, Arrow, LeftParen, RightParen, // ...

    // Indentazione
    Indent, Dedent, Newline,

    // Speciali
    Eof, Error,
}

struct Token {
    tipo: TipoToken,
    lessema: String,
    linea: usize,
    colonna: usize,
}
```

### 2. Analisi Sintattica (Parser)

Il parser trasforma il flusso di token in un Abstract Syntax Tree (AST) che rappresenta la struttura gerarchica del programma.

#### Dettagli Implementativi:

- Implementa un parser a discesa ricorsiva
- Utilizza il generatore di parser LALRPOP per la definizione della grammatica
- Tiene traccia dei livelli di indentazione per le strutture a blocchi
- Esegue un recupero di base degli errori sintattici

#### Tipi di Nodi AST:

```rust
enum Expr {
    Binary { left: Box<Expr>, operatore: Token, right: Box<Expr> },
    Unary { operatore: Token, right: Box<Expr> },
    Literal { valore: Valore },
    Variable { nome: Token },
    Call { callee: Box<Expr>, argomenti: Vec<Expr> },
    // ...
}

enum Stmt {
    Expression { expr: Expr },
    Declaration { nome: Token, annotazione_tipo: Option<TipoExpr>, inizializzatore: Option<Expr> },
    Function { nome: Token, params: Vec<Parametro>, tipo_ritorno: Option<TipoExpr>, corpo: Vec<Stmt> },
    If { condizione: Expr, ramo_then: Vec<Stmt>, ramo_else: Option<Vec<Stmt>> },
    // ...
}
```

### 3. Analisi Semantica

L'analizzatore semantico valida l'AST ed esegue il controllo dei tipi e l'inferenza.

#### Dettagli Implementativi:

- Costruisce tabelle dei simboli per la gestione dello scope
- Esegue la risoluzione dei nomi
- Conduce il controllo dei tipi con tipizzazione graduale
- Implementa l'inferenza di tipo usando un algoritmo basato su vincoli
- Rileva errori semantici (es., variabili non definite, mismatch di tipo)

#### Controllo dei Tipi:

```rust
struct TypeChecker {
    ambiente: Ambiente,
    funzione_corrente: Option<TipoFunzione>,
}

impl TypeChecker {
    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), TipoErrore> {
        // ...
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<Tipo, TipoErrore> {
        // ...
    }

    fn infer_type(&mut self, expr: &Expr) -> Result<Tipo, TipoErrore> {
        // ...
    }
}
```

### 4. Rappresentazione Intermedia (IR)

Il compilatore trasforma l'AST in una Rappresentazione Intermedia (IR) di livello inferiore che è più facile da ottimizzare e tradurre nel codice target.

#### Dettagli Implementativi:

- Utilizza un IR personalizzato progettato per NovaScript (NovaIR)
- Appiattisce espressioni complesse in operazioni più semplici
- Rappresenta il flusso di controllo con blocchi base e salti
- Adatto per passaggi di ottimizzazione

#### Struttura IR:

```rust
struct Function {
    nome: String,
    params: Vec<Parametro>,
    blocchi: Vec<BloccoBase>,
    tipo_ritorno: Tipo,
}

struct BloccoBase {
    etichetta: String,
    istruzioni: Vec<Istruzione>,
    terminatore: Terminatore,
}

enum Istruzione {
    BinaryOp { risultato: String, op: OperatoreBinario, left: Operando, right: Operando },
    UnaryOp { risultato: String, op: OperatoreUnario, operando: Operando },
    Call { risultato: Option<String>, funzione: String, argomenti: Vec<Operando> },
    Load { risultato: String, indirizzo: Operando },
    Store { indirizzo: Operando, valore: Operando },
    // ...
}

enum Terminatore {
    Jump { target: String },
    Branch { condizione: Operando, target_then: String, target_else: String },
    Return { valore: Option<Operando> },
    // ...
}
```

### 5. Ottimizzazione

Il compilatore esegue varie ottimizzazioni sull'IR per migliorare le prestazioni e ridurre la dimensione del codice.

#### Passaggi di Ottimizzazione:

- **Constant Folding**: Valuta espressioni costanti a tempo di compilazione
- **Dead Code Elimination**: Rimuove codice irraggiungibile
- **Common Subexpression Elimination**: Evita calcoli ridondanti
- **Function Inlining**: Sostituisce le chiamate di funzione con i loro corpi
- **Loop Optimizations**: Unrolling, hoisting e fusion
- **Type Specialization**: Genera codice specializzato per tipi noti

### 6. Generazione di Codice (WebAssembly)

Lo stadio finale traduce l'IR ottimizzato in WebAssembly.

#### Dettagli Implementativi:

- Utilizza `walrus` o librerie Rust simili per la generazione WebAssembly
- Traduce le istruzioni NovaIR in istruzioni WebAssembly
- Gestisce la gestione della memoria e le chiamate di funzione runtime
- Emette il modulo binario WASM

#### Generazione WASM:

```rust
struct WasmGenerator {
    modulo: Modulo,
    funzioni: HashMap<String, FunctionId>,
    globali: HashMap<String, GlobalId>,
    // ...
}

impl WasmGenerator {
    fn generate_function(&mut self, funzione: &Function) -> FunctionId {
        // ...
    }

    fn translate_instruction(&mut self, istr: &Istruzione, func_builder: &mut FunctionBuilder) {
        // ...
    }
}
```

## Supporto Runtime

Il compilatore include librerie runtime che forniscono:

1. **Gestione della Memoria**: Garbage collection per la gestione sicura della memoria
2. **Event Loop**: Gestione delle operazioni asincrone
3. **Libreria Standard**: Funzionalità core e utilità
4. **JS Interop**: Bridge tra NovaScript e JavaScript

## Supporto al Debugging

Il compilatore genera source map che mappano il codice WebAssembly al sorgente NovaScript, abilitando il debugging a livello di sorgente.

## Sistema di Build

Il compilatore NovaScript è integrato con un sistema di build che:

1. Gestisce le dipendenze
2. Gestisce la compilazione incrementale
3. Fornisce un server di sviluppo
4. Supporta l'hot module replacement

## Implementazione del Compilatore

### Struttura del Progetto del Compilatore

```
novascript-compiler/
├── src/
│   ├── main.rs                  // Entry point
│   ├── lexer/
│   │   ├── mod.rs
│   │   ├── token.rs             // Definizioni dei token
│   │   └── scanner.rs           // Scanner principale
│   ├── parser/
│   │   ├── mod.rs 
│   │   ├── ast.rs               // Strutture AST
│   │   ├── grammar.lalrpop      // Definizione grammatica
│   │   └── parser.rs            // Implementazione parser
│   ├── semantics/
│   │   ├── mod.rs
│   │   ├── symbol_table.rs      // Tabelle dei simboli
│   │   ├── type_checker.rs      // Verificatore di tipo
│   │   └── type_inferrer.rs     // Inferenza di tipo
│   ├── ir/
│   │   ├── mod.rs
│   │   ├── ir.rs                // Definizioni IR
│   │   └── lowering.rs          // Abbassamento da AST a IR
│   ├── optimizer/
│   │   ├── mod.rs
│   │   ├── constant_folder.rs   // Ottimizzatore constant folding
│   │   ├── dce.rs               // Dead code elimination
│   │   └── inliner.rs           // Function inlining
│   ├── codegen/
│   │   ├── mod.rs
│   │   ├── wasm_gen.rs          // Generazione WebAssembly
│   │   └── sourcemap.rs         // Generazione source map
│   └── utils/
│       ├── mod.rs
│       ├── diagnostics.rs       // Sistema di diagnostica
│       └── config.rs            // Configurazione compilatore
├── runtime/
│   ├── core.ns                  // Libreria runtime core
│   ├── gc.rs                    // Implementazione garbage collector
│   └── async.rs                 // Supporto operazioni asincrone
├── tests/                       // Test di integrazione
└── examples/                    // Esempi di programmi NovaScript
```

### Ottimizzazioni Avanzate

Il compilatore NovaScript implementa ottimizzazioni avanzate per massimizzare le prestazioni:

1. **Analisi del Flusso di Dati**: Traccia come i dati fluiscono attraverso il programma
2. **Alias Analysis**: Determina quando puntatori diversi accedono alla stessa memoria
3. **Strength Reduction**: Sostituisce operazioni costose con equivalenti più efficienti
4. **Loop-invariant Code Motion**: Sposta calcoli fuori dai loop quando possibile
5. **Tail Call Optimization**: Ottimizza le chiamate ricorsive in coda

## Estensioni Future

1. **Compilazione Parallela**: Parsing e generazione di codice multithreaded
2. **Ottimizzazione Whole Program**: Ottimizzazioni cross-modulo
3. **Profile-Guided Optimization**: Uso di feedback runtime per ottimizzare il codice
4. **Integrazione WASM GC**: Uso della proposta WebAssembly Garbage Collection
5. **Ahead-of-Time Compilation**: Compilazione AOT per ambienti server

## Strumenti di Supporto

### Formatter

Il formatter NovaScript applica uno stile di codice consistente, rispettando le regole di formattazione configurabili.

### Linter

Il linter NovaScript analizza staticamente il codice per identificare problemi potenziali, bug e violazioni di stile.

### Analizzatore di Prestazioni

L'analizzatore di prestazioni identifica colli di bottiglia nel codice compilato e suggerisce ottimizzazioni.

### Debugger

Il debugger NovaScript si integra con gli strumenti di sviluppo del browser per fornire un'esperienza di debugging nativa per il codice NovaScript.