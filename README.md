# PTAS auf Planaren und anderen Graphklassen

<div>

[![CI/CD](https://github.com/thm-mni-ii/graph-algo-ptas/actions/workflows/ci-cd.yml/badge.svg)](https://github.com/thm-mni-ii/graph-algo-ptas/actions/workflows/ci-cd.yml) 
[![Coverage Status](https://coveralls.io/repos/github/thm-mni-ii/graph-algo-ptas/badge.svg?branch=main)](https://coveralls.io/github/thm-mni-ii/graph-algo-ptas?branch=main)

</div>

- Klingt kompliziert, Idee ist aber recht simpel.
- Input: Planarer Graph
- Output: Eine Approximative Lösung für ein Problem wie Vertex Cover mit einer bestimmten Garantie
- PTAS steht für: Polynomial Time Approximation Scheme
- Ein Algorithmus mit folgender Struktur ist ein PTAS:
  - Input: Graph G, Problem P, Float e mit 0 < e < 1
  - Output: Eine Lösung für P auf G als 1+e Approximation
Daher: Wenn die Optimale Lösung eine Größe von OPT hat, liefert der Algorithmus eine Lösung der Größe (1+e)*OPT für Minimierungsprobleme (Vertex Cover).
