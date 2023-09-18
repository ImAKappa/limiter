# limiter

Limiter is a exercise program interpreter and management tool

> The name is a reference to the concept of a limiter from One Punch Man!


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