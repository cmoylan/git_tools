require "git_tool/version"
require 'git_tool/adapters/git'
require "thor"
require 'pry'

module GitTool
  class Error < StandardError; end

  class CLI < Thor
    desc "bn", "Branch New - creates a new branch for a ticket"
    def bn
      ticket_no = ask("Enter ticket number > ").delete("#")

      desc = ask("Enter a short description > ").split(" ").join("_")

      branch_name  = "chris/#{ticket_no}-#{desc}"

      `git stash save "GitTool - stashed to create: #{branch_name}"`
      `git checkout master`
      `git pull`
      `git checkout -b #{branch_name}`

      `emacsclient -q --eval '(work/log-ticket "#{ticket_no}" "#{desc}")'`
    end

    desc "fp", "Force Push - push it real good"
    def fp
      branch_name = `git branch --show-current`
      if branch_name.in? ["master", "release"]
        say "no" and return
      end

      Git.force_push(branch_name)
    end

    desc "ca", "Commit Ammend - write/rewrite"
    def ca
      Git.commit_amend
    end

    SAVE_POSTFIX = "__SAVE"
    desc "bs", "Branch Save - mark branch for saving"
    def bs
      branch_name = Git.current_branch_name

      if branch_name.match(/#{SAVE_POSTFIX}$/)
        new_name = branch_name.gsub(/#{SAVE_POSTFIX}$/, "")
      else
        new_name = "#{branch_name}#{SAVE_POSTFIX}"
      end

      Git.rename_branch(new_name, branch: branch_name)
    end

    desc "rb", "Rebase current branch against master"
    def rb
      asdf=Git.stashed?
      binding.pry
    end

    desc "clean", "Burn it to the ground"
    def clean
    end
  end
end
