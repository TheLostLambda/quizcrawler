# Quizcrawler

## What is it?
Quizcrawler is an application that, when fed a file of class-notes, crawls the
structure and generates interactive quizzes that can be used as review. It
leverages spaced repetition and forced / active recall to enhance learning. The
gamification of studying should further increase engagement and recall.

## What am I doing now?
  1. Finishing Quiz implementation
  2. Add tests for question state changes (mastery, correct, etc)
  3. Basic Persistence
  4. Fix MC (Similarity + Diminishing Options)

## Progress
### v0.5.0
  - [x] Support multiple question types
    - [x] Flashcard-style
    - [x] Proof / Process / ordered list
    - [x] Topical Details / unordered list
  - [ ] Quiz Types
    - [ ] Typed answer
      - [ ] Configurable strictness (case-sensitive, etc)
      - [x] "I was right" option
    - [ ] Multiple choice
      - [x] Basic support
      - [ ] Hint / eliminate some answers
      - [ ] Detect and present similar answers
    - [ ] Check yourself / open
      - [ ] Basic support
      - [ ] Pull out keywords / emphasis
      - [ ] Keep user answer & correct on screen together
  - [ ] Subject organisation
    - [x] Parse into a tree data structure (files are top level)
    - [x] Allow for menu based exploration of the tree
      - [x] Show actual tree preview
      - [x] Easy keyboard navigation (no return needed)
  - [ ] Persistence & Metadata
    - [ ] Save tree data / metadata
    - [ ] Record / save quiz progress
      - [ ] Save "sessions" (which question was I on?)
    - [x] Record how many times a question was answered correctly & incorrectly
      - [x] Have a single "mastery" number (higher -> less asked)
    - [ ] Record the last time questions were asked
  - [ ] Interface
    - [ ] Main screen
      - [ ] List of quizzes due for review (spaced reps)
        - [ ] Based on last time seen & mastery
      - [x] Explore the tree
    - [ ] When asking a question display the tree path for context
    - [ ] Colour coding / fancy terminal witchcraft
      - [x] Clear screen
      - [x] No return
      - [ ] Colour by mastery
      - [x] Colour by right / wrong
  - [ ] Presentation & Publication
    - [ ] Formal & professional release notes + feature list on Gitlab
    - [ ] Add to CODING section of my website
    - [ ] Produce a video showcasing usage and features
### v1.0.0
  - [ ] Support more question types
    - [ ] Tables
  - [ ] More Quiz Types
    - [ ] Fill in the blanks
      - [ ] Focus cutting out keywords
    - [ ] Targeted Revision (no answer needed)
      - [ ] Brings up parent / siblings with context of the question
      - [ ] Gives the notes with answer inside, but you need to find it
      - [ ] Go up another level is parent / siblings are too small
  - [ ] Better Subject organisation
    - [ ] Allow certain subjects to be bookmarked
      - [ ] Stars out of 5
  - [ ] More Persistence & Metadata
    - [ ] Store which types of question each bit of info best fits with
      - [ ] Quiz type preference
      - [ ] Strictness preference
  - [ ] More Flexible Interface
    - [ ] Main screen
      - [ ] View the bookmarks / stars ranking
      - [ ] Sort by difficulty / mastery (most needed)
        - [ ] Show mastery percent (inverse difficulty)
    - [ ] Quiz tweaker: Allows for viewing & setting
    - [ ] More colour coding / fancy terminal witchcraft
      - [ ] Colour answers diffs
      - [ ] Rankings in tree view
      - [ ] Fun effect motivational messages
    - [ ] Motivational & informative messages
      - [ ] After each question
      - [ ] Mastery memes / jokes
  - [ ] Presentation & Publication
    - [ ] Formal & professional release notes + feature list on Gitlab
    - [ ] Produce a video showcasing usage and features

## TODO
* Set up some unit tests
* Comments and documentation
* Better / smarter answer checking
* Clean up multi-line strings (get rid of whitespace after '\n')
* A nicer interface with colour and ansi terminal features
* Error handling! No more unwrap! Move error handling to main.
* Document the purpose of each module. Including mod.rs
* The QuizDispatcher should be configurable and pick the most needed questions
