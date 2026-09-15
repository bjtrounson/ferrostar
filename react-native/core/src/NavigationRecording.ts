import type {
  NavigationControllerConfig,
  NavigationRecordingChunk,
  NavigationRecordingChunkOptions,
  NavigationRecordingEvent,
} from '@stadiamaps/ferrostar-uniffi-react-native';

export type {
  NavigationRecordingChunk,
  NavigationRecordingChunkOptions,
} from '@stadiamaps/ferrostar-uniffi-react-native';

/**
 * A read-only handle to a navigation session recording.
 *
 * Ferrostar keeps recording in memory. The application decides if and where
 * the resulting JSON should be stored, uploaded, or shared.
 */
export interface NavigationRecording {
  getEvents(): ReadonlyArray<NavigationRecordingEvent>;
  /**
   * Exports a complete recording JSON document bounded by `maxBytes`.
   * Pass each `nextCursor` back in until `done` is true.
   */
  getRecordingChunk(
    options: NavigationRecordingChunkOptions
  ): NavigationRecordingChunk;
  getRecordingJson(): string;
}

export type RecordedNavigationOptions = {
  recording: true;
  config?: NavigationControllerConfig;
};
