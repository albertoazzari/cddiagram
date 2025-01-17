# Critical Difference (CD) Diagram

This repository provides a tool for drawing Critical Difference (CD) diagrams, widely used to visualize and compare the performance ranks of multiple models or algorithms. 

The process starts by performing the Friedman Chi-Square test to determine if there are significant differences between the models. 
If significant differences are found, the Nemenyi post-hoc test is applied to identify which model pairs differ significantly.

The resulting CD diagram illustrates the average ranks of the models, highlighting groups of models that are not significantly different. 

This tool is ideal for researchers and practitioners who need to create clear, publication-ready visualizations for statistical model comparisons.

## Usage
```python
from cd_diagram import draw_cd_diagram
import numpy as np
import pandas as pd


rng = np.random.default_rng(1)

models = {
    'model1': rng.normal(loc=0.2, scale=0.1, size=30),
    'model2': rng.normal(loc=0.2, scale=0.1, size=30),
    'model3': rng.normal(loc=0.4, scale=0.1, size=30),
    'model4': rng.normal(loc=0.5, scale=0.1, size=30),
    'model5': rng.normal(loc=0.7, scale=0.1, size=30),
    'model6': rng.normal(loc=0.7, scale=0.1, size=30),
    'model7': rng.normal(loc=0.8, scale=0.1, size=30),
    'model8': rng.normal(loc=0.9, scale=0.1, size=30),
}

models = pd.DataFrame(models)

# it works with both a pandas Dataframe and a numpy array
draw_cd_diagram(models, labels=models.columns.to_list(), out_file="out.svg")
```
<img src="./out.svg">