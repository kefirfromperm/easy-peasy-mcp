import subprocess


def test_check(tmp_path):
    # --check runs load_config and prints the settings
    out = subprocess.run(["app", "--check"], capture_output=True, text=True, cwd=tmp_path)
    assert "port" in out.stdout  # what load_config read
