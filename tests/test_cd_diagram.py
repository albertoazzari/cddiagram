import pytest
import numpy as np
import pandas as pd
from cd_diagram import draw_cd_diagram

def test_draw_cd_diagram():
   rng = np.random.default_rng(1)
   size = 30
   values = {
      'model1': rng.normal(loc=0.2, scale=0.1, size=size),
      'model2': rng.normal(loc=0.2, scale=0.1, size=size),
      'model3': rng.normal(loc=0.4, scale=0.1, size=size),
      'model4': rng.normal(loc=0.5, scale=0.1, size=size),
      'model5': rng.normal(loc=0.7, scale=0.1, size=size),
      'model6': rng.normal(loc=0.7, scale=0.1, size=size),
      'model7': rng.normal(loc=0.8, scale=0.1, size=size),
      'model8': rng.normal(loc=0.9, scale=0.1, size=size),
   }

   samples = pd.DataFrame(values, index=[f"Dataset{x+1}" for x in range(size)])

   draw_cd_diagram(samples, labels=[x.upper() for x in samples.columns], title="TEST", out_file="tests/test.svg")