# 3D Heat Equation

$$
u(t,x_{1},x_{2},x_{3}), \ \ \ -1 \le x_{1}, x_{2}, x_{3} \le 1
$$

Heat equation.

$$
\frac{\partial u}{\partial t} = \frac{\partial^2 u}{\partial x_{1}^{2}} + \frac{\partial^2 u}{\partial x_{2}^{2}} + \frac{\partial^2 u}{\partial x_{3}^{2}}
$$

Initial conditions.

$$
u(t=0,x_{1},x_{2},x_{3}) = \exp \left\lbrace - 40 (x_{1}^{2} + x_{2}^{2} + x_{3}^{2})\right\rbrace
$$

Boundary Conditions: Dirichlet 0.

## Results

### value at (0,0,0).

$t$, $u(t,0,0,0)$

```
0.0000000000 1.0000000000
0.0001600000 0.7104677132
0.0003200000 0.5379093547
0.0004800000 0.4254157169
0.0006400000 0.3473110512
0.0008000000 0.2904884086
0.0009600000 0.2476303825
0.0011200000 0.2143639142
0.0012800000 0.1879315497
0.0014400000 0.1665174525
0.0016000000 0.1488822082
0.0017600000 0.1341539606
0.0019200000 0.1217034451
0.0020800000 0.1110663043
0.0022400000 0.1018931819
0.0024000000 0.0939167327
0.0025600000 0.0869292613
0.0027200000 0.0807672255
0.0028800000 0.0753002825
0.0030400000 0.0704234080
0.0032000000 0.0660511344
0.0033600000 0.0621132769
0.0035200000 0.0585517215
0.0036800000 0.0553179811
0.0038400000 0.0523713155
0.0040000000 0.0496772708
0.0041600000 0.0472065330
0.0043200000 0.0449340212
0.0044800000 0.0428381633
0.0046400000 0.0409003146
0.0048000000 0.0391042860
0.0049600000 0.0374359605
0.0051200000 0.0358829775
0.0052800000 0.0344344730
0.0054400000 0.0330808641
0.0056000000 0.0318136688
0.0057600000 0.0306253554
0.0059200000 0.0295092152
0.0060800000 0.0284592557
0.0062400000 0.0274701086
```

### Graph

![](graph.png)
