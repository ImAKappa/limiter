# limiter

Limiter is a language and exercise management tool.

The goal is to specify a language with which it is convenient to accurately and concisely record workouts.


Additionally, having a formally defined language makes it easier to automatically:

- generate progress reports
- convert records to a long-term storage solutions (like a CSV, spreadsheet, or a database)

> Note: The design might be biased towards calisthenics bc that's what I do

## Usage

```bash
limiter --progress workout-history.wrk
```

## Exercise Terminology

### Muscle & Muscle Groups

Muscle is a tissue composed of bundles of protein called muscle fibres.
These cells have a special property where they can contract and relax in response to electrical and chemical stimulation.
The contraction and relaxtion of muscles enables people to walk, run, dance, sing, eat, twerk, mime, fight, juggle, digest food, and more.

There are various kinds of muscles, but for exercising we mostly care about skeletal muscle[^1][^2]. See [How Your Muscles Change With Exercise (Institute of Human Anatomy)](https://www.youtube.com/watch?v=2vXOq-aRtYY) for more details.

[^1]: We care about joints, too. But they are beyond the scope of this discussion. See [Bursae and Tendon Sheaths (Medicine LibreTexts)](https://med.libretexts.org/Bookshelves/Anatomy_and_Physiology/Anatomy_and_Physiology_(Boundless)/8%3A_Joints/8.4%3A_Synovial_Joints/8.4C%3A_Bursae_and_Tendon_Sheaths) for physiology of particular joints, and see [Bruce Lee and the IsoChain (NoLimitSquad)](https://www.youtube.com/watch?v=ALaoOLumD4Y&t=1097s) for a particular technique for strengthening joints.

[^2]: We also care about neuromuscular adaption, but this is also beyond the scope of this discussion. See [Rate Coding Explained (The Movement System)](https://www.youtube.com/watch?v=eJLkrdozT6Q) for an introduction, and [Principles of Neural Science (Kandel et al. 2013)](https://archive.org/details/PrinciplesOfNeuralScienceFifthKANDEL/page/n9/mode/2up), Chapter 34 on Motor Units, for more detail.

Muscle groups are groups of muscles[^3]. See this webpage on [Muscle Groups (National Cancer Institute)](https://training.seer.cancer.gov/anatomy/muscular/groups/) for more details.

[^3]: [Relevant xkcd](https://xkcd.com/703/)

### Exercise

A movement pattern intended to stimulate a muscle/muscle group. Some types of exercises include:

- Compound: An exercise that targets multiple muscle groups, like pull ups
- Isometric: An exercise that involves maintaining the contraction length of your muscles, like pushing a wall with maximum effort 
- Isolation: An exercise that targets a specific muscle groups, like dumbell curls

Additionally, exercises can be categorized by their aspect:

- Endurance
- Strength: Amount of force your muscles can produce
- Power: Amount of force per unit time your muscles can effect; akin to "explosiveness"
- Flexibility: Ability of your joints to move unrestricted and without pain
- Balance/Coordination: Ability to acheive a smooth, accurate, and controlled motion and/or stable configuration

Some broad categories of exercises that target some combination of the above aspects:

- Aerobics: For endurance and balance/coordination. E.g. running, biking, swimming
- Team Sports: For balance/coordination, endurance, and power. E.g. football, basketball, hockey
- Combat Sports: For endurance, strength, power, flexibility, and balance/coordination. E.g. MMA, Boxing, Brazillian Jiu Jutsu
- Resistance Training: For strength and power. E.g. weight-lifting
- Calisthenics: For strength, and balance/coordination. E.g. pull-ups, pushups, squats
- Stretching: For flexibility
- Yoga: For flexibility and balance/coordination
- Bouldering/Climbing: For balance/coordination, and strength
- Martial Arts: For endurance, strength, power, flexibility, and balance/coordination. E.g. Karate, Kung Fu, Judo

### Rep

A repetition of an exercise

### Set

A set of reps perfomed one after the other.

### Superset

A set of sets performed one after the other.

### Program

A schedule of various sets and/or supersets, geared towards a specific kind of exercise or specific aspect of exercise.

Typically, programs will be divided into days dedicated to a particular collection of muscle groups.
For example "Pull" and "Push" days, or "Upper" and "Lower" days, or even "Full-Body" days.
Just remember never to skip "Leg Day".

## Workout Notation & Markup Language

See these resources for more detail:

- [Understanding Training Notation (Climbstrong)](https://www.climbstrong.com/education-center/understanding-training-notation/)

### Reps & Sets

A set of 5 reps, then a set of 6 reps, then a set of 5 reps

```
5/6/5
```

3 sets of 8 reps

```
3x8
```

3 sets of 5 reps on each side

```
3x5+5
```

3 sets of your Rep Maximum

```
3xRM
```

3 sets of 80% of your Rep Maximum
```
3xRM%80
```

3 sets of a variable number of reps, min 5 and max 8

```
3x5-8
```

3 sets of 5 reps, using an 9kg weight

```
3x5@9kg
```

3 sets of 5 reps, using a 20lbs weight

```
3x5@20lbs
```

3 sets of 8 reps, with a 60 second break

```
3x8; 60s
```

3 sets of 8 reps, with a 15 second break between reps, and a 60 second break between sets

```
3x8; 60s, 15s
```

3 sets of 40 seconds with 15 second break

```
3x40s; 15s
```

3 sets of 40 seconds, with 2 minute break

```
3x40s; 2min
```

3 sets of 40 seconds on each side, with 15 second break between sets

```
3x40s+40s; 15s
```

3 sets of Duration Maximum

```
3xDM
```

3 sets, but each set has a different duration. Break for 15s between every set

```
30s/20s/10s; 15s
```

3 sets of 30 seconds, using a 9kg weight. Break for 15s between each set

```
3x30s@9kg; 15s
```

### Exercises

Define exercises using the `ex` keyword

```js
ex Pushups
```

When referencing an exercise, you can optionally specifiy a variation (you can include whitespace)

```rust
Pushups(Archer)

Pushups(One Arm)

Pushups(Planche)

Pushups(Handstand)

Pushups(Wall)

Pushups(Knee)
```

### Targets vs Actual

You can indicate a target for an exercise using `>>`

```rust
Pushups >> 3x20
```

Record your actual performance using

```rust
Pushups: 12/14/8
```

Optionally, you can specify both the target and actual in one line.
This is more useful if your target is rapidly changing over a few sessions.

```rust
Pushups >> 3x20 : 12/14/18
```

### Sessions

Use `start [Session Name] [Date], [Time]` to indicate the start of a session, and `end [Time]` for the end

```rust
start Push 2023-09-22, 20:36

...

end 21:40
```

### Notes

You can include notes on any line, using `[]`

If you want to include a url in your notes, wrap the url in `<>`

> If you want to use the `<` or `>` symbol literally, then escape using backslash `\`

### Examples

Sample "Push" day

```rust
// push-day-example.wrk

let tempo = 45s; 15s 

start Push 2022-09-23 20:00 [<https://youtu.be/asIIiww53-w?si=VTHyNoBpETqQyfYU>]
    Pushups
    | [One Arm Assisted]: 22s+22s; 15s
    | Archer: tempo
    | [With 90deg Hold]: tempo
    | Explosive: tempo
    | Diamond: tempo
    | Pushups: tempo
    | Incline: tempo
    | Knee: tempo

    Plank >> 2min : 90s
end 20:30
```