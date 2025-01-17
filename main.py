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