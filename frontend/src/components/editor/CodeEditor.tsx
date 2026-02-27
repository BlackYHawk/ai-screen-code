import { useState, useCallback, useRef } from 'react'
import Editor, { type Monaco, type OnMount } from '@monaco-editor/react'
import type { editor } from 'monaco-editor'
import { Copy, Download, Check, Sun, Moon, Search, UnfoldVertical } from 'lucide-react'
import { Button } from '../common'
import type { ProgrammingLanguage } from '@/types/api'

interface CodeEditorProps {
  code: string
  language: ProgrammingLanguage
  readOnly?: boolean
  showToolbar?: boolean
  height?: string
}

const languageMap: Record<ProgrammingLanguage, string> = {
  kotlin: 'kotlin',
  react: 'typescript',
  swift: 'swift',
  vue: 'vue',
}

const extensionMap: Record<ProgrammingLanguage, string> = {
  kotlin: 'kt',
  react: 'tsx',
  swift: 'swift',
  vue: 'vue',
}

export function CodeEditor({
  code,
  language,
  readOnly = true,
  showToolbar = true,
  height = '100%',
}: CodeEditorProps) {
  const [copied, setCopied] = useState(false)
  const [isDark, setIsDark] = useState(false)
  const [showSearch, setShowSearch] = useState(false)
  const [isFolded, setIsFolded] = useState(false)
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null)
  const monacoRef = useRef<Monaco | null>(null)

  const handleCopy = useCallback(async () => {
    try {
      await navigator.clipboard.writeText(code)
      setCopied(true)
      setTimeout(() => setCopied(false), 2000)
    } catch (err) {
      console.error('Failed to copy:', err)
    }
  }, [code])

  const handleDownload = useCallback(() => {
    const extension = extensionMap[language]
    const filename = `generated-code.${extension}`
    const blob = new Blob([code], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = filename
    a.click()
    URL.revokeObjectURL(url)
  }, [code, language])

  const toggleTheme = useCallback(() => {
    setIsDark((prev) => !prev)
    if (monacoRef.current) {
      monacoRef.current.editor.setTheme(isDark ? 'vs-light' : 'vs-dark')
    }
  }, [isDark])

  const toggleSearch = useCallback(() => {
    if (editorRef.current) {
      if (!showSearch) {
        editorRef.current.getAction('actions.find')?.run()
      } else {
        editorRef.current.getAction('editor.action.closeFindWidget')?.run()
      }
      setShowSearch((prev) => !prev)
    }
  }, [showSearch])

  const toggleFold = useCallback(() => {
    if (editorRef.current) {
      if (isFolded) {
        editorRef.current.getAction('editor.unfoldAll')?.run()
      } else {
        editorRef.current.getAction('editor.foldAll')?.run()
      }
      setIsFolded(!isFolded)
    }
  }, [isFolded])

  const handleEditorMount: OnMount = (editor, monaco) => {
    editorRef.current = editor
    monacoRef.current = monaco

    // Configure editor options
    editor.updateOptions({
      folding: true,
      foldingStrategy: 'indentation',
      showFoldingControls: 'always',
      wordWrap: 'on',
      bracketPairColorization: {
        enabled: true,
      },
      guides: {
        bracketPairs: true,
        indentation: true,
      },
    })
  }

  const getLanguageLabel = (lang: ProgrammingLanguage): string => {
    const labels: Record<ProgrammingLanguage, string> = {
      kotlin: 'Kotlin',
      react: 'React + TypeScript',
      swift: 'Swift',
      vue: 'Vue 3 + TypeScript',
    }
    return labels[lang]
  }

  return (
    <div className="flex flex-col h-full border border-gray-200 rounded-lg overflow-hidden bg-white">
      {/* Toolbar */}
      {showToolbar && (
        <div className="flex items-center justify-between px-4 py-2 border-b border-gray-200 bg-gray-50">
          <span className="text-sm font-medium text-gray-600">
            {getLanguageLabel(language)}
          </span>
          <div className="flex items-center space-x-1">
            <Button
              variant="ghost"
              size="sm"
              onClick={toggleSearch}
              className="flex items-center space-x-1"
              title="搜索 (Ctrl/Cmd + F)"
            >
              <Search className="w-4 h-4" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={toggleFold}
              className="flex items-center space-x-1"
              title={isFolded ? '展开全部' : '折叠全部'}
            >
              <UnfoldVertical className="w-4 h-4" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={toggleTheme}
              className="flex items-center space-x-1"
              title={isDark ? '浅色模式' : '深色模式'}
            >
              {isDark ? (
                <Sun className="w-4 h-4" />
              ) : (
                <Moon className="w-4 h-4" />
              )}
            </Button>
            <div className="w-px h-4 bg-gray-300 mx-1" />
            <Button
              variant="ghost"
              size="sm"
              onClick={handleCopy}
              className="flex items-center space-x-1"
            >
              {copied ? (
                <>
                  <Check className="w-4 h-4 text-green-500" />
                  <span className="text-green-600 text-xs">已复制</span>
                </>
              ) : (
                <>
                  <Copy className="w-4 h-4" />
                  <span className="text-xs">复制</span>
                </>
              )}
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={handleDownload}
              className="flex items-center space-x-1"
            >
              <Download className="w-4 h-4" />
              <span className="text-xs">下载</span>
            </Button>
          </div>
        </div>
      )}

      {/* Editor */}
      <div className="flex-1" style={{ minHeight: height === '100%' ? '400px' : height }}>
        <Editor
          height={height}
          language={languageMap[language]}
          value={code}
          theme={isDark ? 'vs-dark' : 'vs-light'}
          onMount={handleEditorMount}
          options={{
            readOnly,
            minimap: { enabled: false },
            fontSize: 14,
            lineNumbers: 'on',
            scrollBeyondLastLine: false,
            automaticLayout: true,
            padding: { top: 16, bottom: 16 },
            folding: true,
            foldingHighlight: true,
            showFoldingControls: 'always',
            renderLineHighlight: 'all',
            scrollbar: {
              verticalScrollbarSize: 10,
              horizontalScrollbarSize: 10,
            },
          }}
        />
      </div>
    </div>
  )
}
