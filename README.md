# Quizcrawler

## What is it?
Quizcrawler is an application that, when fed a file of class-notes, crawls the
structure and generates interactive quizzes that can be used as review. It
leverages spaced repetition and forced / active recall to enhance learning. The
gamification of studying should further increase engagement and recall.

## What am I doing now?
  1. Allow quizzes to be started from the tree menu

## Progress
### v1.0.0
  - [3/4] Support multiple question types
    - [x] Flashcard-style
    - [x] Proof / Process / ordered list
    - [x] Topical Details / unordered list
    - [ ] Crude formula questions (label vars)
  - [0/5] Quiz Types
    - [-] Typed answer
      - [ ] Configurable strictness (case-sensitive, etc)
      - [x] "I was right" option
    - [ ] Fill in the blanks
      - [ ] Focus cutting out keywords
    - [-] Multiple choice
      - [x] Basic support
      - [ ] Hint / eliminate some answers
      - [ ] Detect and present similar answers
    - [ ] Check yourself / open
      - [ ] Basic support
      - [ ] Pull out keywords / emphasis
      - [ ] Keep user answer & correct on screen together
    - [ ] Targeted Revision (no answer needed)
      - [ ] Brings up parent / siblings with context of the question
      - [ ] Gives the notes with answer inside, but you need to find it
      - [ ] Go up another level is parent / siblings are too small
  - [1/3] Subject organisation
    - [x] Parse into a tree data structure (files are top level)
    - [x] Allow for menu based exploration of the tree
      - [x] Show actual tree preview
      - [x] Easy keyboard navigation (no return needed)
    - [ ] Allow certain subjects to be bookmarked
      - [ ] Stars out of 5
  - [0/5] Persistence & Metadata
    - [ ] Save tree data / metadata
      - [ ] TOML File
    - [ ] Record / save quiz progress
      - [ ] Save "sessions" (which question was I on?)
    - [ ] Record how many times a question was answered correctly & incorrectly
      - [ ] Have a single "mastery" number (higher -> less asked)
    - [ ] Record the last time questions were asked
    - [ ] Store which types of question each bit of info best fits with
      - [ ] Quiz type preference
      - [ ] Strictness preference
  - [0/5] Interface
    - [ ] Main screen
      - [ ] List of quizzes due for review (spaced reps)
        - [ ] Based on last time seen & mastery
      - [ ] Explore the tree
      - [ ] View the bookmarks / stars ranking
      - [ ] Sort by difficulty / mastery (most needed)
        - [ ] Show mastery percent (inverse difficulty)
    - [ ] Quiz tweaker: Allows for viewing & setting
    - [ ] When asking a question display the tree path for context
    - [ ] More colour coding / fancy terminal witchcraft
      - [ ] Clear screen
      - [ ] No return
      - [ ] Colour by mastery
      - [ ] Colour by right / wrong
      - [ ] Colour answers diffs
      - [ ] Rankings in tree view
      - [ ] Fun effect motivational messages
    - [ ] Motivational & informative messages
      - [ ] After each question
      - [ ] Mastery memes / jokes
  - [0/3] Presentation & Publication
    - [ ] Formal & professional release notes + feature list on Gitlab
    - [ ] Add to CODING section of my website
    - [ ] Produce a video showcasing usage and features

## TODO
* Set up some unit tests
* Comments and documentation
* Better / smarter answer checking
* Verify that all of the regex is resonable for borg
* Clean up multi-line strings (get rid of whitespace after '\n')
* A nicer interface with colour and ansi terminal features
* Use hashmaps for the flash cards?
* More functional or OO style?
* Error handling! No more unwrap! Move error handling to main.
* Document the purpose of each module. Including mod.rs
