import { useEffect, useState, useCallback } from 'react'

let wasmModule: any = null

export const useGame = () => {
  const [game, setGame] = useState<any>(null)
  const [gameState, setGameState] = useState<any>(null)
  const [isLoading, setIsLoading] = useState(true)

  useEffect(() => {
    const initWasm = async () => {
      try {
        wasmModule = await import('../../pkg/tetriz_game.js')
        await wasmModule.default()
        const newGame = new wasmModule.TetrisGame()
        setGame(newGame)
        setIsLoading(false)
      } catch (error) {
        console.error('Failed to load WASM module:', error)
      }
    }

    initWasm()
  }, [])

  const updateGameState = useCallback(() => {
    if (game) {
      setGameState({
        board: game.get_board(),
        currentPiece: game.get_current_piece(),
        nextPiece: game.get_next_piece(),
        scoring: game.get_scoring(),
        state: game.get_state(),
        isGameOver: game.is_game_over()
      })
    }
  }, [game])

  useEffect(() => {
    if (game) {
      updateGameState()
      
      const gameLoop = () => {
        if (game && !game.is_game_over()) {
          game.update(Date.now())
          updateGameState()
        }
        requestAnimationFrame(gameLoop)
      }
      
      requestAnimationFrame(gameLoop)
    }
  }, [game, updateGameState])

  return { game, gameState, isLoading, updateGameState }
}