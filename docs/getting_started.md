# Primi Passi con NovaScript

Benvenuto in NovaScript! Questa guida ti aiuterà a configurare il tuo ambiente di sviluppo e a creare il tuo primo progetto NovaScript.

## Che cos'è NovaScript?

NovaScript è un linguaggio di programmazione moderno progettato per lo sviluppo web. Combina la flessibilità di JavaScript con migliorie in sicurezza, prestazioni e funzionalità di concorrenza. NovaScript viene compilato in WebAssembly per prestazioni ottimali pur mantenendo una sintassi familiare ed espressiva.

Caratteristiche principali:
- Tipizzazione statica con sistema di tipi graduale
- Indentazione significativa ispirata a Python
- Async di prima classe con concorrenza trasparente
- Sicurezza della memoria con gestione automatica
- Target WebAssembly per prestazioni
- Interoperabilità seamless con JavaScript

## Installazione

### Prerequisiti

Prima di installare NovaScript, assicurati di avere i seguenti prerequisiti:
- Node.js (versione 16 o successiva)
- npm (versione 7 o successiva)

### Installare NovaScript

Installa la toolchain NovaScript usando npm:

```bash
npm install -g novascript
```

Verifica la tua installazione:

```bash
nova --version
```

## Creare il Tuo Primo Progetto

### Utilizzare la CLI

Il modo più semplice per creare un nuovo progetto NovaScript è utilizzare la CLI:

```bash
nova new mio-primo-progetto
cd mio-primo-progetto
```

Questo creerà un nuovo progetto con il template predefinito. Puoi anche specificare un template:

```bash
nova new mia-web-app --template web-app
```

I template disponibili includono:
- `basic` - Un setup minimo di progetto
- `web-app` - Un'applicazione web con una UI di base
- `api` - Un server API RESTful
- `library` - Un pacchetto libreria riutilizzabile

### Struttura del Progetto

Un tipico progetto NovaScript ha la seguente struttura:

```
mio-primo-progetto/
├── nova.config.ns     # Configurazione del progetto
├── package.ns         # Manifest del pacchetto
├── src/
│   ├── index.ns       # Punto di entrata
│   └── lib/           # Codice libreria
├── tests/
│   └── index.test.ns  # File di test
├── public/            # Asset statici
└── README.md          # Documentazione del progetto
```

## Esempio Hello World

Creiamo un semplice programma "Hello World":

1. Crea un nuovo file `src/index.ns`:

```novascript
// src/index.ns

// Importa la libreria standard
import { println } from "Core"

// Definisci una funzione
fn greet(name: string): string
  return "Hello, " + name + "!"

// Punto di entrata principale
fn main()
  // Chiama la funzione greet e stampa il risultato
  let message = greet("NovaScript")
  println(message)

// Esegui la funzione main
main()
```

2. Esegui il programma:

```bash
nova run src/index.ns
```

Dovresti vedere l'output: `Hello, NovaScript!`

## Aggiungere Variabili e Tipi

NovaScript supporta una varietà di tipi con annotazioni di tipo opzionali:

```novascript
// Dichiarazioni di variabili
let name = "Alice"          // Tipo inferito come string
let age: number = 30        // Annotazione di tipo esplicita
const PI = 3.14159          // Valore costante

// Tipi di base
let isActive: boolean = true
let count: number = 42
let message: string = "Hello"
let values: number[] = [1, 2, 3]
let nothing: null = null

// Oggetti e interfacce
interface Person
  name: string
  age: number

let person: Person = { name: "Bob", age: 25 }

// Funzioni con annotazioni di tipo
fn add(a: number, b: number): number
  return a + b

// Tipi generici
class Box<T>
  value: T
  
  constructor(value: T)
    this.value = value
  
  fn getValue(): T
    return this.value

let numberBox = new Box<number>(42)
let stringBox = new Box("Hello")  // Tipo inferito come Box<string>
```

## Controllo di Flusso

NovaScript utilizza l'indentazione significativa per le strutture di controllo del flusso:

```novascript
// Istruzione if
let x = 10

if x > 5
  println("x è maggiore di 5")
else if x < 5
  println("x è minore di 5")
else
  println("x è uguale a 5")

// Ciclo for
for i in range(5)
  println(i)  // Stampa 0, 1, 2, 3, 4

// Ciclo while
let i = 0
while i < 5
  println(i)
  i = i + 1

// Espressione match (pattern matching)
let value = 2

match value
  case 1:
    println("Uno")
  case 2:
    println("Due")
  case 3:
    println("Tre")
  default:
    println("Altro valore")
```

