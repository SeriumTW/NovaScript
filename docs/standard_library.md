# Libreria Standard NovaScript

## Panoramica

La Libreria Standard di NovaScript fornisce un set completo di moduli e funzioni integrate che consentono agli sviluppatori di eseguire attività comuni senza dover implementarle da zero. La libreria standard è progettata per essere:

- **Consistente**: Seguendo pattern di denominazione e comportamento coerenti
- **Efficiente**: Ottimizzata per le prestazioni
- **Sicura**: Prevenendo errori di programmazione comuni
- **Estensibile**: Permettendo agli sviluppatori di costruire su di essa

## Moduli Core

### Modulo `Core`

Costrutti e utility di linguaggio di base.

```novascript
import { Option, Result, println } from "Core"

let maybeValue: Option<string> = Option.some("hello")
println(maybeValue.unwrapOr("default"))  // Stampa: hello

let result: Result<number, string> = Result.ok(42)
if result.isOk()
  println("Valore ottenuto: " + result.unwrap())
```

#### Tipi Core

- `Option<T>`: Rappresenta un valore opzionale (some o none)
- `Result<T, E>`: Rappresenta un risultato che può essere o un successo o un errore
- `Tuple<T1, T2, ...>`: Rappresenta una collezione di dimensione fissa di valori di tipi diversi
- `Function<Args, Return>`: Rappresenta un tipo di funzione

#### Funzioni Core

- `println(value: any)`: Stampa un valore sulla console con una nuova riga
- `print(value: any)`: Stampa un valore sulla console senza una nuova riga
- `assert(condition: boolean, message?: string)`: Asserisce che una condizione sia vera
- `panic(message: string)`: Termina il programma con un messaggio di errore

### Modulo `Math`

Costanti e funzioni matematiche.

```novascript
import { PI, sin, abs, round } from "Math"

println(PI)                // 3.141592653589793
println(sin(PI / 2))       // 1
println(abs(-42))          // 42
println(round(3.7))        // 4
```

#### Costanti Math

- `PI`: La costante matematica π (pi)
- `E`: La costante matematica e (numero di Eulero)
- `TAU`: La costante matematica τ (tau, uguale a 2π)
- `PHI`: Il rapporto aureo φ (phi)

#### Funzioni Math

- Base: `abs`, `ceil`, `floor`, `round`, `trunc`, `sign`
- Esponenziali: `exp`, `log`, `log10`, `log2`, `pow`, `sqrt`
- Trigonometriche: `sin`, `cos`, `tan`, `asin`, `acos`, `atan`, `atan2`
- Iperboliche: `sinh`, `cosh`, `tanh`, `asinh`, `acosh`, `atanh`
- Speciali: `erf`, `gamma`, `beta`
- Random: `random`, `randomInt`, `randomNormal`

### Modulo `String`

Utility per la manipolazione delle stringhe.

```novascript
import { trim, split, join, replace } from "String"

let text = "  Hello, World!  "
println(trim(text))                     // "Hello, World!"
println(split("a,b,c", ","))            // ["a", "b", "c"]
println(join(["a", "b", "c"], "-"))     // "a-b-c"
println(replace("Hello, World!", "World", "NovaScript"))  // "Hello, NovaScript!"
```

#### Funzioni String

- Trasformazione: `toLowerCase`, `toUpperCase`, `trim`, `trimStart`, `trimEnd`, `padStart`, `padEnd`
- Ispezione: `startsWith`, `endsWith`, `includes`, `indexOf`, `lastIndexOf`, `match`
- Manipolazione: `concat`, `slice`, `substring`, `replace`, `replaceAll`, `split`, `join`
- Formattazione: `format`, `template`
- Encoding: `encodeURI`, `decodeURI`, `encodeBase64`, `decodeBase64`

### Modulo `Array`

Utility per la manipolazione degli array.

```novascript
import { map, filter, reduce, zip } from "Array"

let numbers = [1, 2, 3, 4, 5]
let doubled = map(numbers, x => x * 2)                      // [2, 4, 6, 8, 10]
let evens = filter(numbers, x => x % 2 == 0)                // [2, 4]
let sum = reduce(numbers, (acc, x) => acc + x, 0)           // 15
let pairs = zip([1, 2, 3], ["a", "b", "c"])                 // [[1, "a"], [2, "b"], [3, "c"]]
```

