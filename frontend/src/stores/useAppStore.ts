import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { AIModel, ProgrammingLanguage, UserConfig, GenerateResponse, UploadedFile } from '@/types/api'

interface AppState {
  // User config
  config: UserConfig

  // Current generation state
  currentFile: UploadedFile | null
  selectedModel: AIModel
  selectedLanguage: ProgrammingLanguage
  isGenerating: boolean
  generateResult: GenerateResponse | null
  generateError: string | null

  // Actions
  setConfig: (config: Partial<UserConfig>) => void
  setCurrentFile: (file: UploadedFile | null) => void
  setSelectedModel: (model: AIModel) => void
  setSelectedLanguage: (language: ProgrammingLanguage) => void
  setIsGenerating: (isGenerating: boolean) => void
  setGenerateResult: (result: GenerateResponse | null) => void
  setGenerateError: (error: string | null) => void
  reset: () => void
}

const defaultConfig: UserConfig = {
  default_model: 'qwen',
  default_language: 'react',
  api_keys: {
    qwen: '',
    minimax: '',
    kimi: '',
    glm: '',
  },
  custom_base_urls: {
    qwen: '',
    minimax: '',
    kimi: '',
    glm: '',
  },
}

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      // Initial state
      config: defaultConfig,
      currentFile: null,
      selectedModel: 'qwen',
      selectedLanguage: 'react',
      isGenerating: false,
      generateResult: null,
      generateError: null,

      // Actions
      setConfig: (newConfig) =>
        set((state) => ({
          config: { ...state.config, ...newConfig },
        })),

      setCurrentFile: (file) => set({ currentFile: file }),

      setSelectedModel: (model) => set({ selectedModel: model }),

      setSelectedLanguage: (language) => set({ selectedLanguage: language }),

      setIsGenerating: (isGenerating) => set({ isGenerating }),

      setGenerateResult: (result) => set({ generateResult: result, generateError: null }),

      setGenerateError: (error) => set({ generateError: error, isGenerating: false }),

      reset: () =>
        set({
          currentFile: null,
          generateResult: null,
          generateError: null,
          isGenerating: false,
        }),
    }),
    {
      name: 'ai-screen-code-storage',
      partialize: (state) => ({ config: state.config }),
    }
  )
)
