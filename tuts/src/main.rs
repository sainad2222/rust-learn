use std::{
    thread,
    time::{Duration, Instant},
};

#[derive(Debug)]
enum State {
    Closed,
    Open,
    HalfOpen,
}

struct CircuitBreaker {
    state: State,
    trip_timout: Duration,
    max_failures_threshold: usize,
    consecutive_failures: usize,
    opened_at: Option<Instant>,
}

impl CircuitBreaker {
    pub fn new(max_failures_threshold: usize, trip_timout: Duration) -> CircuitBreaker {
        CircuitBreaker {
            state: State::Closed,
            trip_timout,
            max_failures_threshold,
            consecutive_failures: 0,
            opened_at: None,
        }
    }

    pub fn call<F, T, E>(&mut self, f: F) -> Option<Result<T, E>>
    where
        F: FnOnce() -> Result<T, E>,
    {
        match self.state {
            State::Closed => {
                if self.consecutive_failures < self.max_failures_threshold {
                    let result = f();
                    if let Err(_) = result {
                        self.record_failure();
                    }
                    Some(result)
                } else {
                    self.opened_at = Some(Instant::now());
                    self.state = State::Open;
                    self.consecutive_failures = 0;
                    None
                }
            }
            State::Open => {
                if let Some(opened_at) = self.opened_at {
                    let elapsed = Instant::now().duration_since(opened_at);
                    if elapsed >= self.trip_timout {
                        self.state = State::HalfOpen;
                        self.opened_at = None;
                    }
                }
                None
            }
            State::HalfOpen => {
                let result = f();
                if let Err(_) = result {
                    self.state = State::Open;
                } else {
                    self.state = State::Closed;
                }
                Some(result)
            }
        }
    }
    fn record_failure(&mut self) {
        match self.state {
            State::Open => (),
            State::Closed => self.consecutive_failures += 1,
            State::HalfOpen => self.consecutive_failures -= 1,
        }
    }
}

fn request(dice: u32) -> Result<u32, String> {
    if dice > 6 {
        Err("400: Bad request.".to_string())
    } else {
        Ok(dice)
    }
}

fn main() {
    let mut cb = CircuitBreaker::new(3, Duration::from_secs(10));
    println!("Circuit Breaker has been set with");
    println!("    * 3 as maximum consecutive failures");
    println!("    * 10 seconds as the trip timeout");
    println!("");

    println!("Circuit Breaker is in the initial state, which is closed.");
    let result = cb.call(|| request(5));
    println!("Result for request_dice(5): {:?}", result);
    println!("Circuit Breaker is encounting 3 errors in a row ...");

    // The function returns an error 3 times in a row, so the circuit
    // breaker transitions to the open state
    cb.call(|| request(10));
    cb.call(|| request(10));
    cb.call(|| request(10));

    // The circuit breaker is in the open state, so the function is
    // not executed
    let result = cb.call(|| request(2));
    println!("Result for request_dice(2): {:?}", result);

    println!("Let's have fun by doing nothing in 15 seconds :(");
    println!("...");
    thread::sleep(Duration::from_secs(15));
    let result = cb.call(|| request(5));
    println!("Result for request_dice(5): {:?}", result);
    let result = cb.call(|| request(6));
    println!("Result for request_dice(6): {:?}", result);
}