#### Funzioni Array

- Trasformazione: `map`, `flatMap`, `flat`, `fill`, `reverse`
- Filtraggio: `filter`, `reject`, `compact`, `unique`
- Aggregazione: `reduce`, `reduceRight`, `every`, `some`, `find`, `findIndex`, `includes`, `indexOf`, `lastIndexOf`
- Combinazione: `concat`, `zip`, `zipWith`, `unzip`
- Slicing: `slice`, `chunk`, `take`, `takeWhile`, `drop`, `dropWhile`
- Ordinamento: `sort`, `sortBy`, `reverse`

### Modulo `Object`

Utility per la manipolazione degli oggetti.

```novascript
import { keys, values, entries, merge } from "Object"

let person = { name: "Alice", age: 30 }
println(keys(person))                             // ["name", "age"]
println(values(person))                           // ["Alice", 30]
println(entries(person))                          // [["name", "Alice"], ["age", 30]]
println(merge(person, { city: "New York" }))      // { name: "Alice", age: 30, city: "New York" }
```

#### Funzioni Object

- Ispezione: `keys`, `values`, `entries`, `hasKey`, `getPath`
- Trasformazione: `map`, `mapKeys`, `mapValues`, `pick`, `omit`, `merge`, `deepMerge`
- Conversione: `fromEntries`, `toJSON`, `fromJSON`

### Modulo `Date`

Utility per la gestione di date e orari.

```novascript
import { now, parse, format, add } from "Date"

let today = now()
println(format(today, "yyyy-MM-dd"))          // es., "2025-03-29"

let birthday = parse("1990-01-15", "yyyy-MM-dd")
let nextWeek = add(today, { days: 7 })
```

#### Funzioni Date

- Creazione: `now`, `parse`, `fromTimestamp`
- Formattazione: `format`, `toISOString`, `toLocaleString`
- Componenti: `getYear`, `getMonth`, `getDate`, `getHours`, `getMinutes`, `getSeconds`
- Manipolazione: `add`, `subtract`, `setYear`, `setMonth`, `setDate`
- Confronto: `isBefore`, `isAfter`, `isEqual`, `diff`

### Modulo `RegExp`

Utility per le espressioni regolari.

```novascript
import { match, test, replace, split } from "RegExp"

let text = "Hello, World!"
let pattern = /Hello/
println(test(pattern, text))                  // true
println(match(pattern, text))                 // ["Hello"]
println(replace(pattern, text, "Hi"))         // "Hi, World!"
println(split(/\s+/, "Hello  World"))        // ["Hello", "World"]
```

#### Funzioni RegExp

- Test: `test`, `match`, `matchAll`
- Sostituzione: `replace`, `replaceAll`
- Splitting: `split`
- Creazione: `compile`, `escape`

## Moduli I/O e Sistema

### Modulo `IO`

Operazioni di input/output.

```novascript
import { readFile, writeFile, exists } from "IO"

if exists("config.json")
  let content = readFile("config.json")
  let config = JSON.parse(content)
  println(config.version)
  
  // Config modificato
  config.version = "2.0"
  writeFile("config.json", JSON.stringify(config, null, 2))
```

#### Funzioni IO

- Operazioni su File: `readFile`, `writeFile`, `appendFile`, `exists`, `remove`, `rename`
- Operazioni su Directory: `readDir`, `createDir`, `removeDir`, `watchDir`
- Manipolazione Percorsi: `joinPath`, `resolvePath`, `dirname`, `basename`, `extname`

### Modulo `Http`

Funzionalità client HTTP.

```novascript
import { get, post, Response } from "Http"

// Richiesta GET
let response = get("https://api.example.com/data")
println(response.status)                           // es., 200
println(response.json())                          // Risposta JSON parsata

// Richiesta POST
let data = { name: "Alice", email: "alice@example.com" }
let postResponse = post("https://api.example.com/users", {
  headers: { "Content-Type": "application/json" },
  body: JSON.stringify(data)
})
```

