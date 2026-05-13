use std::{fmt, time::Duration};

#[derive(Clone, Debug)]
pub struct AlgoResult {
    pub distance: Option<f32>,
    pub visited_nodes: Option<u32>,
    pub error: Option<String>,
    pub duration: Option<Duration>
}

impl AlgoResult{
    pub fn ok(distance: f32, visited_nodes: u32, duration: Duration) -> Self{
        Self{
           distance: Some(distance),
           visited_nodes: Some(visited_nodes),
           duration: Some(duration),
           error: None
        }
    }
    pub fn err(error: &str)->Self{
        Self{
            distance: None,
            visited_nodes: None,
            duration: None,
            error: Some(error.to_string())
        }
    }
}

impl fmt::Display for AlgoResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(err) = &self.error {
            return write!(f, "error: {}", err);
        }

        match (
            self.distance,
            self.visited_nodes,
            self.duration,
        ) {
            (Some(d), Some(v), Some(t)) => {
                let time_str = if t.as_millis() > 0 {
                    format!("{} ms", t.as_millis())
                } else {
                    format!("{} μs", t.as_micros())
                };

                write!(
                    f,
                    "distance: {:.2}, visited nodes: {}, time: {}",
                    d,
                    v,
                    time_str
                )
            }

            _ => write!(f, "No path found"),
        }
    }
}
