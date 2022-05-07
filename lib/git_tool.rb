require 'open3'
require "git_tool/version"
require "thor"

module GitTool
  class Error < StandardError; end
  # Your code goes here...
  #
  #
  class CLI < Thor
    desc "bn", "Branch New - creates a new branch for a ticket"
    def bn
      ticket_no = ask("Enter ticket number > ").delete("#")

      desc = ask("Enter a short description > ").split(" ").join("_")

      branch_name  = "chris/#{ticket_no}-#{desc}"

      puts branch_name
      puts Open3.popen3("pwd")

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

      `git push -f origin #{branch_name}`
    end


    desc "ca", "Commit Ammend - write/rewrite"
    def ca
      `git commit --amend`
    end

    SAVE_POSTFIX = "__SAVE"
    desc "bs", "Branch Save - mark branch for saving"
    def bs
      branch_name = `git branch --show-current`.gsub("\n", "")

      if branch_name.match(/#{SAVE_POSTFIX}$/)
        new_name = branch_name.gsub(/#{SAVE_POSTFIX}$/, "")
      else
        new_name = "#{branch_name}#{SAVE_POSTFIX}"
      end

      `git branch -m #{branch_name} #{new_name}`
    end
  end
end
