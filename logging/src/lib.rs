// 1. Bring the macros `error, warn, info, debug, trace` into scope from the log package with a
// `use` statement.
//
// You should be able to run `cargo build --lib` successfully after this step (and each step in this
// file)
//
// Hint: You need to update Cargo.toml to add the `log` dependency, first.

use log::{debug, error, info, trace, warn};

#[derive(Debug)]
pub struct Frog {
    energy: u8,
    sleeping: bool,
}

impl Frog {
    pub fn new() -> Self {
        // 2. Use debug!() to log "A new Frog has been created"
        debug!(target: "Frog::new","A new Frog has been created");
        Default::default()
    }
    pub fn hop(&mut self) {
        self.energy -= 1;
        // 3. Use info!() to log that a Frog hopped, and how much energy is left
        info!(target: "Frog::hop", "Frog hopped. Energy left: {}", self.energy);
        if self.energy == 0 {
            // 4. Use warn!() to warn that the frog will go to sleep since he ran out of energy
            warn!(target: "Frog::hop", "Frog is out of energy and will sleep");
            self.sleep();
        }
    }
    pub fn sleep(&mut self) {
        if self.sleeping {
            // 5. Use error!() to log a (non-fatal) error stating that the Frog is already asleep
            error!(target: "Frog::sleep", "Frog is already asleep");
        } else {
            self.sleeping = true;
        }
    }
}

impl Default for Frog {
    fn default() -> Self {
        // 6. Use trace!() to log that a default value was generated, with the debug representation
        let frog = Frog { energy: 5, sleeping: false };
        trace!(target: "Frog::default", "Generated default Frog: {:?}", frog);
        frog
    }
}
