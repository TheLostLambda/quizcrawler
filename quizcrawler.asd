(defsystem "quizcrawler"
  :version "0.1.0"
  :author "Brooks J Rady"
  :license "Unlicense"
  :depends-on ("cl-ppcre"
               "cl-toml")
  :components ((:module "crawler"
			:components ((:file "util")
				     (:file "data"))))
  :description "Automated Quiz Generator")
