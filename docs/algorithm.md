# Algorithm (Group B)

Die Aufgaben dieser Gruppe beinhalteten die Bereitstellung der verschiedenen Algorithmen, die mit planaren Graphen arbeiten.
Dazu zählt die Berechnung einer [Baumzerlegung](#tree_decomposition), ...

**Developers:**

- David Martschenko ([@davidmrt98](https://github.com/davidmrt98))
- Timon Pellekoorne ([@TimonPllkrn](https://github.com/TimonPllkrn))
- Pia-Doreen Ritzke ([@pdrtzk](https://github.com/pdrtzk))

## Triangulation

Für verschiedene Algorithmen (bspw. Berechnung einer Baumzerlegung) kann es notwendig werden einen planaren Graphen so zu erweitern (triangulieren), dass dieser maximal planar bzw. chordal wird.
Der implementierte Algorithmus zur Triangulation eines planaren Graphen berechnet lediglich welche Kanten eingefügt werden müssten, damit dies gilt.
Ein bereits maximal planarer graph kann nicht weiter trianguliert werden.
Der Graph muss außerdem zusammenhängend sein, damit er vollständig trianguliert werden kann.


**resources:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/algorithm)

## Span Tree

Der Spannbaum eines Graphen wird ausgehend von einem spezifizierten Knoten errechnet, welcher die Wurzel des Baumes bildet. 


**resources:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/algorithm)

## Dual Graph (Face Tree)

Der duale Graph, genauer Facettenbaum, ordnet jeder Facette des Graphen einen Knoten zu. 
Diese Facetten-Knoten werden mit jenen Knoten verbunden, die eine anliegende Facette repräsentieren.
Dabei werden anliegende Facetten, dessen Kante zum Spannbaum gehört, nicht verbunden und es wird so sichergestellt, dass der entstehende duale Graph keine Kreise hat.
Daher wird der Spannbaum zur Berechnung des dualen Graphen benötigt.
Mit dem dualen Graphen/Facettenbaum kann u. A. eine Baumzerlegung effizient berechnet werden.

**resources:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/algorithm)

## Tree Decomposition 

Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.

**resources:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/algorithm)

## Leveling

Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.

**resources:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/algorithm)