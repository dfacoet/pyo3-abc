def test_import_sum_as_string():
    from pyo3_abc import sum_as_string

    assert sum_as_string(1, 2) == "3"


def test_import_pyo3_abc():
    import pyo3_abc
