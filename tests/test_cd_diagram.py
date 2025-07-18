import numpy as np
import pandas as pd
from cd_diagram import draw_cd_diagram

# def test_draw_cd_diagram():
   
# samples = pd.read_csv("tests/test_data.csv", index_col=0, header=0)
# draw_cd_diagram(samples, labels=[x.upper() for x in samples.columns], title="TEST", out_file="tests/test.svg")
rng = np.random.default_rng(1)

num_models = 40  # Adjust this number to create as many models as you want
models = {
    f'model{i}': rng.normal(loc=0.1 * i, scale=0.1, size=30)
    for i in range(num_models)
}
models = pd.DataFrame(models)

# it works with both a pandas Dataframe and a numpy array
draw_cd_diagram(models, labels=models.columns.to_list(), out_file="tests/test1.svg")

num_models = 9  # Adjust this number to create as many models as you want
models = {
    f'model{i}': rng.normal(loc=0.1 * i, scale=0.1, size=30)
    for i in range(num_models)
}
models = pd.DataFrame(models)

# it works with both a pandas Dataframe and a numpy array
draw_cd_diagram(models, labels=models.columns.to_list(), out_file="tests/test2.svg")