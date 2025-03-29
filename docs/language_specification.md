# Specifiche del Linguaggio NovaScript

## Introduzione

NovaScript è un linguaggio di programmazione moderno progettato per combinare la flessibilità di JavaScript con migliorie in sicurezza, prestazioni e funzionalità di concorrenza. Si compila in WebAssembly ed è progettato sia per ambienti browser che server.

## Caratteristiche Principali

- **Tipizzazione Statica con Sistema di Tipi Graduale**: Annotazioni di tipo opzionali con potente inferenza di tipo
- **Sintassi Moderna**: Indentazione significativa pulita, ispirata a Python
- **Async di Prima Classe**: Async/await impliciti con concorrenza trasparente
- **Sicurezza della Memoria**: Gestione automatica della memoria
- **Target WebAssembly**: Compilato in WebAssembly per prestazioni
- **Interoperabilità**: Seamless JavaScript interop

## Sintassi e Grammatica

### Struttura Lessicale

#### Parole Chiave

```
let, const, fn, if, else, for, while, break, continue, return, import, export, 
try, catch, throw, class, interface, type, enum, as, is, null, true, false
```

#### Identificatori

Gli identificatori iniziano con una lettera (a-z, A-Z) o underscore (_), seguiti da un numero qualsiasi di lettere, cifre o underscore.

#### Letterali

- **Letterali Numerici**: `123`, `123.45`, `0xFF` (hex), `0b1010` (binary)
- **Letterali Stringa**: `"hello"`, `'world'`, `"""stringa multi-linea"""`
- **Letterali Booleani**: `true`, `false`
- **Letterale Null**: `null`

### Grammatica (EBNF)

```
program        ::= statement*

statement      ::= declaration
                | assignment
                | expression
                | if_statement
                | for_statement
                | while_statement
                | return_statement
                | try_statement
                | import_statement
                | export_statement

declaration    ::= 'let' IDENTIFIER (':' type)? ('=' expression)?
                | 'const' IDENTIFIER (':' type)? '=' expression
                | function_declaration
                | class_declaration
                | interface_declaration
                | type_declaration

function_declaration ::= 'fn' IDENTIFIER '(' parameter_list? ')' ('->' type)? block

parameter_list ::= parameter (',' parameter)*
parameter      ::= IDENTIFIER (':' type)?

block          ::= INDENT statement+ DEDENT

if_statement   ::= 'if' expression block ('else' 'if' expression block)* ('else' block)?

for_statement  ::= 'for' IDENTIFIER 'in' expression block

while_statement ::= 'while' expression block

return_statement ::= 'return' expression?

try_statement  ::= 'try' block 'catch' ('(' IDENTIFIER (':' type)? ')')? block ('finally' block)?

import_statement ::= 'import' (IDENTIFIER | '{' import_list '}') 'from' STRING

export_statement ::= 'export' (declaration | '{' export_list '}')

type           ::= basic_type
                | array_type
                | function_type
                | object_type
                | union_type
                | intersection_type

basic_type     ::= 'number' | 'string' | 'boolean' | 'any' | IDENTIFIER

array_type     ::= type '[]'

function_type  ::= '(' parameter_type_list? ')' '->' type

parameter_type_list ::= parameter_type (',' parameter_type)*
parameter_type ::= (IDENTIFIER ':')? type

object_type    ::= '{' (property_type (',' property_type)*)? '}'
property_type  ::= IDENTIFIER (':' type)? ('?')?

union_type     ::= type '|' type
intersection_type ::= type '&' type
```

### Indentazione Significativa

NovaScript utilizza l'indentazione significativa per i blocchi di codice, simile a Python. Ogni livello di indentazione è 2 o 4 spazi (configurabile).

## Sistema di Tipi

### Tipi Primitivi

- `number` - Punto decimale a 64 bit (può essere specificato come `i32`, `i64`, `f32`, `f64` per tipi numerici specifici)
- `string` - Testo codificato UTF-8
- `boolean` - true o false
- `null` - assenza di valore

### Tipi Compositi

- **Array**: `number[]`, `string[][]`
- **Oggetti**: `{ name: string, age: number }`
- **Funzioni**: `(x: number, y: number) -> number`
- **Tuple**: `[string, number, boolean]`
- **Unioni**: `string | number`
- **Intersezioni**: `A & B`
- **Generici**: `Array<T>`, `Map<K, V>`

### Inferenza di Tipo

NovaScript include una potente inferenza di tipo, che permette agli sviluppatori di omettere le annotazioni di tipo dove possono essere determinate:

