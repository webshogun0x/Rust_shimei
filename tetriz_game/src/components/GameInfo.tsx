import React from 'react'
import styled from 'styled-components'

const InfoContainer = styled.div`
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  min-width: 200px;
`

const InfoPanel = styled.div`
  background: rgba(255, 255, 255, 0.1);
  border-radius: 12px;
  padding: 1.5rem;
  backdrop-filter: blur(5px);
  border: 1px solid rgba(255, 255, 255, 0.2);
`

const InfoTitle = styled.h3`
  color: white;
  margin: 0 0 1rem 0;
  font-size: 1.2rem;
  text-align: center;
`

const InfoText = styled.p`
  color: white;
  margin: 0.5rem 0;
  font-size: 1rem;
  display: flex;
  justify-content: space-between;
`

const NextPieceContainer = styled.div`
  display: grid;
  grid-template-columns: repeat(4, 20px);
  gap: 1px;
  justify-content: center;
  margin-top: 1rem;
`

const NextPieceCell = styled.div<{ filled: boolean; color?: string }>`
  width: 20px;
  height: 20px;
  background: ${props => props.filled ? props.color || '#666' : 'transparent'};
  border: ${props => props.filled ? '1px solid #fff' : 'none'};
  border-radius: 2px;
`

const GameOverOverlay = styled.div`
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
`

const GameOverPanel = styled.div`
  background: rgba(255, 255, 255, 0.1);
  border-radius: 20px;
  padding: 3rem;
  text-align: center;
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
`

const GameOverTitle = styled.h2`
  color: #ff4444;
  font-size: 2.5rem;
  margin: 0 0 1rem 0;
`

const RestartButton = styled.button`
  background: linear-gradient(45deg, #667eea, #764ba2);
  color: white;
  border: none;
  padding: 1rem 2rem;
  border-radius: 25px;
  font-size: 1.2rem;
  cursor: pointer;
  margin-top: 1rem;
  transition: transform 0.2s;

  &:hover {
    transform: scale(1.05);
  }
`

const getPieceColor = (pieceType: number): string => {
  const colors = ['#00f0f0', '#f0f000', '#a000f0', '#00f000', '#f00000', '#0000f0', '#f0a000']
  return colors[pieceType] || '#666'
}

interface GameInfoProps {
  game: any
  gameState: any
}

const GameInfo: React.FC<GameInfoProps> = ({ game, gameState }) => {
  if (!game || !gameState) return null

  const { scoring, nextPiece, isGameOver } = gameState

  const renderNextPiece = () => {
    if (!nextPiece) return null
    
    const blocks = nextPiece.get_blocks()
    const grid = Array(16).fill(false)
    
    blocks.forEach(([x, y]: [number, number]) => {
      const adjustedX = x - 4 + 1
      const adjustedY = y + 1
      if (adjustedX >= 0 && adjustedX < 4 && adjustedY >= 0 && adjustedY < 4) {
        grid[adjustedY * 4 + adjustedX] = true
      }
    })

    return grid.map((filled, index) => (
      <NextPieceCell
        key={index}
        filled={filled}
        color={filled ? getPieceColor(nextPiece.piece_type) : undefined}
      />
    ))
  }

  return (
    <>
      <InfoContainer>
        <InfoPanel>
          <InfoTitle>Score</InfoTitle>
          <InfoText>
            <span>Points:</span>
            <span>{scoring.score.toLocaleString()}</span>
          </InfoText>
          <InfoText>
            <span>Level:</span>
            <span>{scoring.level}</span>
          </InfoText>
          <InfoText>
            <span>Lines:</span>
            <span>{scoring.lines_cleared}</span>
          </InfoText>
        </InfoPanel>

        <InfoPanel>
          <InfoTitle>Next Piece</InfoTitle>
          <NextPieceContainer>
            {renderNextPiece()}
          </NextPieceContainer>
        </InfoPanel>

        <InfoPanel>
          <InfoTitle>Controls</InfoTitle>
          <InfoText style={{ flexDirection: 'column', gap: '0.5rem' }}>
            <span>← → Move</span>
            <span>↑ Rotate</span>
            <span>↓ Soft Drop</span>
            <span>Space Hard Drop</span>
            <span>P Pause</span>
            <span>R Restart</span>
          </InfoText>
        </InfoPanel>
      </InfoContainer>

      {isGameOver && (
        <GameOverOverlay>
          <GameOverPanel>
            <GameOverTitle>GAME OVER</GameOverTitle>
            <InfoText>
              <span>Final Score:</span>
              <span>{scoring.score.toLocaleString()}</span>
            </InfoText>
            <InfoText>
              <span>Level Reached:</span>
              <span>{scoring.level}</span>
            </InfoText>
            <RestartButton onClick={() => game.reset()}>
              Play Again
            </RestartButton>
          </GameOverPanel>
        </GameOverOverlay>
      )}
    </>
  )
}

export default GameInfo