import { DEFAULT_FLOOR_MODEL, DEFAULT_PIPER_VOICE_ID } from '@/lib/voiceCatalog';
import type { Theme } from '@/types';

export interface AppSettings {
  theme: Theme;
  defaultModel: string | null;
  contextWindowSize: number;
  memoryPath: string;
  autoSaveConversations: boolean;
  streamResponses: boolean;
  sendOnEnter: boolean;
  showTokenCount: boolean;
  saveConversationHistory: boolean;
  enableVoiceOutput: boolean;
  piperVoicePreset: string;
  piperExecutablePath: string;
  piperModelPath: string;
  enableVoiceInput: boolean;
  whisperExecutablePath: string;
  whisperModelPath: string;
  whisperLanguage: string;
}

export const DEFAULT_SETTINGS: AppSettings = {
  theme: 'system',
  defaultModel: DEFAULT_FLOOR_MODEL,
  contextWindowSize: 4096,
  memoryPath: '',
  autoSaveConversations: true,
  streamResponses: true,
  sendOnEnter: true,
  showTokenCount: false,
  saveConversationHistory: true,
  enableVoiceOutput: false,
  piperVoicePreset: DEFAULT_PIPER_VOICE_ID,
  piperExecutablePath: '',
  piperModelPath: '',
  enableVoiceInput: false,
  whisperExecutablePath: '',
  whisperModelPath: '',
  whisperLanguage: 'auto',
};