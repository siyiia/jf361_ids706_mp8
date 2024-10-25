import pandas as pd
import numpy as np
from io import StringIO
from main import analyse_data

test_csv_data = """Factor1,Factor2,Factor3
1,2,3
4,5,6
7,8,9
"""


def test_analyse_data():
    test_df = pd.read_csv(StringIO(test_csv_data))

    expected_means = test_df.mean().tolist()
    expected_std_devs = test_df.std().tolist()

    headers, stats_data = analyse_data(StringIO(test_csv_data))

    assert headers == test_df.columns.tolist(), "Headers do not match."

    np.testing.assert_almost_equal(
        stats_data.loc["mean"].values,
        expected_means,
        decimal=2,
        err_msg="Means do not match.",
    )
    np.testing.assert_almost_equal(
        stats_data.loc["std"].values,
        expected_std_devs,
        decimal=2,
        err_msg="Standard deviations do not match.",
    )

    print("analyse_data test passed.")
