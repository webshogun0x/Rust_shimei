import React from 'react'
import styled from 'styled-components'

const BoardContainer = styled.div`
  display: grid;
  grid-template-columns: repeat(10, 30px);
  grid-template-rows: repeat(20, 30px);
  gap: 1px;
  background: #000;
  border: 3px solid #333;
  border-radius: 8px;
  padding: 10px;
`

const Cell = styled.div<{ filled: boolean; color?: string }>`
  width: 30px;
  height: 30px;
  background: ${props => props.filled ? props.color || '#666' : '#111'};
  border: ${props => props.filled ? '1px solid #fff' : '1px solid #333'};
  border-radius: 2px;
  box-shadow: ${props => props.filled ? 'inset 0 0 5px rgba(255,255,255,0.3)' : 'none'};
`

const getPieceColor = (pieceType: number): string => {
  const colors = ['#00f0f0', '#f0f000', '#a000f0', '#00f000', '#f00000', '#0000f0', '#f0a000']
  return colors[pieceType] || '#666'
}

interface GameBoardProps {
  game: any
}

const GameBoard: React.FC<GameBoardProps> = ({ game }) => {
  if (!game) return null

  const board = game.get_board()
  const grid = board.get_grid()
  const currentPiece = game.get_current_piece()

  // Create a display grid combining board and current piece
  const displayGrid = Array(20).fill(null).map(() => Array(10).fill(null))
  
  // Fill with board pieces
  for (let y = 0; y < 20; y++) {
    for (let x = 0; x < 10; x++) {
      if (grid[y][x] !== null) {
        displayGrid[y][x] = { type: grid[y][x], isActive: false }
      }
    }
  }

  // Add current piece
  if (currentPiece) {
    const blocks = currentPiece.get_blocks()
    blocks.forEach(([x, y]: [number, number]) => {
      if (y >= 0 && y < 20 && x >= 0 && x < 10) {
        displayGrid[y][x] = { type: currentPiece.piece_type, isActive: true }
      }
    })
  }

  return (
    <BoardContainer>
      {displayGrid.flat().map((cell, index) => (
        <Cell
          key={index}
          filled={cell !== null}
          color={cell ? getPieceColor(cell.type) : undefined}
        />
      ))}
    </BoardContainer>
  )
}

export default GameBoard