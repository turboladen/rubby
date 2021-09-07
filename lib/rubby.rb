# frozen_string_literal: true

require "rubby/version"
require "rutie"
require "rbs"

# Makin the rubbies oh so good...
module Rubby
  Rutie.new(:rubby).init "init_rubby", __dir__
end
