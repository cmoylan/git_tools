
lib = File.expand_path("../lib", __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require "git_tool/version"

Gem::Specification.new do |spec|
  spec.name          = "git_tool"
  spec.version       = GitTool::VERSION
  spec.authors       = ["Chris Moylan"]
  spec.email         = ["chris@chrismoylan.com"]

  spec.summary       = %q{Tools to automate my git workflow}
  spec.description   = %q{Convenience tools to automate my workflow}
  spec.homepage      = "https://github.com/CTZNcrew/git_tools"

  # Prevent pushing this gem to RubyGems.org. To allow pushes either set the 'allowed_push_host'
  # to allow pushing to a single host or delete this section to allow pushing to any host.
  if spec.respond_to?(:metadata)
    spec.metadata["allowed_push_host"] = "http://example.com"

    spec.metadata["homepage_uri"] = spec.homepage
    spec.metadata["source_code_uri"] = "https://github.com/CTZNcrew/git_tools"
    spec.metadata["changelog_uri"] = "https://github.com/CTZNcrew/git_tools/changelog.org"
  else
    raise "RubyGems 2.0 or newer is required to protect against " \
      "public gem pushes."
  end

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files         = Dir.chdir(File.expand_path('..', __FILE__)) do
    `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  end
  spec.bindir        = "bin"
  spec.executables   = spec.files.grep(%r{^bin/}) { |f| File.basename(f) }
  spec.executables << 'git_tool'
  spec.require_paths = ["lib"]

  spec.add_development_dependency "bundler", "~> 2.3"
  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "rspec", "~> 3.11"
  spec.add_development_dependency "pry", "~> 0.14"
  spec.add_dependency "thor", "~> 1.2"
end