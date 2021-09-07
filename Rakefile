# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

namespace :rutie do
  desc "Build Rust code"
  task :build do
    system "cargo build --release"
  end

  desc "Test Rust code"
  task :test do
    system "cargo test"
  end
end

desc "Run Rust & Ruby testsuites"
task :test do
  result = system "cargo build --release"
  abort unless result

  Rake::Task[:spec].invoke
end

require "rubocop/rake_task"

RuboCop::RakeTask.new

task spec: :test
task default: %i[spec rubocop]
