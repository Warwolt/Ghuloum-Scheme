from test import compiler_interface as ci

def test_number():
    assert ci.run_program(r"23") == r"23"

def test_character():
    assert ci.run_program(r"#\a") == r"#\a"

def test_true():
    assert ci.run_program(r"#t") == r"#t"

def test_false():
    assert ci.run_program(r"#f") == r"#f"

def test_empty_list():
    assert ci.run_program(r"()") == r"()"
