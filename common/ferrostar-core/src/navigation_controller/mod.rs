pub mod models;

use crate::models::{
    Route, UserLocation,
};
use crate::utils::snap_to_line;
use geo::{Coord};
use std::sync::Mutex;
use models::*;

/// Manages the navigation lifecycle of a single trip, requesting the initial route and updating
/// internal state based on inputs like user location updates.
///
/// By "navigation lifecycle of a single trip", we mean that this controller comes into being when
/// one (or more) routes have already been calculated and a route has been selected from the
/// alternatives (if applicable). It ends when the user either 1) expresses their intent to cancel
/// the navigation, or 2) they successfully visit all waypoints. This puts a clear bound on the
/// life of this controller, though higher level constructs may live longer.
///
/// In the grand scheme of the architecture, this is a mid-level construct. It wraps some lower
/// level constructs like the route adapter, but a higher level wrapper handles things
/// like feeding in user location updates, route recalculation behavior, etc.
///
/// This is intentionally impossible to construct without a user location, so tasks like
/// waiting for a fix (or determining if a cached fix is good enough), are left to higher levels.
pub struct NavigationController {
    /// The last known location of the user. For all intents and purposes, the "user" is assumed
    /// to be at the location reported by their device (phone, car, etc.)
    ///
    /// NOTE: [Mutex] is used because UniFFI doesn't handle mutating struct operations
    /// very well. Others like [core::cell::RefCell] are not enough as the entire object is required to be both
    /// [Send] and [Sync], and [core::cell::RefCell] is explicitly `!Sync`.
    state: Mutex<TripState>,
}

impl NavigationController {
    /// Creates a new trip navigation controller given the user's last known location and a route.
    pub fn new(last_user_location: UserLocation, route: Route) -> Self {
        let remaining_waypoints = route.waypoints.clone();
        let remaining_steps = route.steps.clone();
        let route_line_string = route
            .geometry
            .iter()
            .map(|c| Coord { x: c.lng, y: c.lat })
            .collect();

        Self {
            state: Mutex::new(TripState::Navigating {
                last_user_location,
                snapped_user_location: snap_to_line(last_user_location, &route_line_string),
                route,
                route_line_string,
                remaining_waypoints,
                remaining_steps,
            }),
        }
    }

    pub fn update_user_location(&self, location: UserLocation) -> NavigationStateUpdate {
        match self.state.lock() {
            Ok(mut guard) => {
                match *guard {
                    // TODO: Determine current step + mode of travel
                    TripState::Navigating {
                        mut last_user_location,
                        mut snapped_user_location,
                        ref route,
                        ref route_line_string,
                        ref remaining_waypoints,
                        ref remaining_steps,
                        ..
                    } => {
                        last_user_location = location;

                        let Some(current_step) = remaining_steps.first() else {
                            return NavigationStateUpdate::Arrived {
                                spoken_instruction: None,
                                visual_instructions: None,
                            };
                        };

                        //
                        // Core navigation logic
                        //

                        // Find the nearest point on the route line
                        snapped_user_location = snap_to_line(location, &route_line_string);

                        // TODO: Check if the user's distance is > some configurable threshold, accounting for GPS error, mode of travel, etc.
                        // TODO: If so, flag that the user is off route so higher levels can recalculate if desired

                        // TODO: If on track, update the set of remaining waypoints, remaining steps (drop from the list), and update current step.
                        // IIUC these should always appear within the route itself, which simplifies the logic a bit.
                        // TBD: Do we want to support disjoint routes?

                        // TODO: Calculate distance to the next step
                        // TODO: Advance to the next step when close enough

                        if remaining_waypoints.is_empty() {
                            *guard = TripState::Complete;

                            // TODO: Better info
                            NavigationStateUpdate::Arrived {
                                spoken_instruction: None,
                                visual_instructions: None,
                            }
                        } else {
                            NavigationStateUpdate::Navigating {
                                snapped_user_location,
                                remaining_waypoints: remaining_waypoints.clone(),
                                current_step: current_step.clone(),
                                spoken_instruction: None,
                                visual_instructions: None,
                            }
                        }
                    }
                    // It's tempting to throw an error here, since the caller should know better, but
                    // a mistake like this is technically harmless.
                    TripState::Complete => NavigationStateUpdate::Arrived {
                        spoken_instruction: None,
                        visual_instructions: None,
                    },
                }
            }
            Err(_) => {
                // The only way the mutex can become poisoned is if another caller panicked while
                // holding the mutex. In which case, there is no point in continuing.
                unreachable!("Poisoned mutex. This should never happen.");
            }
        }
    }
}
