# frozen_string_literal: true

require 'test/unit'
require 'playground'

include Test::Unit::Assertions

assert_raise Playground::PlaygroundError::IntegerOverflow do
  Playground.add 18_446_744_073_709_551_615, 1
end

assert_equal Playground.add(2, 4), 6
assert_equal Playground.add(4, 8), 12

assert_raise Playground::PlaygroundError::IntegerOverflow do
  Playground.sub 0, 1
end

assert_equal Playground.sub(4, 2), 2
assert_equal Playground.sub(8, 4), 4
assert_equal Playground.div(8, 4), 2

assert_raise Playground::InternalError do
  Playground.div 8, 0
end

assert Playground.equal(2, 2)
assert Playground.equal(4, 4)

assert !Playground.equal(2, 4)
assert !Playground.equal(4, 8)
