# Algorithm (Gruppe B)

Diese Gruppe hat sich mit der Implementierung des PTAS sowie den dafür notwendigen Algorithmen beschäftigt.

**Entwickler:**

- David Martschenko ([@davidmrt98](https://github.com/davidmrt98))
- Timon Pellekoorne ([@TimonPllkrn](https://github.com/TimonPllkrn))
- Pia-Doreen Ritzke ([@pdrtzk](https://github.com/pdrtzk))

## Triangulation

Für verschiedene Algorithmen (bspw. Berechnung einer Baumzerlegung) kann es notwendig werden, einen planaren Graphen so
zu erweitern (triangulieren), dass dieser maximal planar bzw. chordal wird.
Der implementierte Algorithmus zur Triangulation eines planaren Graphen berechnet lediglich, welche Kanten eingefügt
werden müssten, damit dies gilt.
Ein bereits maximal planarer Graph kann nicht weiter trianguliert werden.
Der Graph muss außerdem zusammenhängend sein, damit er vollständig trianguliert werden kann.

**Materialien:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/triangulation/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/algorithm/triangulation.rs)

## Span Tree

Der Spannbaum eines Graphen wird ausgehend von einem spezifizierten Knoten errechnet, welcher die Wurzel des Baumes
bildet.

**Materialien:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/spantree/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/algorithm/spantree.rs)

## Dual Graph (Face Tree)

Der duale Graph, genauer Facettenbaum, ordnet jeder Facette des Graphen einen Knoten zu.
Diese Facetten-Knoten werden mit jenen Knoten verbunden, die eine anliegende Facette repräsentieren.
Dabei werden anliegende Facetten, dessen Kante zum Spannbaum gehört, nicht verbunden und es wird so sichergestellt, dass
der entstehende duale Graph keine Kreise hat.
Daher wird der Spannbaum zur Berechnung des dualen Graphen benötigt.
Mit dem dualen Graphen/Facettenbaum kann u. A. eine Baumzerlegung effizient berechnet werden.

**Materialien:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/dualgraph/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/algorithm/dualgraph.rs)

## Tree Decomposition

Mithilfe des dualen Graphen (Facettenbaum) und des dazugehörigen Spannbaums ist es möglich, eine Baumzerlegung in
linearer Zeit für einen triangulierten Graphen zu berechnen.
Dazu bilden alle drei Knoten einer Facette im Facettenbaum den jeweiligen Bag der Baumzerlegung.
Zusätzlich werden alle Knoten diesem Bag hinzugefügt, für die es von den Knoten der Facette aus entlang des Spannbaums
einen Pfad bis hin zu Wurzel des Spannbaums gibt.
Die Baumweite dieser Baumzerlegung wird durch die Höhe des Spannbaums (Durchmesser des Graphen) beschränkt.

**Materialien:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/tree_decomposition/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/algorithm/tree_decomposition.rs)

## Leveling

Beim Leveling wird ein planarer Graphen ausgehend von einem Startknoten in verschiedene Level eingeteilt.
Dazu wird der Spannbaum genutzt, dessen Wurzel der Startknoten ist.
Dieser bildet das Level 0.
Jeder Knoten _v_ in einem Level _n_ ist _n_ Knoten von dem Startknoten _s_ entfernt.
Das bedeutet, dass auf einem Pfad _v->s_, der minimal ist, _n-1_ andere Knoten liegen.

Zusätzlich können diese Level in Ringe der Dicke _k_ aufgeteilt werden.
Die Aufteilung beginnt bei Level 0, sodass der letzte Ring weniger als _k_ Level hat, genau dann, wenn die Anzahl der
Level nicht (restlos) durch _k_ teilbar ist.

**Materialien:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/leveling/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/tree/main/src/algorithm/leveling.rs)

## Nice Tree Decomposition

Der Algorithmus für die dynamische Programmierung setzt voraus, dass es sich bei der verwendeten Baumzerlegung um eine "schöne" Baumzerlegung handelt (engl.: *Nice Tree Decomposition*), deren Knoten entweder *Leafs*, *Join*-Knoten, *Introduce*-Knoten oder *Forget*-Knoten sind. Eine gewöhnliche Baumzerlegung lässt sich in linearer Zeit in eine schöne Baumzerlegung transformieren.

**Materialien:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/nice_tree_decomposition/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/blob/main/src/algorithm/nice_tree_decomposition.rs)
- [Reference](https://tcs.rwth-aachen.de/lehre/Graphentheorie/SS2018/oeljeklaus_report.pdf)

## Dynamic Programming

Viele schwere Probleme wie z.B. *Minimum Vertex Cover* oder *Maximum Independent Set* lassen sich mit Hilfe von dynamischer Programmierung in Polynomialzeit lösen, wenn eine (gute) Baumzerlegung des Eingabegraphen vorliegt. Der Algorithmus nimmt einen Graphen und eine schöne Baumzerlegung des Graphen als Parameter an und berechnet bei einem Post-Order-Traversal für jeden Knoten der Baumzerlegung eine Tabelle mit Einträgen für jede Untermenge des Knoten-Bags. Aus der Tabelle des Wurzelknotens lässt sich letztlich die Lösung ablesen.

Der im Code implementierte Algorithmus (`dp_solve`) ist generisch bezüglich des zu lösenden Problems. Es muss nur spezifiziert werden, ob es sich um ein Maximierungs- oder Minimierungsproblem handelt und wie die Tabelleneinträge für die verschiedenen Knotentypen der schönen Baumzerlegung berechnet werden, der Rest wird von der `dp_solve`-Funktion erledigt.

**Materialien:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/dynamic_programming/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/blob/main/src/algorithm/dynamic_programming/solve.rs)
- [Reference](https://tcs.rwth-aachen.de/lehre/Graphentheorie/SS2018/oeljeklaus_report.pdf)
- [Benchmarks](https://thm-mni-ii.github.io/graph-algo-ptas/benchmark/report/)

## PTAS

Der Hauptalgorithmus für das PTAS wird durch die `ptas`-Funktion umgesetzt, die (ähnlich wie `dp_solve`) generisch bezüglich des zu lösenden Problems ist. Neben der Probleminstanz nimmt die Funktion einen `eps`-Wert als Parameter an, über den sich die Approximationsgenauigkeit steuern lässt.

Der Algorithmus erstellt zunächst `k = 1 / eps` Subgraphen, indem bei einer Breitensuche jeweils mit einem Level Versatz jedes $k$-te Level gelöscht wird. Die Subgraphen bestehen somit jeweils aus höchstens $(k - 1)$-außenplanaren Ringen. Die (approximative) Lösung für einen Subgraph wird berechnet, indem für jeden seiner Ringe mit Hilfe von `dp_solve` die optimale Lösung berechnet wird[^1] und anschließend die Vereinigungsmenge gebildet wird (bei Minimierungsproblemem müssen zuätzlich noch die im ersten Schritt rausgelöschten Knoten mit in die Lösung aufgenommen werden). Bei mindestens einem der k Subgraphen wurden höchstens $1/k * n$ Knoten rausgelöscht, die Lösung dieses Graphen ist somit $(1 - eps)$-optimal (bei Minimierungsproblemen $(1 + eps)$-optimal) und wird von der Funktion zurückgegeben.

[^1] Bei diesem Schritt sollte eigentlich ein Algorithmus verwendet werden, der die $k$-Außenplanarität des Graphen nutzt, um in Linearzeit eine Baumzerlegung mit Weite höchstens $3k - 1$ zu berechnen. Da wir zwar die einzelnen Schritte dieses Algorithmus weitestgehend implementiert haben, aber nicht zu einem korrekten Gesamtalgorithmus zusammenfügen konnten, nutzen wir für die Erstellung der Baumzerlegung die `arboretum-td`-Bibliothek. Da hierdurch eine exponentielle Laufzeitabhängigkeit bezüglich `n` entsteht, implementiert die `ptas`-Funktion im strikten theoretischen Sinne kein richtiges PTAS.

**Materialien:**

- [API-Docs](https://thm-mni-ii.github.io/graph-algo-ptas/graph_algo_ptas/algorithm/ptas/index.html)
- [Code](https://github.com/thm-mni-ii/graph-algo-ptas/blob/main/src/algorithm/ptas.rs)
- [Reference](https://tcs.rwth-aachen.de/lehre/Graphentheorie/WS2013/Marius_Knabben.pdf)
- [Benchmarks](https://thm-mni-ii.github.io/graph-algo-ptas/benchmark/report/)