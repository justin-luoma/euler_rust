### From problem 64:

m<sub>0</sub> = 0

d<sub>0</sub> = 1

a<sub>0</sub> = ⌊√S⌋

m<sub>n+1</sub> = d<sub>n</sub>a<sub>n</sub> - m<sub>n</sub>

d<sub>n+1</sub> = (S - m<sup>2</sup><sub>n+1</sub>) / d<sub>n</sub>

a<sub>n+1</sub> = ⌊ (√S + m<sub>n+1</sub>) / d<sub>n+1</sub> ⌋

### Problem 66:

n = numerator

d = denominator

n<sub>i</sub> = a<sub>i</sub>n<sub>i-1</sub> + n<sub>i-2</sub>

d<sub>I</sub> = a<sub>i</sub>d<sub>i-1</sub> + d<sub>i-2</sub>

#### init:

n<sub>-1</sub> = 1

n<sub>0</sub> = a<sub>0</sub>

d<sub>-1</sub> = 0

d<sub>0</sub> = 1
