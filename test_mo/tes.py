from ejemplo import check_limit

def test_check_limit():
    # flag = check_limit(400)
    flag = check_limit(100)
    print(flag)
    assert flag

test_check_limit();
