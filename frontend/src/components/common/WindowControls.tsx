import { Minus, Square, X, Maximize2 } from 'lucide-react'
import { useEffect, useState } from 'react'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { isTauri } from '@/utils/runtime'

export function WindowControls() {
  const [isMaximized, setIsMaximized] = useState(false)

  useEffect(() => {
    if (!isTauri()) return

    const checkMaximized = async () => {
      try {
        const appWindow = getCurrentWindow()
        const maximized = await appWindow.isMaximized()
        setIsMaximized(maximized)
      } catch (error) {
        console.error('Failed to check maximized state:', error)
      }
    }

    checkMaximized()

    // Listen for window state changes
    const appWindow = getCurrentWindow()
    const unlisten = appWindow.onResized(async () => {
      try {
        const maximized = await appWindow.isMaximized()
        setIsMaximized(maximized)
      } catch {
        // Ignore errors during cleanup
      }
    })

    return () => {
      unlisten.then((fn) => fn())
    }
  }, [])

  const handleMinimize = async () => {
    if (!isTauri()) return
    try {
      const appWindow = getCurrentWindow()
      await appWindow.minimize()
    } catch (error) {
      console.error('Failed to minimize:', error)
    }
  }

  const handleMaximize = async () => {
    if (!isTauri()) return
    try {
      const appWindow = getCurrentWindow()
      await appWindow.toggleMaximize()
    } catch (error) {
      console.error('Failed to toggle maximize:', error)
    }
  }

  const handleClose = async () => {
    if (!isTauri()) return
    try {
      const appWindow = getCurrentWindow()
      await appWindow.close()
    } catch (error) {
      console.error('Failed to close:', error)
    }
  }

  if (!isTauri()) return null

  return (
    <div className="flex items-center">
      <button
        onClick={handleMinimize}
        className="flex items-center justify-center w-10 h-10 text-gray-500 hover:bg-gray-200 transition-colors"
        aria-label="Minimize"
      >
        <Minus className="w-4 h-4" />
      </button>
      <button
        onClick={handleMaximize}
        className="flex items-center justify-center w-10 h-10 text-gray-500 hover:bg-gray-200 transition-colors"
        aria-label={isMaximized ? 'Restore' : 'Maximize'}
      >
        {isMaximized ? <Square className="w-3.5 h-3.5" /> : <Maximize2 className="w-4 h-4" />}
      </button>
      <button
        onClick={handleClose}
        className="flex items-center justify-center w-10 h-10 text-gray-500 hover:bg-red-500 hover:text-white transition-colors"
        aria-label="Close"
      >
        <X className="w-4 h-4" />
      </button>
    </div>
  )
}
