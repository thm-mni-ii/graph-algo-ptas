# Laufzeittests

## Minimum Vertex Cover

Der Linienplot  zeigt, wie sich die Laufzeit der `ptas`-Funktion mit $eps=1/2$ bezüglich der Knotenanzahl ändert:

![](https://thm-mni-ii.github.io/graph-algo-ptas/benchmark/PTAS%20_%20eps=1_2%20_%20Minimum%20Vertex%20Cover/report/lines.svg)

Es liegt ein nahezu linearer Zusammenhang vor. 

Mit $eps=1/3$ lässt sich zwar auch ein linearer Zusammenhang beobachten, aber die Laufzeit ist insgesamt deutlich höher (bei 2000 Knoten werden über 6 Sekunden benötigt; zuvor waren es nur ein wenig über 100 Millisekunden). Dies lässt sich dadurch erklären, dass die Laufzeit des Algorithmus exponentiell in $k$ ist (mit $k=1/eps$):

![](https://thm-mni-ii.github.io/graph-algo-ptas/benchmark/PTAS%20_%20eps=1_3%20_%20Minimum%20Vertex%20Cover/report/lines.svg)

Außerdem wurde getestet, wie sich die Laufzeit des Dynamic-Programming-Algorithmus allein verhält:

![](https://thm-mni-ii.github.io/graph-algo-ptas/benchmark/DP%20_%20Minimum%20Vertex%20Cover/report/lines.svg)

Bis ca. 120 Knoten ist der Algorithmus recht schnell, danach exploziert die Laufzeit jedoch. Eine mögliche Erklärung hierfür ist, dass der Algorithmus der `arboretum-td`-Bibliothek bei kleinen Graphen mit Hilfe von Heuristiken und Reduktionsregeln schnell eine Lösung findet, aber bei größeren Graphen die exponentielle Laufzeit nicht mehr "aufhalten" kann.

## Maximum Independent Set

Die gleichen Test wie im vorherigen Abschnitt wurden für *Maximum Independent Set* durchgeführt. Die Plots sehen sehr ähnlich aus, was verdeutlicht, dass das zugrundeliegende Problem so gut wie keinen Einfluss auf die Laufzeit hat; entscheidend ist die Größe des Graphen und vor allem der gewählte $eps$-Wert.

![](https://thm-mni-ii.github.io/graph-algo-ptas/benchmark/PTAS%20_%20eps=1_2%20_%20Maximum%20Independent%20Set/report/lines.svg)


![](https://thm-mni-ii.github.io/graph-algo-ptas/benchmark/PTAS%20_%20eps=1_3%20_%20Maximum%20Independent%20Set/report/lines.svg)

![](https://thm-mni-ii.github.io/graph-algo-ptas/benchmark/DP%20_%20Maximum%20Independent%20Set/report/lines.svg)