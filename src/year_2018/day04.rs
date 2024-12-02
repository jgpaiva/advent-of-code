use std::{collections::HashMap, error::Error, str::FromStr};

#[cfg(test)]
use crate::utils;

#[test]
fn test() {
    let input = utils::read_lines("2018/test_day04");
    assert_eq!(day04(input), "part 1: 240 part2: 4455");
}

pub fn day04(lines: Vec<String>) -> String {
    #[derive(Debug)]
    struct SleepCycle {
        id: i32,
        duration: i32,
        start: i32,
        end: i32,
    }
    let lines = parse(lines);
    let mut sleep_cycles = vec![];
    let mut current_guard = None;
    let mut start_sleep = None;
    let mut current_day = None;
    for line in lines {
        match line.state {
            State::BeginShift { id } => {
                current_guard = Some(id);
                assert_eq!(start_sleep, None);
            }
            State::FallAsleep => {
                assert_eq!(start_sleep, None);
                start_sleep = Some(line.ts.minute);
                current_day = Some(line.ts.day);
            }
            State::WakeUp => {
                if current_day.unwrap() != line.ts.day {
                    panic!(
                        "current_day: {:?} line: {:?} guard: {:?}",
                        current_day, line, current_guard
                    );
                }
                sleep_cycles.push(SleepCycle {
                    id: current_guard.unwrap(),
                    duration: line.ts.minute - start_sleep.unwrap(),
                    start: start_sleep.unwrap(),
                    end: line.ts.minute,
                });
                start_sleep = None;
            }
        }
    }
    let mut id_to_duration = HashMap::new();
    for cycle in &sleep_cycles {
        let counter = id_to_duration.entry(cycle.id).or_insert(0);
        *counter += cycle.duration;
    }
    let (max_sleep_id, _duration) = id_to_duration
        .into_iter()
        .max_by(|(_id1, dur1), (_id2, dur2)| dur1.cmp(dur2))
        .unwrap();
    let best_sleeper_sleep_cycles: Vec<&SleepCycle> = sleep_cycles
        .iter()
        .filter(|x| x.id == max_sleep_id)
        .collect();
    let mut minute_to_sleep = HashMap::new();
    for cycle in best_sleeper_sleep_cycles {
        for minute in cycle.start..cycle.end {
            let counter = minute_to_sleep.entry(minute).or_insert(0);
            *counter += 1;
        }
    }
    let (max_sleep_minute, _count) = minute_to_sleep
        .into_iter()
        .max_by(|(_minute1, count1), (_minute2, count2)| count1.cmp(count2))
        .unwrap();
    let mut sleeper_per_minute: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    for cycle in &sleep_cycles {
        for minute in cycle.start..cycle.end {
            let per_id_counter = sleeper_per_minute.entry(minute).or_default();
            let counter = per_id_counter.entry(cycle.id).or_insert(0);
            *counter += 1;
        }
    }
    let mut max_sleeper = None;
    for (minute, sleepers) in sleeper_per_minute {
        for (id, counter) in sleepers {
            max_sleeper = match max_sleeper {
                None => Some((id, minute, counter)),
                Some((id_best, minute_best, counter_best)) => {
                    if counter_best < counter {
                        Some((id, minute, counter))
                    } else {
                        Some((id_best, minute_best, counter_best))
                    }
                }
            }
        }
    }
    let (id_best, minute_best, _counter_best) = max_sleeper.unwrap();
    format!(
        "part 1: {} part2: {}",
        max_sleep_id * max_sleep_minute,
        id_best * minute_best
    )
}

#[test]
fn test_parse() {
    assert_eq!(
        parse(utils::to_vec(&[
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-02 00:25] wakes up",
            "[1518-11-02 00:00] Guard #11 begins shift",
            "[1518-11-01 23:59] falls asleep",
            "[1518-09-27 00:59] Guard #12 begins shift",
        ])),
        vec![
            Input {
                ts: Ts {
                    day: 927,
                    minute: 59
                },
                state: State::BeginShift { id: 12 }
            },
            Input {
                ts: Ts {
                    day: 1101,
                    minute: 0
                },
                state: State::BeginShift { id: 10 }
            },
            Input {
                ts: Ts {
                    day: 1101,
                    minute: 5
                },
                state: State::FallAsleep
            },
            Input {
                ts: Ts {
                    day: 1102,
                    minute: -1
                },
                state: State::FallAsleep
            },
            Input {
                ts: Ts {
                    day: 1102,
                    minute: 0
                },
                state: State::BeginShift { id: 11 }
            },
            Input {
                ts: Ts {
                    day: 1102,
                    minute: 25
                },
                state: State::WakeUp
            },
        ]
    );
}

#[derive(PartialEq, Eq, Debug, Hash)]
#[allow(non_camel_case_types)]
struct Input {
    state: State,
    ts: Ts,
}
type GuardId = i32;
#[derive(PartialEq, Eq, Debug, Hash)]
#[allow(non_camel_case_types)]
enum State {
    BeginShift { id: GuardId },
    FallAsleep,
    WakeUp,
}

#[derive(PartialEq, Eq, Debug, Hash, PartialOrd, Ord)]
struct Ts {
    day: i32,
    minute: i32,
}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (ts, rest) = s
            .split_once(']')
            .ok_or(format!("couldn't parse line {}", s))?;
        let (_, ts) = ts
            .split_once("1518-")
            .ok_or(format!("couldn't parse line {}", s))?;
        let (day, minute) = ts
            .split_once(' ')
            .ok_or(format!("couldn't parse line {}", s))?;
        let day: i32 = day
            .chars()
            .filter(|x| *x != '-')
            .collect::<String>()
            .parse()?;
        let minute: i32 = minute
            .chars()
            .filter(|x| *x != ':')
            .collect::<String>()
            .parse()?;
        let (day, minute) = if minute > 60 {
            (day + 1, -(2360 - minute))
        } else {
            (day, minute)
        };
        let ts = Ts { day, minute };
        let c = rest
            .chars()
            .nth(1)
            .ok_or(format!("couldn't parse line {}", s))?;
        match c {
            'f' => Ok(Input {
                ts,
                state: State::FallAsleep,
            }),
            'G' => {
                let (id, _) = rest
                    .split_once(" begins")
                    .ok_or(format!("couldn't parse line {}", s))?;
                let id: i32 = id.chars().skip(8).collect::<String>().parse()?;
                Ok(Input {
                    ts,
                    state: State::BeginShift { id },
                })
            }
            'w' => Ok(Input {
                ts,
                state: State::WakeUp,
            }),
            _ => unreachable!(),
        }
    }
}

fn parse(lines: Vec<String>) -> Vec<Input> {
    let mut lines: Vec<_> = lines
        .iter()
        .map(|line| line.parse::<Input>().unwrap())
        .collect();
    lines.sort_by(|p1, p2| p1.ts.cmp(&p2.ts));
    lines
}
