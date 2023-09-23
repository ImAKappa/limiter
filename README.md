# limiter

Limiter is a exercise program interpreter and management tool

## Exercise Terminology

### Muscle & Muscle Groups

Muscle is a tissue composed of bundles of protein called muscle fibres.
These cells have a special property where they can contract and relax in response to electrical and chemical stimulation.
The contraction and relaxtion of muscles enables people to walk, run, dance, sing, eat, twerk, mime, fight, juggle, digest food, and more.

There are various kinds of muscles, but for exercising we mostly care about skeletal muscle[^1][^2].

[^1]: We care about joints, too. But they are beyond the scope of this discussion. See [Bursae and Tendon Sheaths (Medicine LibreTexts)](https://med.libretexts.org/Bookshelves/Anatomy_and_Physiology/Anatomy_and_Physiology_(Boundless)/8%3A_Joints/8.4%3A_Synovial_Joints/8.4C%3A_Bursae_and_Tendon_Sheaths) for physiology of particular joints, and see [Bruce Lee and the IsoChain (NoLimitSquad)](https://www.youtube.com/watch?v=ALaoOLumD4Y&t=1097s) for a particular technique for strengthening joints.

[^2]: We also care about neuromuscular adaption, but this is also beyond the scope of this discussion. See [Rate Coding Explained (The Movement System)](https://www.youtube.com/watch?v=eJLkrdozT6Q) for an introduction, and [Principles of Neural Science (Kandel et al. 2013)](https://archive.org/details/PrinciplesOfNeuralScienceFifthKANDEL/page/n9/mode/2up), Chapter 34 on Motor Units, for more detail.

See [How Your Muscles Change With Exercise (Institute of Human Anatomy)](https://www.youtube.com/watch?v=2vXOq-aRtYY) for more details.

Muscle groups are groups of muscles. See this webpage on [Muscle Groups (National Cancer Institute)](https://training.seer.cancer.gov/anatomy/muscular/groups/) for more details.

### Exercise

A pattern movement intended to stimulate a muscle/muscle group. Some types of exercises include:

- Compound: An exercise that targets multiple muscle groups, like pull ups
- Isometric: An exercise that involves maintaining the contraction length of your muscles, like pushing a wall with maximum effort 
- Isolation: An exercise that targets a specific muscle groups, like dumbell curls

### Rep

A repetition of an exercise

### Set

A set of repetitions of an exercise

### Superset

A set of sets

### Program

A schedule of various sets and/or supersets, typically grouped by day and muscle groups.




## Usage

```bash
limiter --record pull-day.workout
```

It will then prompt you to select a `day` from the program input

Then it will prompt you for data for each exercise

Then it will produce a CSV file with these columns:

- Program
- Day
- Exercise
- Performance (Reps/Duration, etc) in notation `a/b/c`

Ideally, it does progress reports using Polars


## Workout Notation

See these resources for more detail:

- [Understanding Training Notation (Climbstrong)](https://www.climbstrong.com/education-center/understanding-training-notation/)

### Repetitions (Reps)

```
3x5+5
```

- 3 sets of 5 reps on each side

```
3xRM
```

- 3 sets of Rep Maximum


```
3x5/3/2
```

- 3 sets, but for first set do 5 reps, next 3, then last set do 2 reps


```
3x5-8
```

- 3 sets of a variable number of reps, min 5, max 8

```
3x5(9kg)
3x5(20lbs)
```

- 3 sets of 5 reps, using an 9kg weight
- 3 sets of 5 reps, using a 20lbs weight

### Duration

```
3x40s,15s
3x40s,2min
```

- 3 sets of 40 seconds with 15 second break
- 3 sets of 40 seconds, with 2 minute break


```
3x40s+40s,15s
```

- 3 sets of 40 seconds on each side, with 15 second break

```
3xDM
```

- 3 sets of Duration Maximum

```
3x30s/20s/10s,15s
```

- 3 sets, but each set has a different duration. The break time is 15s between all sets

```
3x30s(9kg),15s
```

- 3 sets of 30 seconds, using a 9kg weight


### Keywords

- `start` start of a program
- `end` end of a program
- `day(#)` day for a program
- `endday` end of a day for a program
- `super` super set context


### Programs

```
start

super(A):
    pushups = 3x25
    archer = 3x10

diamond = 3x8-10
iso_pushup = 3x10s,20s


end
```

## Notes

> The name is a reference to One Punch Man