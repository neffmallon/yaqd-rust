use crate::is_daemon::IsDaemon;

struct HasPositionState{
    destination: f64,
    position: f64,
}

// It is tempting to have public methods return bitstrings and have another set of private methods
// return the Rust-typed values for internal use.
pub trait HasPosition: IsDaemon{
    fn get_destination(&self) -> f64;

    fn get_units(&self) -> Option(String);

    fn get_position(&self) -> f64;

    fn set_position(&self, position: f64);

    fn set_relative(&self, distance: f64) -> f64;
}

struct HasLimitsState {
    hw_limits: (f64, f64)
}

pub trait HasLimits: HasPosition{
    fn in_limits(&self) -> bool{
        let (lower, upper) = self.get_limits();
        let p = self.get_position();
        return p >= lower && p <= upper
    }

    fn get_limits(&self)-> (f64, f64);
}