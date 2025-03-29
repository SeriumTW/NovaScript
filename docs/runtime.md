# Runtime NovaScript

## Panoramica

Il Runtime NovaScript è responsabile dell'esecuzione del codice NovaScript sia in ambienti browser che server. Fornisce gestione della memoria, un event loop per operazioni asincrone e integrazione con la piattaforma host.

## Componenti Core

### 1. Gestione della Memoria

Il Runtime NovaScript include un sistema di gestione della memoria che:

- **Garbage Collection**: Recupera automaticamente la memoria che non è più in uso
- **Allocazione della Memoria**: Alloca efficientemente memoria per oggetti, array e altre strutture dati
- **Tracciamento dei Riferimenti**: Monitora i riferimenti per prevenire memory leak
- **Compattazione Heap**: Riduce la frammentazione della memoria

Quando si prende come target WebAssembly, il runtime può:
- Utilizzare la proposta WebAssembly GC se disponibile nell'ambiente target
- Ripiegare su un garbage collector mark-and-sweep personalizzato per ambienti senza supporto GC nativo

```rust
struct MemoryManager {
    // Pool di memoria per diverse dimensioni di allocazione
    small_objects: Pool,
    medium_objects: Pool,
    large_objects: Vec<Allocation>,
    
    // Statistiche di garbage collection
    last_gc_time: Instant,
    allocations_since_gc: usize,
    
    // Riferimenti a oggetti root
    roots: Vec<Reference>,
}

impl MemoryManager {
    fn allocate(&mut self, size: usize) -> *mut u8 {
        // Alloca memoria della dimensione richiesta
        // Attiva la garbage collection se necessario
    }
    
    fn collect_garbage(&mut self) {
        // Fase di Mark: Traccia tutti gli oggetti raggiungibili
        // Fase di Sweep: Recupera memoria per gli oggetti non raggiungibili
    }
}
```

### 2. Sistema Event Loop

NovaScript presenta un event loop integrato che gestisce operazioni asincrone in modo trasparente:

- **Multitasking Cooperativo**: Permette a più task di essere eseguiti contemporaneamente senza sintassi async/await esplicita
- **Scheduling dei Task**: Dà priorità e schedula i task in modo efficiente
- **Sospensione e Ripresa dei Task**: Sospende e riprende automaticamente i task ai punti di I/O
- **Integrazione con Promise**: Si integra in modo trasparente con le Promise JS quando eseguito nei browser

```rust
struct EventLoop {
    tasks: VecDeque<Task>,
    pending_io: HashMap<IoHandle, Vec<TaskId>>,
    completed_io: VecDeque<(IoHandle, Result<Value, Error>)>,
    timers: BinaryHeap<Timer>,
    microtasks: VecDeque<TaskId>,
}

impl EventLoop {
    fn enqueue_task(&mut self, task: Task) {
        self.tasks.push_back(task);
    }
    
    fn run_tick(&mut self) {
        // Processa operazioni I/O completate
        // Esegue microtask
        // Esegue task in sospeso finché non cedono o completano
        // Processa timer
        // Attende eventi I/O se non ci sono task in sospeso
    }
    
    fn suspend_for_io(&mut self, task_id: TaskId, io_handle: IoHandle) {
        // Sposta il task da attivo a in attesa, aspettando I/O
    }
}
```

### 3. Libreria Standard

Il Runtime NovaScript fornisce una libreria standard completa:

- **Tipi Core**: Implementazioni di string, number, array, object, ecc.
- **Collections**: Map, set, queue, ecc.
- **Operazioni I/O**: File system, network, ecc.
- **Primitive di Concorrenza**: Channel, mutex, ecc.
- **Utility**: Espressioni regolari, gestione di data/ora, ecc.

### 4. Interoperabilità con l'Host

Il runtime fornisce interoperabilità con l'ambiente host:

- **Interop JavaScript**: Quando eseguito nei browser, il codice NovaScript può interagire con le API JavaScript
- **Accesso al DOM**: Accesso diretto al DOM e alle API del browser
- **Integrazione Node.js**: Quando eseguito su server, integrazione con le API Node.js
- **Moduli Nativi**: Supporto per caricare e utilizzare moduli nativi

