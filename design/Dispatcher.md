# Quiz Dispatcher Design

## Purpose
The role of the quiz dispatcher is to take a list of questions and quizzes, then return an initialized quiz used to ask one of the provided questions.

## Learning Algorithms

### Question Completion
How does Quizcrawler decide when a question has been learned and can be removed from the list of questions to ask?

#### Once Correct
Perhaps the simplest approach to the problem: once a question has been answered correctly once, it gets removed. If it is not correctly answered, it stays in the set but gets pushed to the back of the asking queue (questions are sorted by how many times they have been seen in a given quiz, from least to most seen)

#### Mastery Improvement (Alternative?)
This would track the change in mastery of each question. Mastery exists on a scale from 0-10 and this would force the repeated review of a question if you missed it. The complexity comes from what to do when mastery has reached 10 and can no longer be improved.