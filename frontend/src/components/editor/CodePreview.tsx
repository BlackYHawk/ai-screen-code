import { useState, useCallback, useMemo, useEffect, useRef } from 'react'
import { Copy, Download, Check, RefreshCw, ExternalLink, AlertCircle } from 'lucide-react'
import { Button } from '../common'
import type { ProgrammingLanguage } from '@/types/api'

interface CodePreviewProps {
  code: string
  language: ProgrammingLanguage
}

// Web languages that can be previewed in browser
const webLanguages: ProgrammingLanguage[] = ['react', 'vue']

// Non-web languages that show code explanation
const nativeLanguages: ProgrammingLanguage[] = ['kotlin', 'swift']

export function CodePreview({ code, language }: CodePreviewProps) {
  const [copied, setCopied] = useState(false)
  const [iframeKey, setIframeKey] = useState(0)
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const iframeRef = useRef<HTMLIFrameElement>(null)

  const isWebLanguage = webLanguages.includes(language)
  const isNativeLanguage = nativeLanguages.includes(language)

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
    const extension = language === 'react' ? 'tsx' : language === 'vue' ? 'vue' : language
    const filename = `generated-code.${extension}`
    const blob = new Blob([code], { type: 'text/plain' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = filename
    a.click()
    URL.revokeObjectURL(url)
  }, [code, language])

  const handleRefresh = useCallback(() => {
    setIframeKey((prev) => prev + 1)
    setError(null)
  }, [])

  const handleOpenExternal = useCallback(() => {
    if (iframeRef.current) {
      // Try to open in new tab if possible
      const htmlContent = generatePreviewHtml(code, language)
      const blob = new Blob([htmlContent], { type: 'text/html' })
      const url = URL.createObjectURL(blob)
      window.open(url, '_blank')
    }
  }, [code, language])

  // Generate preview HTML for web languages
  const generatePreviewHtml = (previewCode: string, lang: ProgrammingLanguage): string => {
    if (lang === 'react') {
      return `
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>React Preview</title>
  <script src="https://unpkg.com/react@18/umd/react.development.js" crossorigin></script>
  <script src="https://unpkg.com/react-dom@18/umd/react-dom.development.js" crossorigin></script>
  <script src="https://unpkg.com/@babel/standalone/babel.min.js"></script>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
    #root { padding: 16px; }
  </style>
</head>
<body>
  <div id="root"></div>
  <script type="text/babel">
    ${previewCode}
    const root = ReactDOM.createRoot(document.getElementById('root'));
    root.render(<App />);
  </script>
</body>
</html>`
    }

    if (lang === 'vue') {
      return `
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Vue Preview</title>
  <script src="https://unpkg.com/vue@3/dist/vue.global.js"></script>
  <style>
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; }
    #app { padding: 16px; }
  </style>
</head>
<body>
  <div id="app"></div>
  <script>
    const { createApp, ref } = Vue;
    ${previewCode}
    createApp({
      components: { App },
      setup() {
        return {};
      }
    }).mount('#app');
  </script>
</body>
</html>`
    }

    return ''
  }

  // Generate explanation for native languages
  const codeExplanation = useMemo(() => {
    if (!isNativeLanguage) return null

    const explanations: Record<ProgrammingLanguage, { title: string; description: string; features: string[] }> = {
      kotlin: {
        title: 'Kotlin (Jetpack Compose)',
        description: '生成的代码使用 Jetpack Compose，这是 Android 现代 UI 工具包。',
        features: [
          '声明式 UI 编程',
          'Composable 函数构建界面',
          'Material Design 3 组件',
          '状态管理与 LiveData/StateFlow',
        ],
      },
      swift: {
        title: 'Swift (SwiftUI)',
        description: '生成的代码使用 SwiftUI，这是 Apple 的现代 UI 框架。',
        features: [
          '声明式 UI 编程',
          '@State 和 @Binding 状态管理',
          'SwiftUI 视图构建器',
          '跨平台支持 (iOS, macOS, watchOS)',
        ],
      },
      react: { title: '', description: '', features: [] },
      vue: { title: '', description: '', features: [] },
    }

    return explanations[language]
  }, [language, isNativeLanguage])

  // Handle iframe load
  const handleIframeLoad = useCallback(() => {
    setIsLoading(false)
  }, [])

  const handleIframeError = useCallback(() => {
    setIsLoading(false)
    setError('预览加载失败，请检查代码语法')
  }, [])

  // Generate the preview URL
  const previewHtml = useMemo(() => {
    if (!isWebLanguage) return ''
    return generatePreviewHtml(code, language)
  }, [code, language, isWebLanguage])

  useEffect(() => {
    if (isWebLanguage && iframeRef.current) {
      setIsLoading(true)
      const blob = new Blob([previewHtml], { type: 'text/html' })
      const url = URL.createObjectURL(blob)
      iframeRef.current.src = url
      return () => URL.revokeObjectURL(url)
    }
  }, [previewHtml, isWebLanguage, iframeKey])

  if (isWebLanguage) {
    return (
      <div className="flex flex-col h-full border border-gray-200 rounded-lg overflow-hidden bg-white">
        {/* Preview Toolbar */}
        <div className="flex items-center justify-between px-4 py-2 border-b border-gray-200 bg-gray-50">
          <span className="text-sm font-medium text-gray-600">预览</span>
          <div className="flex items-center space-x-1">
            <Button
              variant="ghost"
              size="sm"
              onClick={handleRefresh}
              className="flex items-center space-x-1"
              title="刷新预览"
            >
              <RefreshCw className="w-4 h-4" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={handleOpenExternal}
              className="flex items-center space-x-1"
              title="在新窗口打开"
            >
              <ExternalLink className="w-4 h-4" />
            </Button>
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

        {/* Iframe Preview */}
        <div className="flex-1 relative bg-white min-h-[400px]">
          {isLoading && (
            <div className="absolute inset-0 flex items-center justify-center bg-white/80 z-10">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-500" />
            </div>
          )}
          {error && (
            <div className="absolute top-2 left-2 right-2 bg-red-50 border border-red-200 rounded-lg p-3 flex items-start space-x-2 z-10">
              <AlertCircle className="w-5 h-5 text-red-500 flex-shrink-0" />
              <p className="text-sm text-red-600">{error}</p>
            </div>
          )}
          <iframe
            ref={iframeRef}
            key={iframeKey}
            title="Code Preview"
            sandbox="allow-scripts allow-same-origin"
            className="w-full h-full border-0"
            onLoad={handleIframeLoad}
            onError={handleIframeError}
          />
        </div>
      </div>
    )
  }

  if (isNativeLanguage && codeExplanation) {
    return (
      <div className="flex flex-col h-full border border-gray-200 rounded-lg overflow-hidden bg-white">
        {/* Explanation Toolbar */}
        <div className="flex items-center justify-between px-4 py-2 border-b border-gray-200 bg-gray-50">
          <span className="text-sm font-medium text-gray-600">代码说明</span>
          <div className="flex items-center space-x-1">
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

        {/* Code Explanation */}
        <div className="flex-1 overflow-auto p-4">
          <div className="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-4">
            <h3 className="text-lg font-semibold text-blue-900 mb-2">
              {codeExplanation.title}
            </h3>
            <p className="text-blue-800 text-sm">{codeExplanation.description}</p>
          </div>

          <div className="mb-4">
            <h4 className="text-sm font-medium text-gray-700 mb-2">主要特性:</h4>
            <ul className="space-y-2">
              {codeExplanation.features.map((feature, index) => (
                <li key={index} className="flex items-start space-x-2">
                  <span className="w-2 h-2 mt-1.5 rounded-full bg-green-500 flex-shrink-0" />
                  <span className="text-sm text-gray-600">{feature}</span>
                </li>
              ))}
            </ul>
          </div>

          <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-3">
            <div className="flex items-start space-x-2">
              <AlertCircle className="w-5 h-5 text-yellow-600 flex-shrink-0" />
              <div className="text-sm text-yellow-800">
                <p className="font-medium mb-1">使用说明:</p>
                <p>
                  此代码需要使用相应的 IDE
                  运行。对于 Kotlin，请使用 Android Studio 或 IntelliJ
                  IDEA。对于 Swift，请使用 Xcode。
                </p>
              </div>
            </div>
          </div>
        </div>
      </div>
    )
  }

  return null
}
