from app.testing import load_config, write_config


def test_reads_the_port(tmp_path):
    path = write_config(tmp_path, port=8080)
    assert load_config(path).port == 8080


def test_requires_the_port(tmp_path):
    path = write_config(tmp_path)
    assert load_config(path) is None
