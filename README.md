# NovaScript

<p align="center">
  <em>Un linguaggio di programmazione moderno per il web, compilato in WebAssembly</em>
</p>

<p align="center">
  <strong>Veloce • Sicuro • Espressivo</strong>
</p>

---

## 🌟 Panoramica

NovaScript è un linguaggio di programmazione moderno progettato per essere un concorrente diretto di JavaScript, combinando la flessibilità di JavaScript con migliorie in sicurezza, prestazioni e funzionalità di concorrenza. Viene compilato in WebAssembly per ottenere prestazioni quasi native pur mantenendo una sintassi familiare ed espressiva.

```novascript
// Esempio Hello World
import { println } from "Core"

fn greet(name: string): string
  return "Hello, " + name + "!"

fn main()
  let message = greet("NovaScript")
  println(message)  // Output: Hello, NovaScript!
```

## ✨ Caratteristiche Principali

- **Tipizzazione Statica con Sistema di Tipi Graduale**: Annotazioni di tipo opzionali con potente inferenza di tipo
- **Sintassi Moderna**: Indentazione significativa pulita, ispirata a Python
- **Async di Prima Classe**: Async/await impliciti con concorrenza trasparente
- **Sicurezza della Memoria**: Gestione automatica della memoria
- **Target WebAssembly**: Compilato in WebAssembly per prestazioni
- **Interoperabilità**: Seamless JavaScript interop

## 📚 Documentazione

- [Primi Passi](./docs/getting_started.md) - Un'introduzione rapida a NovaScript
- [Specifiche del Linguaggio](./docs/language_specification.md) - Documentazione dettagliata del linguaggio
- [Libreria Standard](./docs/standard_library.md) - Panoramica dei moduli e delle funzioni integrate
- [Architettura del Compilatore](./docs/compiler_architecture.md) - Come funziona il compilatore NovaScript
- [Runtime](./docs/runtime.md) - Dettagli sul runtime NovaScript
- [Strumenti](./docs/tools.md) - Strumenti e utilità di sviluppo

## 🚀 Primi Passi

### Installazione

```bash
npm install -g novascript
```

### Creare un Nuovo Progetto

```bash
nova new my-project
cd my-project
```

### Eseguire un Programma NovaScript

```bash
nova run src/index.ns
```

## 🔍 Esempi

### Variabili e Funzioni

```novascript
// Dichiarazioni di variabili con inferenza di tipo
let name = "Alice"
let age: number = 30
const PI = 3.14159

// Dichiarazione di funzione
fn add(a: number, b: number): number
  return a + b

// Espressione lambda
let multiply = (a: number, b: number) -> a * b
```

### Controllo di Flusso

```novascript
// Istruzioni condizionali
if age >= 18
  println("Adulto")
else
  println("Minorenne")

// Cicli
for i in range(5)
  println(i)

let i = 0
while i < 5
  println(i)
  i = i + 1
```

### Classi e Interfacce

```novascript
// Interfaccia
interface Shape
  area(): number

// Implementazione della classe
class Rectangle implements Shape
  width: number
  height: number
  
  constructor(width: number, height: number)
    this.width = width
    this.height = height
  
  fn area(): number
    return this.width * this.height

// Creazione di un'istanza
let rect = new Rectangle(5, 3)
println(rect.area())  // Output: 15
```

### Programmazione Asincrona

```novascript
// Nessun async/await esplicito necessario
fn fetchData(url: string)
  let response = Http.get(url)  // Implicitamente asincrono
  return response.json()

fn processData()
  // L'esecuzione parallela avviene automaticamente
  let users = fetchData("https://api.example.com/users")
  let products = fetchData("https://api.example.com/products")
  
  // Risultati disponibili quando necessario
  println("Users: " + users.length)
  println("Products: " + products.length)
```

## 🔧 Strumenti

NovaScript viene fornito con un set completo di strumenti di sviluppo:

- **Compilatore**: Trasforma NovaScript in WebAssembly
- **Gestore Pacchetti**: Gestisce dipendenze e progetti
- **Framework di Test**: Scrivere ed eseguire test
- **Formattatore**: Stile di codice consistente
- **Linter**: Controllo della qualità del codice
- **Server di Sviluppo**: Server di sviluppo con hot reloading

## 🤝 Contribuire

Accogliamo con favore i contributi a NovaScript! Si prega di consultare le nostre [linee guida per i contributi](./docs/CONTRIBUTING.md) per iniziare.

## 📄 Licenza

NovaScript è concesso in licenza sotto la [Licenza MIT](./LICENSE).

---

<p align="center">
  Fatto con ❤️ dal team NovaScript
</p>