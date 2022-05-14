# lib/git.rb

module GitTool
  module Adapters
    module Git
      def current_branch_name
        `git branch --show-current`.gsub("\n", "")
      end

      def rename_branch(new_name:, branch: current_branch_name)
        `git branch -m #{branch} #{new_name}`
      end

      def commit_amend
        `git commit --amend`
      end

      def force_push(branch: current_branch_name, remote: "origin")
        `git push -f #{remote} #{branch}`
      end

      def stashed?
        `git stash list`
      end
    end
  end
end
