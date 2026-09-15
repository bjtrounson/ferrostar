use crate::{
    models::{Route, UserLocation},
    navigation_controller::models::{NavState, NavigationControllerConfig},
    navigation_session::{
        NavigationObserver,
        recording::models::{
            NavigationRecordingChunk, NavigationRecordingChunkOptions, NavigationRecordingEvent,
            NavigationRecordingMetadata, RecordingError,
        },
    },
};
use std::sync::Mutex;

pub mod models;
pub mod replay;

#[cfg_attr(feature = "uniffi", derive(uniffi::Object))]
pub struct NavigationRecorder {
    pub recording: NavigationRecordingMetadata,
    events: Mutex<Vec<NavigationRecordingEvent>>,
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl NavigationRecorder {
    #[cfg_attr(feature = "uniffi", uniffi::constructor)]
    pub fn new(route: Route, config: NavigationControllerConfig) -> Self {
        let recording = NavigationRecordingMetadata::new(config, route);
        Self {
            recording,
            events: Mutex::new(Vec::new()),
        }
    }
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl NavigationRecorder {
    pub fn get_events(&self) -> Vec<NavigationRecordingEvent> {
        self.events.lock().unwrap().clone()
    }

    pub fn get_recording_json(&self) -> Result<String, RecordingError> {
        let events = self.get_events();
        self.recording.to_json(events)
    }

    /// Exports a bounded, non-destructive section of the recording.
    ///
    /// The cursor is an event index. Each returned string is a complete recording JSON document
    /// with the original metadata and a contiguous subset of events.
    pub fn get_recording_chunk(
        &self,
        options: NavigationRecordingChunkOptions,
    ) -> Result<NavigationRecordingChunk, RecordingError> {
        let metadata_json = serde_json::to_string(&self.recording).map_err(|error| {
            RecordingError::SerializationError {
                error: error.to_string(),
            }
        })?;
        let metadata_prefix =
            metadata_json
                .strip_suffix('}')
                .ok_or_else(|| RecordingError::SerializationError {
                    error: "recording metadata did not serialize as a JSON object".to_string(),
                })?;
        let mut json = String::with_capacity(metadata_json.len() + 12);
        json.push_str(metadata_prefix);
        json.push_str(",\"events\":[");

        let suffix = "]}";
        let minimum_bytes = byte_len(&json).saturating_add(byte_len(suffix));
        if minimum_bytes > u64::from(options.max_bytes) {
            return Err(RecordingError::ChunkSizeTooSmall {
                max_bytes: options.max_bytes,
                minimum_bytes,
            });
        }

        let events = self
            .events
            .lock()
            .map_err(|_| RecordingError::RecordingUnavailable)?;
        let event_count =
            u32::try_from(events.len()).map_err(|error| RecordingError::SerializationError {
                error: error.to_string(),
            })?;
        if options.cursor > event_count {
            return Err(RecordingError::InvalidChunkCursor {
                cursor: options.cursor,
                event_count,
            });
        }

        let start =
            usize::try_from(options.cursor).map_err(|_| RecordingError::InvalidChunkCursor {
                cursor: options.cursor,
                event_count,
            })?;
        let mut next_cursor = options.cursor;

        for event in &events[start..] {
            let event_json = serde_json::to_string(event).map_err(|error| {
                RecordingError::SerializationError {
                    error: error.to_string(),
                }
            })?;
            let separator_bytes = u64::from(next_cursor > options.cursor);
            let required_bytes = byte_len(&json)
                .saturating_add(separator_bytes)
                .saturating_add(byte_len(&event_json))
                .saturating_add(byte_len(suffix));

            if required_bytes > u64::from(options.max_bytes) {
                if next_cursor == options.cursor {
                    return Err(RecordingError::EventTooLarge {
                        cursor: next_cursor,
                        event_bytes: byte_len(&event_json),
                        required_bytes,
                        max_bytes: options.max_bytes,
                    });
                }
                break;
            }

            if separator_bytes > 0 {
                json.push(',');
            }
            json.push_str(&event_json);
            next_cursor = next_cursor.saturating_add(1);
        }

        json.push_str(suffix);
        Ok(NavigationRecordingChunk {
            json,
            next_cursor,
            done: next_cursor == event_count,
        })
    }
}

fn byte_len(value: &str) -> u64 {
    u64::try_from(value.len()).unwrap_or(u64::MAX)
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
impl NavigationObserver for NavigationRecorder {
    fn on_get_initial_state(&self, state: NavState) {
        let event = NavigationRecordingEvent::state_update(state.into());
        if let Ok(mut events) = self.events.lock() {
            events.push(event);
        }
    }

    fn on_user_location_update(
        &self,
        // The users location is captured in the NavState
        #[allow(unused_variables)] location: UserLocation,
        state: NavState,
    ) {
        let event = NavigationRecordingEvent::state_update(state.into());
        if let Ok(mut events) = self.events.lock() {
            events.push(event);
        }
    }

    fn on_advance_to_next_step(&self, state: NavState) {
        let event = NavigationRecordingEvent::state_update(state.into());
        if let Ok(mut events) = self.events.lock() {
            events.push(event);
        }
    }

    fn on_route_available(&self, #[allow(unused_variables)] route: Route) {
        // TODO: We could capture the route on the recording if desired.
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::routing_adapters::osrm::models::OsrmWaypointProperties;
    use crate::test_utils::{TestRoute, redact_properties};
    use crate::{
        navigation_controller::{
            NavigationController,
            test_helpers::{
                get_test_navigation_controller_config, get_test_step_advance_condition,
                nav_controller_insta_settings,
            },
        },
        navigation_session::recording::models::{
            NavigationRecording, NavigationRecordingChunkOptions, NavigationRecordingEvent,
            NavigationRecordingEventData, RecordingError,
        },
        navigation_session::{
            NavigationSession, recording::NavigationRecorder,
            test_helpers::test_full_route_state_snapshot,
        },
    };

    #[test]
    fn test_recording_serialization() {
        nav_controller_insta_settings().bind(|| {
            let route = TestRoute::ValhallaSelfIntersecting.first_route();
            let config = get_test_navigation_controller_config(get_test_step_advance_condition(0));
            let recorder = Arc::new(NavigationRecorder::new(route.clone(), config.clone()));
            let session = NavigationSession::new(
                Arc::new(NavigationController::new(route.clone(), config)),
                vec![recorder.clone()],
            );
            let _ = test_full_route_state_snapshot(route, session);

            let json = recorder.get_recording_json().unwrap();
            let value: serde_json::Value = serde_json::from_str(&json).unwrap();
            insta::assert_yaml_snapshot!(value, {
                ".**.utteranceId" => "[uuid]",
                ".**.remainingWaypoints[].properties" => insta::dynamic_redaction(redact_properties::<OsrmWaypointProperties>),
                ".**.remaining_waypoints[].properties" => insta::dynamic_redaction(redact_properties::<OsrmWaypointProperties>),
                ".**.waypoints[].properties" => insta::dynamic_redaction(redact_properties::<OsrmWaypointProperties>),
            });
        });
    }

    fn recorder_with_route_events(event_count: usize) -> NavigationRecorder {
        let route = TestRoute::ValhallaSelfIntersecting.first_route();
        let config = get_test_navigation_controller_config(get_test_step_advance_condition(0));
        let recorder = NavigationRecorder::new(route.clone(), config);
        let events = (0..event_count).map(|_| {
            NavigationRecordingEvent::new(NavigationRecordingEventData::RouteUpdate {
                route: route.clone(),
            })
        });
        recorder.events.lock().unwrap().extend(events);
        recorder
    }

    #[test]
    fn recording_chunk_export_reassembles_the_full_recording() {
        let recorder = recorder_with_route_events(3);
        let all_events = recorder.get_events();
        let max_bytes = u32::try_from(
            recorder
                .recording
                .to_json(vec![all_events[0].clone()])
                .unwrap()
                .len(),
        )
        .unwrap();
        let full_json = recorder.get_recording_json().unwrap();
        let mut full_value: serde_json::Value = serde_json::from_str(&full_json).unwrap();

        let mut cursor = 0;
        let mut assembled_events = Vec::new();
        let mut chunk_count = 0;
        loop {
            let chunk = recorder
                .get_recording_chunk(NavigationRecordingChunkOptions { cursor, max_bytes })
                .unwrap();
            assert!(u32::try_from(chunk.json.len()).unwrap() <= max_bytes);
            assert!(chunk.next_cursor > cursor);

            let mut value: serde_json::Value = serde_json::from_str(&chunk.json).unwrap();
            assembled_events.extend(
                value
                    .get_mut("events")
                    .unwrap()
                    .as_array_mut()
                    .unwrap()
                    .drain(..),
            );
            cursor = chunk.next_cursor;
            chunk_count += 1;
            if chunk.done {
                break;
            }
        }

        assert!(chunk_count > 1);
        *full_value.get_mut("events").unwrap() = assembled_events.into();
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&full_json).unwrap(),
            full_value
        );
    }

    #[test]
    fn recording_chunk_export_can_retry_the_same_cursor() {
        let recorder = recorder_with_route_events(2);
        let options = NavigationRecordingChunkOptions {
            cursor: 0,
            max_bytes: u32::MAX,
        };

        let first = recorder.get_recording_chunk(options.clone()).unwrap();
        let retry = recorder.get_recording_chunk(options).unwrap();

        assert_eq!(retry, first);
        assert_eq!(first.json, recorder.get_recording_json().unwrap());
        assert_eq!(recorder.get_events().len(), 2);
    }

    #[test]
    fn recording_chunk_export_rejects_an_event_over_the_byte_limit() {
        let recorder = recorder_with_route_events(1);
        let event = recorder.get_events().remove(0);
        let one_event_bytes = recorder.recording.to_json(vec![event]).unwrap().len();
        let max_bytes = u32::try_from(one_event_bytes - 1).unwrap();

        let error = recorder
            .get_recording_chunk(NavigationRecordingChunkOptions {
                cursor: 0,
                max_bytes,
            })
            .unwrap_err();

        assert!(matches!(
            error,
            RecordingError::EventTooLarge {
                cursor: 0,
                max_bytes: error_max_bytes,
                ..
            } if error_max_bytes == max_bytes
        ));
    }

    #[test]
    fn recording_chunk_export_rejects_a_limit_smaller_than_the_json_envelope() {
        let recorder = recorder_with_route_events(0);
        let minimum_bytes = u64::try_from(recorder.get_recording_json().unwrap().len()).unwrap();
        let max_bytes = u32::try_from(minimum_bytes - 1).unwrap();

        let error = recorder
            .get_recording_chunk(NavigationRecordingChunkOptions {
                cursor: 0,
                max_bytes,
            })
            .unwrap_err();

        assert!(matches!(
            error,
            RecordingError::ChunkSizeTooSmall {
                max_bytes: error_max_bytes,
                minimum_bytes: error_minimum_bytes,
            } if error_max_bytes == max_bytes && error_minimum_bytes == minimum_bytes
        ));
    }

    #[test]
    fn recording_chunk_export_rejects_a_cursor_beyond_the_event_stream() {
        let recorder = recorder_with_route_events(1);

        let error = recorder
            .get_recording_chunk(NavigationRecordingChunkOptions {
                cursor: 2,
                max_bytes: u32::MAX,
            })
            .unwrap_err();

        assert!(matches!(
            error,
            RecordingError::InvalidChunkCursor {
                cursor: 2,
                event_count: 1,
            }
        ));
    }

    #[test]
    fn recording_deserializes_legacy_and_canonical_formats() {
        let legacy = include_str!("../../fixtures/recording_legacy_native.json");
        let canonical = include_str!("../../fixtures/recording_canonical.json");

        let legacy_recording = NavigationRecording::try_from_json(legacy).unwrap();
        let canonical_recording = NavigationRecording::try_from_json(canonical).unwrap();
        let expected: serde_json::Value = serde_json::from_str(canonical).unwrap();

        assert_eq!(serde_json::to_value(legacy_recording).unwrap(), expected);
        assert_eq!(serde_json::to_value(canonical_recording).unwrap(), expected);
    }

    #[test]
    fn recording_deserialization_preserves_serde_error_context() {
        let error = NavigationRecording::try_from_json("{}").err().unwrap();

        if !matches!(error, RecordingError::DeserializationError { .. }) {
            panic!("expected a deserialization error");
        };
    }

    #[test]
    fn recording_deserialization_rejects_duplicate_aliases() {
        let canonical = include_str!("../../fixtures/recording_canonical.json");
        let mut value: serde_json::Value = serde_json::from_str(canonical).unwrap();
        value
            .pointer_mut("/events/0/event_data/StateUpdate/trip_state/Navigating/progress")
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("distance_to_next_maneuver".into(), 1.0.into());

        let error = NavigationRecording::try_from_json(&value.to_string())
            .err()
            .unwrap();
        assert!(error.to_string().contains("duplicate field"));
    }

    #[test]
    fn recording_deserialization_rejects_invalid_legacy_timestamp() {
        let legacy = include_str!("../../fixtures/recording_legacy_native.json");
        let mut value: serde_json::Value = serde_json::from_str(legacy).unwrap();
        *value
            .pointer_mut(
                "/events/0/event_data/StateUpdate/trip_state/Navigating/user_location/\
                 timestamp/nanos_since_epoch",
            )
            .unwrap() = 1_000_000_000_u64.into();

        let error = NavigationRecording::try_from_json(&value.to_string())
            .err()
            .unwrap();
        assert!(
            error
                .to_string()
                .contains("nanos_since_epoch must be less than 1000000000")
        );
    }

    #[test]
    fn recording_deserialization_rejects_overflowing_legacy_timestamp() {
        let legacy = include_str!("../../fixtures/recording_legacy_native.json");
        let mut value: serde_json::Value = serde_json::from_str(legacy).unwrap();
        *value
            .pointer_mut(
                "/events/0/event_data/StateUpdate/trip_state/Navigating/user_location/\
                 timestamp/secs_since_epoch",
            )
            .unwrap() = u64::MAX.into();

        let error = NavigationRecording::try_from_json(&value.to_string())
            .err()
            .unwrap();
        assert!(
            error
                .to_string()
                .contains("system time exceeds epoch milliseconds")
        );
    }
}