## Funzioni e Lambda

NovaScript supporta sia dichiarazioni di funzioni tradizionali che espressioni lambda:

```novascript
// Dichiarazione di funzione
fn add(a: number, b: number): number
  return a + b

// Espressione lambda
let multiply = (a: number, b: number) -> a * b

// Funzioni di ordine superiore
fn applyOperation(a: number, b: number, operation: (number, number) -> number): number
  return operation(a, b)

let result = applyOperation(5, 3, add)       // 8
let product = applyOperation(5, 3, multiply)  // 15

// Parametri predefiniti
fn greet(name: string = "World"): string
  return "Hello, " + name + "!"

println(greet())         // "Hello, World!"
println(greet("Alice"))  // "Hello, Alice!"

// Parametri rest
fn sum(...numbers: number[]): number
  let total = 0
  for n in numbers
    total += n
  return total

println(sum(1, 2, 3, 4, 5))  // 15
```

## Lavorare con le Collezioni

NovaScript fornisce potenti tipi di collezione e operazioni:

```novascript
// Array
let numbers = [1, 2, 3, 4, 5]
let firstNumber = numbers[0]    // 1
let lastNumber = numbers[4]     // 5
let len = numbers.length        // 5

// Metodi degli array
numbers.push(6)                 // [1, 2, 3, 4, 5, 6]
numbers.pop()                   // [1, 2, 3, 4, 5]
let sliced = numbers.slice(1, 3)  // [2, 3]

// Metodi funzionali
import { map, filter, reduce } from "Array"

let doubled = map(numbers, x => x * 2)                 // [2, 4, 6, 8, 10]
let evens = filter(numbers, x => x % 2 == 0)           // [2, 4]
let sum = reduce(numbers, (acc, x) => acc + x, 0)      // 15

// Map
import { Map } from "Collections"

let userMap = new Map<string, number>()
userMap.set("Alice", 30)
userMap.set("Bob", 25)
println(userMap.get("Alice"))    // 30
println(userMap.has("Charlie"))  // false

// Set
import { Set } from "Collections"

let uniqueNumbers = new Set<number>([1, 2, 2, 3, 3, 3])
println(uniqueNumbers.size)      // 3
println(uniqueNumbers.has(2))    // true
uniqueNumbers.add(4)
uniqueNumbers.delete(1)
```

## Programmazione Asincrona

NovaScript rende la programmazione asincrona semplice con il suo modello di async implicito:

```novascript
// Nessun async/await esplicito necessario
fn fetchData(url: string)
  let response = Http.get(url)  // Implicitamente asincrono
  return response.json()

fn processData()
  // Le chiamate parallele avvengono automaticamente
  let users = fetchData("https://api.example.com/users")
  let products = fetchData("https://api.example.com/products")
  
  // I risultati sono disponibili quando necessario
  println("Users: " + users.length)
  println("Products: " + products.length)

// Gestione errori asincrona
fn fetchWithErrorHandling(url: string)
  try
    let response = Http.get(url)
    return response.json()
  catch e
    println("Errore durante il fetch: " + e.message)
    return { error: true }
```

## Interoperabilità con JavaScript

NovaScript è progettato per una perfetta interoperabilità con JavaScript:

```novascript
// Importare da JavaScript
import { createElement } from "react"

// Esportare per JavaScript
export fn fibonacci(n: number): number
  if n <= 1
    return n
  return fibonacci(n - 1) + fibonacci(n - 2)

// Accedere alle API del browser
fn updateUI()
  let element = document.getElementById("app")
  if element
    element.innerHTML = "<h1>Hello from NovaScript!</h1>"
```

## Prossimi Passi

Ora che hai imparato le basi di NovaScript, puoi:

1. Esplorare la [Libreria Standard](./standard_library.md) per scoprire le funzionalità integrate
2. Approfondire le [Specifiche del Linguaggio](./language_specification.md) per imparare funzionalità avanzate
3. Creare la tua prima app completa seguendo i nostri [tutorial](https://novascript.org/tutorials)
4. Unirti alla [comunità NovaScript](https://community.novascript.org) per fare domande e condividere il tuo lavoro