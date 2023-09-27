# limiter

Limiter is a exercise program interpreter and management tool

## Usage

```bash
limiter --record push-program.wrk
```

The program then prompts you to select a `day` from the program input,
and to record your performance for each exercise.

Next, it produces a CSV file to store your data.
From these CSV files, `limiter` can generate progress reports.

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
3x80%RM
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

### Programs

We can define programs using a simple markup language

> Markup languages are good for describing the _structure_ of something


Time
- Weeks
- Days
- Durations

Progressions & Variations
```rust
// exercise without variations
ex pushups

// Variations are unordered!
// You can optionally specify a default
// ex <ident>[<default>](<variation>)
ex pushups[Regular](Wall, Knee, Regular, Archer, IsoLower, IsoUpper, Diamond, Handstand, Planche, OneArm)
```

```rust
// push-plan-example.wrk
program Push
    week#1
        day#1
            super A
                pushups(Archer): 3x12
                pushups: 3x10
            end

            super B
                pushups(Diamond): 3x8-10
                pushups(IsoLow): 3x10s; 20s
            end

            pushups(OneArm): 3x6s+6s; 15s
        end

        day#2
            ...
        end
    end
end
```
