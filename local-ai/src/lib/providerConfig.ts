export const IS_DIRECT_ENGINE_PROVIDER =
  typeof navigator !== 'undefined' && /windows/i.test(navigator.userAgent);

// Compatibility alias while the Windows parity port still shares some Mac-origin file names.
export const IS_MAC_MODEL_PROVIDER = IS_DIRECT_ENGINE_PROVIDER;

export const APP_DISPLAY_NAME = 'ModernClawMulti';
export const MODEL_PROVIDER_NAME = IS_DIRECT_ENGINE_PROVIDER ? 'Direct Engine' : 'Ollama';
export const MODEL_PROVIDER_STATUS_URL = IS_DIRECT_ENGINE_PROVIDER
  ? 'http://127.0.0.1:8080/v1/models'
  : 'http://localhost:11434/api/tags';
export const MODEL_PROVIDER_APP_PATH = '';
export const MODEL_PROVIDER_DOWNLOAD_URL = IS_DIRECT_ENGINE_PROVIDER
  ? 'https://github.com/ggml-org/llama.cpp'
  : 'https://ollama.com/download';

function normalizeModelKey(name: string | null | undefined) {
  return name?.trim().toLowerCase().replace(/[^a-z0-9]/g, '') ?? '';
}

export function isRecommendedModelName(name: string | null | undefined) {
  const normalized = normalizeModelKey(name);
  return Boolean(normalized && normalized.includes('gemma4'));
}

export function resolvePreferredModelName(
  preferred: string | null | undefined,
  modelNames: string[]
) {
  const normalizedNames = modelNames.filter(Boolean);
  if (normalizedNames.length === 0) {
    return preferred ?? null;
  }

  if (preferred && normalizedNames.includes(preferred)) {
    return preferred;
  }

  const preferredKey = normalizeModelKey(preferred);
  if (preferredKey) {
    const canonicalMatch = normalizedNames.find((name) => normalizeModelKey(name) === preferredKey);
    if (canonicalMatch) {
      return canonicalMatch;
    }

    const fuzzyMatch = normalizedNames.find((name) => normalizeModelKey(name).includes(preferredKey));
    if (fuzzyMatch) {
      return fuzzyMatch;
    }
  }

  const gemmaMatch = normalizedNames.find((name) => isRecommendedModelName(name));
  if (gemmaMatch) {
    return gemmaMatch;
  }

  return normalizedNames[0] ?? null;
}
