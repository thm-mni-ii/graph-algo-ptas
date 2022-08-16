# Data Structure (Group A)

Die Aufgaben dieser Gruppe beinhalteten die Bereitstellung der [grundlegenden Datenstrukturen](#data-structures), die [Algorithmen zur Umwandlung in diese Datenstrukturen](#embedding) und 
die [zufällige Generierung von Test Daten](#random-generation). 

**Developers:**

- Jonas-Ian Kuche ([@Zitrone44](https://github.com/Zitrone44))
- Max Stephan ([@mxsph](https://github.com/mxsph))

## Data Structures

Um mit Graphen zu arbeiten, ist es notwendig, diese als Datenstrukturen repräsentiert werden. Dazu wurden zwei Datenstrukturen implementiert.
`LinkGraph`, welche den Graphen mithilfe von `RC` als "Doubly connected edge list" (DCEL) speichert und `ListGraph` welche den Graphen als eine Liste von geordnet Adjacens-Listen speichert und implementiert somit die in [A simple linear time algorithm for embedding maximal planar graphs](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.31.9303&rep=rep1&type=pdf) beschrieben Datenstruktur.

**resources:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/data_structure/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/data_structure)

## Embedding

Um die Planare Einbettung für den Input Graphen zu generieren wurde der Algorithmus [A simple linear time algorithm for embedding maximal planar graphs](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.31.9303&rep=rep1&type=pdf) verwendet. Dieser ermöglicht eine Planare Einbettung von Graphen in **Linearer Zeit**<!--TODO: genaue O(..) angabe-->, ist allerdings auf die Einbettung von Maximal Planaren Graphen beschränkt.

**resources:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/embedding/index.html)  
- [Benchmarks](https://thm-mni-ii.github.io/graph-algo-ptas/benchmark/embed/report/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/embedding)
- [Pseudocode](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.31.9303&rep=rep1&type=pdf)

## Random Generation

Um das Testen der erstellten Algorithmen und Datenstrukturen zu vereinfachen wurde ein Algorithmus zum Erstellen von Maximal Planaren Zufallsgrafen erstellt. Um das Testen und Debuggen zu erleichtern wurde hierbei auch die Angabe eines `Seeds` ermöglicht, welcher dafür sorgt, dass immer der gleiche Graph erstellt wird.

**resources:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/generation/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/generation)
- [Pseudocode](https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.31.9303&rep=rep1&type=pdf)