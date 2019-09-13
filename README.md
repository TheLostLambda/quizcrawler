# Quizcrawler

## What is it?
Quizcrawler is an application that, when fed a file of class-notes, crawls the
structure and generates interactive quizzes that can be used as review. It
leverages spaced repetition and forced / active recall to enhance learning. The
gamification of studying should further increase engagement and recall.

## TODO
* Set up some unit tests
* Comments and documentation
* Better / smarter answer checking
* Verify that all of the regex is resonable for borg
* Clean up multi-line strings (get rid of whitespace after '\n')
* A nicer interface with colour and ansi terminal features
* Should games.rs be in core? Seperate the game logic and display?
* Use hashmaps for the flash cards?
* More functional or OO style?
* read_file_as_string exists in the standard library (fs::read_to_string)
* Do I still need the Clone trait in games.rs?
* Error handling! No more unwrap! Move error handling to main.
* Document the purpose of each module. Including mod.rs
