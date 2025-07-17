import numpy as np
import pandas as pd
from cd_diagram import draw_cd_diagram

# def test_draw_cd_diagram():
   
samples = pd.read_csv("tests/test_data.csv", index_col=0, header=0)
draw_cd_diagram(samples, labels=[x.upper() for x in samples.columns], title="TEST", out_file="tests/test.svg")
