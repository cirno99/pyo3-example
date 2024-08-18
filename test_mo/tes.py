from ejemplo import check_limit
from ejemplo import test_prime

def test_check_limit():
    # flag = check_limit(400)
    flag = check_limit(100)
    print(flag)
    assert flag

def test_prime1():
    # flag = check_limit(400)
    flag = test_prime(100)
    assert flag
test_check_limit();
test_prime1()