#### Tipi e Funzioni Http

- `Response`: Rappresenta una risposta HTTP con proprietà come `status`, `headers` e metodi come `text()`, `json()` e `blob()`
- Metodi di Richiesta: `get`, `post`, `put`, `delete`, `head`, `options`, `patch`
- Avanzate: `fetch`, `createRequest`, `abortController`

### Modulo `Process`

Utility per la gestione del processo e dell'ambiente.

```novascript
import { env, args, exit, cwd } from "Process"

println(env.NODE_ENV)                        // es., "development"
println(args)                                // Argomenti riga di comando
println(cwd())                               // Directory di lavoro corrente

if args.includes("--help")
  println("Uso: nova-script [opzioni]")
  exit(0)
```

#### Funzioni Process

- Ambiente: `env`, `args`, `cwd`, `pid`
- Controllo: `exit`, `kill`, `spawn`, `exec`
- Informazioni: `platform`, `arch`, `uptime`, `memoryUsage`

### Modulo `Path`

Utility per la gestione dei percorsi del file system.

```novascript
import { join, resolve, dirname, basename, extname } from "Path"

let filePath = join("path", "to", "file.txt")     // "path/to/file.txt"
println(dirname(filePath))                        // "path/to"
println(basename(filePath))                       // "file.txt"
println(extname(filePath))                        // ".txt"
```

#### Funzioni Path

- Manipolazione: `join`, `resolve`, `normalize`, `relative`, `isAbsolute`
- Componenti: `dirname`, `basename`, `extname`, `parse`, `format`
- Utilità: `sep`, `delimiter`, `toNamespacedPath`

### Modulo `Fs`

Operazioni sul file system.

```novascript
import { readDir, stat, mkdir, writeFile } from "Fs"

let files = readDir("./documents")
for file in files
  let stats = stat(join("./documents", file))
  if stats.isFile()
    println(file + ": " + stats.size + " bytes")

mkdir("./output", { recursive: true })
writeFile("./output/data.txt", "Hello NovaScript!")
```

#### Funzioni Fs

- File: `readFile`, `writeFile`, `appendFile`, `copyFile`, `remove`, `rename`
- Directory: `readDir`, `mkdir`, `rmdir`, `move`
- Informazioni: `stat`, `exists`, `access`, `chmod`, `chown`
- Stream: `createReadStream`, `createWriteStream`, `pipeline`
- Watcher: `watch`, `watchFile`, `unwatchFile`

## Moduli di Rete

### Modulo `Net`

Networking a basso livello.

```novascript
import { createServer, connect } from "Net"

// Server TCP
let server = createServer(socket => {
  socket.write("Hello from NovaScript Server!\n")
  socket.on("data", data => {
    println("Ricevuto: " + data.toString())
    socket.write("Echo: " + data.toString())
  })
})
server.listen(3000)

// Client TCP
let client = connect(3000, "localhost")
client.write("Hello Server!")
```

#### Funzioni Net

- Server: `createServer`, `listen`, `close`
- Client: `connect`, `createConnection`
- Socket: `write`, `end`, `pause`, `resume`, `setTimeout`
- Eventi: `on`, `once`, `off`

### Modulo `Url`

Parsing e formattazione URL.

```novascript
import { parse, format, resolve, URLSearchParams } from "Url"

let url = parse("https://example.com/path?query=value#fragment")
println(url.hostname)                          // "example.com"
println(url.pathname)                          // "/path"
println(url.search)                            // "?query=value"

let formatted = format({
  protocol: "https",
  hostname: "example.com",
  pathname: "/users",
  search: "?sort=asc"
})
println(formatted)                             // "https://example.com/users?sort=asc"
```

#### Funzioni Url

- Parsing: `parse`, `parseQuery`
- Formattazione: `format`, `formatQuery`
- Utilità: `resolve`, `normalize`, `isAbsolute`
- Costruttori: `URL`, `URLSearchParams`

## Moduli di Utility

### Modulo `Crypto`

Funzionalità crittografiche.

