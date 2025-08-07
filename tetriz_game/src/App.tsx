import React, { useEffect, useState } from 'react'
import styled from 'styled-components'
import GameBoard from './components/GameBoard'
import GameInfo from './components/GameInfo'
import Controls from './components/Controls'
import { useGame } from './hooks/useGame'

const AppContainer = styled.div`
  display: flex;
  gap: 2rem;
  padding: 2rem;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 20px;
  backdrop-filter: blur(10px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(255, 255, 255, 0.2);
`

const GameContainer = styled.div`
  display: flex;
  flex-direction: column;
  gap: 1rem;
`

const Title = styled.h1`
  color: white;
  text-align: center;
  margin: 0 0 1rem 0;
  font-size: 2.5rem;
  text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
`

const App: React.FC = () => {
  const { game, gameState, isLoading } = useGame()

  if (isLoading) {
    return (
      <div style={{ color: 'white', fontSize: '1.5rem' }}>
        Loading Tetris...
      </div>
    )
  }

  return (
    <AppContainer>
      <GameContainer>
        <Title>TETRIS</Title>
        <GameBoard game={game} />
        <Controls game={game} />
      </GameContainer>
      <GameInfo game={game} gameState={gameState} />
    </AppContainer>
  )
}

export default App