```
let x = 5           // Inferito come number
let y = "hello"     // Inferito come string
let z = [1, 2, 3]   // Inferito come number[]

fn add(a, b)        // Parametri e tipo di ritorno inferiti dall'uso
  return a + b
```

## Esempi

### Dichiarazione di Variabili

```
let name = "NovaScript"
let age: number = 1
const PI = 3.14159
```

### Funzioni

```
fn greet(name: string): string
  return "Hello, " + name + "!"

fn add(a: number, b: number) -> number
  return a + b
  
// Funzioni arrow
let multiply = (a: number, b: number) -> a * b
```

### Controllo di Flusso

```
if x > 10
  print("x è maggiore di 10")
else if x > 5
  print("x è maggiore di 5 ma non maggiore di 10")
else
  print("x è 5 o meno")

for i in range(10)
  print(i)

let i = 0
while i < 10
  print(i)
  i = i + 1
```

### Gestione degli Errori

```
try
  let result = operazioneRischiosa()
  print(result)
catch e: Error
  print("Si è verificato un errore: " + e.message)
finally
  pulizia()
```

### Classi e Oggetti

```
class Person
  name: string
  age: number
  
  constructor(name: string, age: number)
    this.name = name
    this.age = age
  
  fn greet(): string
    return "Ciao, il mio nome è " + this.name

let person = new Person("Alice", 30)
print(person.greet())
```

### Moduli

```
// math.ns
export fn add(a: number, b: number): number
  return a + b

export fn subtract(a: number, b: number): number
  return a - b

// main.ns
import { add, subtract } from "./math"
let result = add(10, subtract(20, 5))
print(result) // 25
```

### Programmazione Asincrona

NovaScript tratta tutte le operazioni I/O come implicitamente asincrone, senza richiedere parole chiave `async/await` esplicite:

```
fn fetchData(url: string)
  let response = Http.get(url)  // Implicitamente asincrono
  return response.json()

fn processData()
  let data = fetchData("https://api.example.com/data")  // Nessun await necessario
  print(data)

// L'event loop gestisce automaticamente la sospensione e la ripresa
```

## Direttive del Compilatore

```
#pragma optimize("speed")
#pragma target("wasm32")
#pragma include("./helpers.ns")
```

## Operatori

NovaScript supporta un insieme completo di operatori per vari tipi di dati:

### Operatori Aritmetici
- `+` Addizione
- `-` Sottrazione
- `*` Moltiplicazione
- `/` Divisione
- `%` Modulo
- `**` Elevamento a potenza

### Operatori di Confronto
- `==` Uguale a
- `!=` Diverso da
- `<` Minore di
- `>` Maggiore di
- `<=` Minore o uguale a
- `>=` Maggiore o uguale a

### Operatori Logici
- `&&` AND logico
- `||` OR logico
- `!` NOT logico

### Operatori di Assegnazione
- `=` Assegnazione 
- `+=` Assegnazione dopo addizione
- `-=` Assegnazione dopo sottrazione 
- `*=` Assegnazione dopo moltiplicazione
- `/=` Assegnazione dopo divisione

### Operatori Bitwise
- `&` AND bitwise
- `|` OR bitwise
- `^` XOR bitwise
- `~` NOT bitwise
- `<<` Shift a sinistra
- `>>` Shift a destra

### Operatori di Tipo
- `is` Verifica di tipo (oggetto is Tipo)
- `as` Cast di tipo (oggetto as Tipo)

## Caratteristiche Avanzate

### Pattern Matching

```novascript
let result = match value
  case 0:
    "Zero"
  case n if n < 0:
    "Negativo"
  case n if n % 2 == 0:
    "Pari"
  default:
    "Dispari"
```

### Destructuring

```novascript
let [first, second, ...rest] = [1, 2, 3, 4, 5]
let { name, age } = person
```

### Spread Operator

```novascript
let combined = [...array1, ...array2]
let newObj = { ...obj1, ...obj2, extraProp: value }
```

### Generatori

```novascript
fn* fibonacci()
  let a = 0
  let b = 1
  while true
    yield a
    [a, b] = [b, a + b]

for n in fibonacci().take(10)
  println(n)
```

### Decorator

```novascript
@deprecated("Usa newMethod() invece")
fn oldMethod()
  // implementazione...

@logged
class Service
  // proprietà e metodi...
```

### Null Safety

```novascript
let name: string? = null  // Tipo nullable
if name?  // Null check
  println(name.length)

// Optional chaining
let length = user?.profile?.name?.length || 0
```