# Quiz Dispatcher Design

## Purpose
The role of the quiz dispatcher is to take a list of questions and quizzes, then return an initialized quiz used to ask one of the provided questions.

## Learning Algorithms

### Dealing With Hints
Hints are somewhat more complex than one might imagine. The hints themselves are stored at the level of each question. The idea is to use them to track the amount of assistance that went into each correct answer. This hint value is later used in Dispatcher score calculations. From the perspective of the learning algorithm, hints present a couple of quirks. Firstly, if a hint is used to correctly answer a question, the `correct` count of the question is incremented, but the `last_correct` time and `mastery` are *not* updated. The justification for this is, if you needed a hint, you don't really know the content well-enough to mark it as improved – rather, it's somewhere between right and wrong. Not updating `last_correct` means that you see the question again soon (though not in the same set of quizzes), and `mastery` neither rises nor falls. When calling `.answer()` on a question, a hint fraction is passed in addition to the answer to check. The hint fraction is a representation of how much of a correct answer was enabled by the hint. If a hint totally gives away an answer (which, it shouldn't, but as an example), then the hint value would be 1.0. If it gets you halfway, then it should be 0.5. This value accumulates and deducts from your score. Getting an answer correct after receiving hints that give away half of the answer gives you half points for the answer (correct - hints). **The hint count of a question should not be updated if the answer was incorrect** – you can't have a negative percentage score (0 - hints doesn't make sense).

### Question Completion
How does Quizcrawler decide when a question has been learned and can be removed from the list of questions to ask?

#### Once Correct
Perhaps the simplest approach to the problem: once a question has been answered correctly once, it gets removed. If it is not correctly answered, it stays in the set but gets pushed to the back of the asking queue (questions are sorted by how many times they have been seen in a given quiz, from least to most seen)

#### Mastery Improvement (Alternative?)
This would track the change in mastery of each question. Mastery exists on a scale from 0-10 and this would force the repeated review of a question if you missed it. The complexity comes from what to do when mastery has reached 10 and can no longer be improved.