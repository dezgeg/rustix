check_equal 'echo'
check_equal 'echo foo'
check_equal 'echo foo     bar'

check_equal 'echo -n'
check_equal 'echo -n foo'

check_equal 'echo -e foo'