```rust
struct JsInterop {
    js_values: HashMap<JsValueId, Value>,
    callback_registry: HashMap<CallbackId, Function>,
}

impl JsInterop {
    fn call_js_function(&mut self, name: &str, args: Vec<Value>) -> Result<Value, Error> {
        // Marshalla i valori NovaScript in valori JS
        // Chiama la funzione JS
        // Marshalla il risultato di nuovo a un valore NovaScript
    }
    
    fn register_callback(&mut self, callback: Function) -> CallbackId {
        // Registra una funzione NovaScript per essere chiamabile da JS
    }
}
```

## Modello di Esecuzione

### 1. Esecuzione delle Funzioni

Le funzioni NovaScript sono eseguite in una macchina virtuale stack-based:

- **Record di Attivazione Funzione**: Tracciamento delle chiamate di funzione e variabili locali
- **Gestione dello Stack**: Gestione efficiente dello stack di esecuzione
- **Gestione delle Eccezioni**: Supporto per blocchi try/catch e propagazione degli errori

```rust
struct CallFrame {
    function: Function,
    pc: usize,          // Program counter
    locals: Vec<Value>, // Variabili locali
    stack: Vec<Value>,  // Stack operandi
}

struct ExecutionContext {
    call_stack: Vec<CallFrame>,
    global_scope: Environment,
    current_exception: Option<Value>,
}
```

### 2. Esecuzione Asincrona

Il modello async trasparente di NovaScript funziona come segue:

1. Il compilatore identifica potenziali punti di sospensione (operazioni I/O, timer, ecc.)
2. A runtime, quando si incontra un punto di sospensione:
   - Lo stato del task corrente viene salvato (program counter, variabili locali, ecc.)
   - Il task viene sospeso e aggiunto a una coda di attesa
   - L'event loop continua a eseguire altri task
3. Quando l'operazione awaited è completata:
   - Il task viene spostato dalla coda di attesa alla coda di esecuzione
   - Lo stato del task viene ripristinato
   - L'esecuzione continua dal punto di sospensione

```rust
enum TaskState {
    Running,
    Suspended { resume_pc: usize, awaited_operation: OperationId },
    Completed { result: Result<Value, Error> },
}

struct Task {
    id: TaskId,
    state: TaskState,
    call_stack: Vec<CallFrame>,
    environment: Environment,
}
```

## Ottimizzazioni

Il Runtime NovaScript include varie ottimizzazioni:

1. **Just-In-Time Compilation**: Compila dinamicamente funzioni hot in codice nativo
2. **Inline Caching**: Ottimizza l'accesso alle proprietà e le chiamate ai metodi
3. **Hidden Classes**: Ottimizza l'accesso alle proprietà degli oggetti
4. **Escape Analysis**: Elimina allocazioni non necessarie
5. **Tail Call Optimization**: Ottimizza le chiamate di funzione ricorsive

## Debugging e Profiling

Il runtime fornisce strumenti per debugging e profiling:

1. **Source Map**: Mappa il codice WebAssembly al codice sorgente NovaScript
2. **Stack Trace**: Fornisce stack trace dettagliati per gli errori
3. **Memory Profiling**: Identifica memory leak e uso inefficiente della memoria
4. **CPU Profiling**: Identifica hot spot e colli di bottiglia delle prestazioni
5. **Remote Debugging**: Supporta il debugging remoto delle applicazioni NovaScript

## Funzionalità di Sicurezza

Il Runtime NovaScript include funzionalità di sicurezza:

1. **Sandboxing**: Isola il codice NovaScript dall'ambiente host
2. **Sicurezza della Memoria**: Previene corruzione della memoria e buffer overflow
3. **Interfaccia Funzione Esterna Sicura**: Chiama in modo sicuro codice nativo
4. **Limitazioni delle Risorse**: Limita l'uso della memoria, tempo CPU, ecc.

## Integrazione con Strumenti di Sviluppo

Il runtime si integra con vari strumenti di sviluppo:

1. **Protocollo Debugger**: Supporta protocolli di debugging standard
2. **Inspector**: Fornisce un inspector runtime per esaminare lo stato dell'applicazione
3. **Monitoraggio delle Prestazioni**: Fornisce metriche per il monitoraggio delle prestazioni
4. **Logging**: Supporta logging strutturato per monitoraggio e debugging

## Implementazione del Runtime

### 1. Core Runtime

Il core runtime gestisce le funzionalità di base necessarie per l'esecuzione del codice NovaScript:

