from cd_diagram import cd_diagram
from typing import Optional, Union, List
from typeguard import typechecked
import pandas as pd
import numpy as np
from scipy.stats import friedmanchisquare
from scikit_posthocs import posthoc_nemenyi_friedman

@typechecked
def draw_cd_diagram(samples: Union[pd.DataFrame, np.ndarray], labels: List[str], title: Optional[str]=None,  out_file: Optional[str]=None, fig_size: Optional[tuple[int, int]]=None):
    
    alpha = 0.05

    if isinstance(samples, np.ndarray):
        samples_ = samples.astype(float)
    else:
        samples_ = samples.values.astype(float)

    _, pvalue = friedmanchisquare(*samples_.T)
    if pvalue < alpha:
        # Compute critical difference       
        N, k = samples_.shape  # Number of datasets, # Number of groups
        q_alpha = qstu_0_05[k]
        CD = q_alpha * np.sqrt((k * (k + 1)) / (6 * N))

        # Compute the ranks and sort them
        avg_ranks = pd.DataFrame(samples).rank(ascending=False, axis=1).mean(axis=0).values
        sorted_indices = np.argsort(-avg_ranks)

        # Double check the cliques
        assert np.array_equal(posthoc_nemenyi_friedman(samples_).values > alpha, CD > np.abs(avg_ranks - avg_ranks[:, np.newaxis]), equal_nan=True)

        cd_diagram.cd_diagram(CD, avg_ranks[sorted_indices], [labels[i] for i in sorted_indices], title, out_file, fig_size)

qstu_0_05 = [np.nan, np.nan, 1.959964233, 2.343700476, 2.569032073, 2.727774717, 2.849705382, 2.948319908, 3.030878867, 3.10173026, 3.16368342, 3.218653901, 3.268003591, 3.312738701, 3.353617959, 3.391230382, 3.426041249, 3.458424619, 3.488684546, 3.517072762, 3.543799277, 3.569040161, 3.592946027, 3.615646276, 3.637252631, 3.657860551, 3.677556303, 3.696413427, 3.71449839, 3.731869175, 3.748578108, 3.764671858, 3.780192852, 3.795178566, 3.809663649, 3.823679212, 3.837254248, 3.850413505, 3.863181025, 3.875578729, 3.887627121, 3.899344587, 3.910747391, 3.921852503, 3.932673359, 3.943224099, 3.953518159, 3.963566147, 3.973379375, 3.98296845, 3.992343271, 4.001512325, 4.010484803, 4.019267776, 4.02786973, 4.036297029, 4.044556036, 4.05265453, 4.060596753, 4.068389777, 4.076037844, 4.083547318, 4.090921028, 4.098166044, 4.105284488, 4.112282016, 4.119161458, 4.125927056, 4.132582345, 4.139131568, 4.145576139, 4.151921008, 4.158168297, 4.164320833, 4.170380738, 4.176352255, 4.182236797, 4.188036487, 4.19375486, 4.199392622, 4.204952603, 4.21043763, 4.215848411, 4.221187067, 4.22645572, 4.23165649, 4.236790793, 4.241859334, 4.246864943, 4.251809034, 4.256692313, 4.261516196, 4.266282802, 4.270992841, 4.275648432, 4.280249575, 4.284798393, 4.289294885, 4.29374188, 4.298139377, 4.302488791] 