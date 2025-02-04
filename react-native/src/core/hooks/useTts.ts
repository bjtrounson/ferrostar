import { useEffect, useState } from 'react';
import Tts, { type Voice } from 'react-native-tts';

type Status =
  | 'preparing'
  | 'initialized'
  | 'started'
  | 'finished'
  | 'cancelled';

type SpeakOptions = {
  voiceId?: string;
  rate?: number;
  pitch?: number;
};

type Progress = {
  utteranceId: string | number;
  location: number;
  length: number;
};

export type Tts = {
  status: Status;
  voices: Voice[];
  selectedVoice: Voice | null;
  error: Error | null;
  progress: Progress | null;
  speak: (
    text: string,
    options?: SpeakOptions
  ) => Promise<string | number | undefined>;
  changeVoice: (voiceId: string) => Promise<void>;
  toggleMute: () => Promise<void>;
  stop: () => Promise<void>;
};

type TtsOptions = {
  speechRate?: number;
  speechPitch?: number;
};

export default function useTts({
  speechPitch,
  speechRate,
}: TtsOptions = {}): Tts {
  const [voices, setVoices] = useState<Voice[]>([]);
  const [status, setStatus] = useState<Status>('preparing');
  const [isMuted, setIsMuted] = useState<boolean>(false);
  const [selectedVoice, setSelectedVoice] = useState<Voice | null>(null);
  const [progress, setProgress] = useState<Progress | null>(null);
  const [error, setError] = useState<Error | null>(null);

  useEffect(() => {
    Tts.addEventListener('tts-start', () => {
      setStatus('started');
    });
    Tts.addEventListener('tts-progress', (e) => {
      setProgress(e);
    });
    Tts.addEventListener('tts-finish', () => {
      setStatus('finished');
      setProgress(null);
    });
    Tts.addEventListener('tts-cancel', () => {
      setStatus('cancelled');
      setProgress(null);
    });

    init();

    return () => {
      Tts.removeAllListeners('tts-start');
      Tts.removeAllListeners('tts-finish');
      Tts.removeAllListeners('tts-cancel');
    };
  }, []);

  const init = async () => {
    const status = await Tts.getInitStatus();
    if (status != 'success') {
      return;
    }

    const voices = await Tts.voices();
    const availableVoices = voices.filter(
      (voice) => !voice.networkConnectionRequired || !voice.notInstalled
    );
    const selectedVoice = availableVoices[0];
    if (voices.length <= 0 || selectedVoice === undefined) {
      return;
    }

    try {
      await Tts.setDefaultLanguage(selectedVoice.language);
    } catch (e) {
      if (e instanceof Error) {
        setError(e);
      }

      console.warn(`setDefaultLanguage failed: ${e}`);
    }

    await Tts.setDefaultVoice(selectedVoice.id);

    if (speechPitch !== undefined) {
      await Tts.setDefaultPitch(speechPitch);
    }

    if (speechRate !== undefined) {
      await Tts.setDefaultRate(speechRate);
    }

    setVoices(availableVoices);
    setSelectedVoice(selectedVoice);
    setStatus('initialized');
  };

  const toggleMute = async () => {
    if (status !== 'initialized') {
      return;
    }

    setIsMuted(!isMuted);
  };

  const changeVoice = async (voiceId: string) => {
    if (status !== 'initialized') {
      return;
    }

    const newVoice = voices.find((voice) => voice.id === voiceId);
    if (newVoice === undefined) {
      setError(new Error(`Voice ${voiceId} not found`));
      return;
    }

    try {
      await Tts.setDefaultVoice(voiceId);
      setSelectedVoice(newVoice);
    } catch (e) {
      if (e instanceof Error) {
        setError(e);
      }

      console.warn(`setDefaultVoice failed: ${e}`);
    }
  };

  const speak = async (text: string): Promise<string | number | undefined> => {
    if (status !== 'initialized') {
      return;
    }

    if (selectedVoice === null) {
      setError(new Error('No voice selected'));
      return;
    }

    let utteranceId: string | number | undefined = undefined;
    try {
      utteranceId = Tts.speak(text);
      setStatus('started');
    } catch (e) {
      if (e instanceof Error) {
        setError(e);
      }

      console.warn(`speak failed: ${e}`);
    }

    return utteranceId;
  };

  const stopAndClearQueue = async () => {
    if (status !== 'started') {
      return;
    }

    try {
      await Tts.stop();
      setStatus('finished');
      setProgress(null);
    } catch (e) {
      if (e instanceof Error) {
        setError(e);
      }

      console.warn(`stop failed: ${e}`);
    }
  };

  return {
    status,
    voices,
    selectedVoice,
    error,
    progress,
    speak,
    changeVoice,
    toggleMute,
    stop: stopAndClearQueue,
  };
}