```rust
struct NovaRuntime {
    // Gestione della memoria
    memory_manager: MemoryManager,
    
    // Event loop per gestire concorrenza e I/O
    event_loop: EventLoop,
    
    // Tabella dei moduli caricati
    modules: HashMap<String, Module>,
    
    // Ambiente globale
    global_environment: Environment,
    
    // Interop con la piattaforma host
    platform: PlatformInterface,
}

impl NovaRuntime {
    // Inizializza il runtime
    fn new() -> Self { /* ... */ }
    
    // Carica ed esegue un modulo
    fn load_and_run(&mut self, module_path: &str) -> Result<Value, Error> { /* ... */ }
    
    // Esegue l'event loop
    fn run_until_completion(&mut self) -> Result<(), Error> { /* ... */ }
    
    // Chiama una funzione NovaScript dall'host
    fn call_function(&mut self, module: &str, fn_name: &str, args: Vec<Value>) -> Result<Value, Error> { /* ... */ }
}
```

### 2. Modulo WebAssembly

Il modulo WebAssembly del runtime carica ed esegue moduli WebAssembly compilati:

```rust
struct WasmModule {
    // Instance del modulo WebAssembly
    instance: WasmInstance,
    
    // Tabella di esportazione
    exports: HashMap<String, WasmExport>,
    
    // Tabella delle funzioni
    function_table: Vec<WasmFunction>,
    
    // Memoria WASM
    memory: WasmMemory,
}

impl WasmModule {
    // Carica un modulo WebAssembly da bytecode
    fn load(runtime: &NovaRuntime, bytecode: &[u8]) -> Result<Self, Error> { /* ... */ }
    
    // Chiama una funzione esportata
    fn call_export(&mut self, fn_name: &str, args: Vec<Value>) -> Result<Value, Error> { /* ... */ }
}
```

### 3. Gestione delle Risorse

Il runtime gestisce le risorse del sistema in modo efficiente:

```rust
struct ResourceManager {
    // Gestione dei file aperti
    open_files: HashMap<FileHandle, File>,
    
    // Gestione delle connessioni di rete
    network_connections: HashMap<ConnectionHandle, Connection>,
    
    // Timer
    active_timers: BinaryHeap<Timer>,
}

impl ResourceManager {
    // Apre un file e restituisce un handle
    fn open_file(&mut self, path: &str, mode: FileMode) -> Result<FileHandle, Error> { /* ... */ }
    
    // Crea una connessione di rete
    fn connect(&mut self, address: &str, port: u16) -> Result<ConnectionHandle, Error> { /* ... */ }
    
    // Crea un timer
    fn create_timer(&mut self, delay_ms: u64, callback: Callback) -> TimerId { /* ... */ }
}
```

### 4. Sistema di Moduli

Il runtime implementa un sistema di moduli per la gestione delle dipendenze:

```rust
struct ModuleSystem {
    // Moduli caricati
    loaded_modules: HashMap<String, Module>,
    
    // Risolutore dei percorsi
    path_resolver: PathResolver,
    
    // Loader per diversi tipi di moduli
    loaders: HashMap<String, Box<dyn ModuleLoader>>,
}

impl ModuleSystem {
    // Carica un modulo per nome
    fn require(&mut self, module_name: &str) -> Result<Module, Error> { /* ... */ }
    
    // Risolve un percorso di modulo
    fn resolve_path(&self, base_path: &str, module_path: &str) -> String { /* ... */ }
}
```

## Interoperabilità API del Browser

Il runtime NovaScript fornisce accesso alle API del browser attraverso un layer di astrazione che mantiene la sicurezza dei tipi del sistema di tipi NovaScript:

```novascript
// Utilizzo di API DOM in NovaScript
import { document } from "Browser"

fn updateHeader(text: string): void
  let header = document.getElementById("header")
  if header
    header.textContent = text
```

## Deployment delle Applicazioni NovaScript

Il runtime NovaScript supporta vari scenari di deployment:

1. **Web Applications**: Caricamento ed esecuzione nel browser, con integrazione WebAssembly
2. **Server-side Applications**: Esecuzione in ambiente Node.js o runtime dedicato
3. **Embedded Applications**: Utilizzo di NovaScript in altri progetti o applicazioni
4. **Command-line Tools**: Creazione di strumenti da riga di comando potenti ed efficienti