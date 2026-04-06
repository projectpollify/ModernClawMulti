import { DEFAULT_PIPER_VOICE_ID, DEFAULT_WHISPER_MODEL_FILENAME, getCuratedVoiceById } from '@/lib/voiceCatalog';

export interface VoicePathDefaults {
  voiceRoot: string;
  piperFolder: string;
  piperVoicesFolder: string;
  whisperFolder: string;
  whisperModelsFolder: string;
  piperExecutablePath: string;
  piperModelPath: string;
  whisperExecutablePath: string;
  whisperModelPath: string;
}

export function getDefaultVoicePaths(memoryPath: string, voicePresetId: string = DEFAULT_PIPER_VOICE_ID): VoicePathDefaults {
  const normalizedBase = memoryPath.replace(/[\\/]+$/, '');
  const voiceRoot = `${normalizedBase}\\tools`;
  const piperFolder = `${voiceRoot}\\piper`;
  const piperVoicesFolder = `${piperFolder}\\voices`;
  const whisperFolder = `${voiceRoot}\\whisper`;
  const whisperModelsFolder = `${whisperFolder}\\models`;
  const curatedVoice = getCuratedVoiceById(voicePresetId);

  return {
    voiceRoot,
    piperFolder,
    piperVoicesFolder,
    whisperFolder,
    whisperModelsFolder,
    piperExecutablePath: `${piperFolder}\\piper.exe`,
    piperModelPath: `${piperVoicesFolder}\\${curatedVoice.filename}`,
    whisperExecutablePath: `${whisperFolder}\\whisper-cli.exe`,
    whisperModelPath: `${whisperModelsFolder}\\${DEFAULT_WHISPER_MODEL_FILENAME}`,
  };
}