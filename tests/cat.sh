# basic usage
check_equal 'cat'           'foo'
check_equal 'cat'           'foo\n'
check_equal 'cat'           'foo\nbar'

# -n
check_equal 'cat -n'        'foo\nbar'

# errors
check_equal 'cat --no-such-param'
