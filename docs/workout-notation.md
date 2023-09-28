# Workout Notation

## Training Volume

See these resources for more detail:

- [Understanding Training Notation (Climbstrong)](https://www.climbstrong.com/education-center/understanding-training-notation/)
- [James Clear Workout Journal](https://jamesclear.com/workout-journal)
- [Bruce Lee Training Workouts](https://www.thebioneer.com/bruce-lee-training-routines/)

### Reps & Sets

#### Discrete (Unitless) Reps

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

3 sets of a variable number of reps, min 5 and max 8

```
3x5-8
```

3 sets of your Rep Maximum

```
3xRM
```

#### Duration Reps

3 sets of 40 seconds with 15 second break

```
3x40s
```

3 sets of 1 minute, each side

```
3x 1min+1min
```

1 set of 2 minutes and 30 seconds, followed by a 1 minute rest

```
1x 02:30; 1min
```

1 set of 1 hour, 30 minutes, and 5 seconds. Then a 10 minute rest

```
1x 01:30:05; 10min
```

3 sets of Duration Maximum

```
3xDM
```

#### Distance Reps

1 set of 400 meters

```
1x400m
```

### Intensity

3 sets of 80% of your Rep Maximum
```
3xRM @ 80%
```

1 set of 600 meters at 75% intensity

```
1x600m @ 75%
```

3 sets to failure

```
3x!
```

### Load

3 sets of 5 reps, using an 9kg weight

```
3x5(9kg)
```

3 sets of 5 reps, using a 20lbs weight

```
3x5(20lbs)
```

### Rest Time

> Assume all rest time is strict lower limit

3 sets of 8 reps, with a 60 second rest between sets

```
3x8; 60s
```

3 sets of 8 reps, with 60 second rest time between sets and 15 second rest before switching sides

```
3x8+8; 60s +15s
```

3 sets of 40 seconds, with 2 minute break between sets

```
3x40s; 2min
```

3 sets of 40 seconds on each side, with 15 second break between sets

```
3x40s+40s; 15s
```

3 sets, but each set has a different duration. Break for 15s between every set

```
30s/20s/10s; 15s
```

3 sets of 30 seconds, using a 9kg weight. Break for 15s between each set

```
3x30s(9kg); 15s
```


## Exercises

Define exercises using the `ex` keyword. Typically you would define

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

## Records

You record each exercise in one line, like so:

```rust
Exercise|Variation volume {Difficulty} [Notes]
```

For example,

```rust
Pushups|Pike 1x40s; 20s {Hard} [Left elbow felt funny after]
```

Of course, the only mandatory parts are the exercise and volume. At minimum, a record looks like:

```rust
Pushups 3x20; 20s
```

### Compound records

Sometimes, a routine involves performing multiple variations of a base exercise. For example:

```rust
Pushups
    | Pike      1x40s; 20s      {Hard}
    | Diamond   1x10; 20s       {Hard}
    | Explosive 1x10; 20s       {Struggling}
    | _         1x10-12; 20s    {Moderate} // The underscore is a convenient place-holder for the base exercise
```

### Sessions

Use `routine <Session Name> <Date>, <Time> <[Notes]>` to indicate the start of a session, and `end [Time]` for the end

```rust
routine Push 2023-09-22, 20:36 [Lorem ipsum]
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
routine Push 2023-09-28, 15:45 [<https://youtu.be/asIIiww53-w?si=VTHyNoBpETqQyfYU>]
    
    Pushups
    | One_Arm_Assisted  22s+22s; 15s    {Struggling}
    | Archer            22+22s; 15s     {Hard}
    | With_90deg_Hold   45s; 15s        {Hard}
    | Explosive         45s; 15s        {Struggling}
    | Diamond           45s; 15s        {Moderate}
    | _                 45s; 15s        {Moderate}
    | Incline           45s; 15s        {Easy}
    | Knee              45s; 15s        {Easy}

end 16:15
```