```novascript
import { hash, hmac, encrypt, decrypt, randomBytes } from "Crypto"

let data = "Dati sensibili"
let hashed = hash("sha256", data)
println(hashed)                             // Hash SHA-256 in formato esadecimale

let key = randomBytes(32)                   // Chiave di 32 byte
let iv = randomBytes(16)                    // Vettore di inizializzazione di 16 byte
let encrypted = encrypt("aes-256-cbc", data, key, iv)
let decrypted = decrypt("aes-256-cbc", encrypted, key, iv)
println(decrypted)                          // "Dati sensibili"
```

#### Funzioni Crypto

- Hash: `hash`, `hmac`, `pbkdf2`
- Cifratura: `encrypt`, `decrypt`, `createCipher`, `createDecipher`
- Utilità: `randomBytes`, `randomUUID`, `timingSafeEqual`
- Avanzate: `createSign`, `createVerify`, `createDiffieHellman`

### Modulo `Zlib`

Compressione e decompressione.

```novascript
import { compress, decompress, gzip, gunzip } from "Zlib"

let data = "Dati da comprimere ripetuti molte volte..."
let compressed = compress(data)
println("Dimensione originale: " + data.length)
println("Dimensione compressa: " + compressed.length)

let decompressed = decompress(compressed)
println(decompressed == data)                // true
```

#### Funzioni Zlib

- Base: `compress`, `decompress`
- Gzip: `gzip`, `gunzip`
- Deflate: `deflate`, `inflate`
- Brotli: `brotliCompress`, `brotliDecompress`
- Stream: `createCompressStream`, `createDecompressStream`

### Modulo `Events`

Sistema di gestione degli eventi.

```novascript
import { EventEmitter } from "Events"

class Logger extends EventEmitter
  fn log(message: string)
    println("LOG: " + message)
    this.emit("logged", message)

let logger = new Logger()
logger.on("logged", message => {
  println("Messaggio loggato: " + message)
})

logger.log("Test del sistema di eventi")
```

#### Funzioni e Classi Events

- `EventEmitter`: Classe base per gestione eventi
- Metodi: `on`, `once`, `off`, `emit`, `eventNames`, `listenerCount`
- Utilità: `createEventTarget`, `fromDOMEvents`

### Modulo `Stream`

Elaborazione stream di dati.

```novascript
import { Readable, Writable, Transform, pipeline } from "Stream"

class NumbersStream extends Readable
  current: number = 1
  max: number
  
  constructor(max: number = 10)
    super()
    this.max = max
  
  fn _read()
    if this.current <= this.max
      this.push(this.current.toString() + "\n")
      this.current += 1
    else
      this.push(null)  // Fine stream

class UppercaseTransform extends Transform
  fn _transform(chunk: Buffer, encoding: string, callback: Function)
    callback(null, chunk.toString().toUpperCase())

let numbers = new NumbersStream(5)
let uppercase = new UppercaseTransform()
let output = process.stdout

pipeline(numbers, uppercase, output, err => {
  if err
    println("Errore Pipeline: " + err.message)
})
```

#### Classi Stream

- `Readable`: Stream leggibile
- `Writable`: Stream scrivibile
- `Duplex`: Stream sia leggibile che scrivibile
- `Transform`: Stream di trasformazione
- Utility: `pipeline`, `finished`, `compose`

## Moduli UI (Browser)

### Modulo `Dom`

Manipolazione del DOM per ambienti browser.

```novascript
import { querySelector, createElement, append } from "Dom"

fn updateUI(message: string)
  let container = querySelector("#app")
  let paragraph = createElement("p")
  paragraph.textContent = message
  paragraph.classList.add("message")
  append(container, paragraph)
```

#### Funzioni Dom

- Selezione: `querySelector`, `querySelectorAll`, `getElementById`
- Creazione: `createElement`, `createTextNode`, `createDocumentFragment`
- Manipolazione: `append`, `prepend`, `remove`, `replace`, `setAttribute`
- Navigazione: `parent`, `children`, `siblings`, `closest`
- Eventi: `addEventListerner`, `removeEventListener`, `dispatchEvent`