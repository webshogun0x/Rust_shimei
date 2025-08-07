import React, { useEffect } from 'react'
import styled from 'styled-components'

const ControlsContainer = styled.div`
  display: flex;
  gap: 1rem;
  justify-content: center;
  flex-wrap: wrap;
`

const ControlButton = styled.button`
  background: rgba(255, 255, 255, 0.2);
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.3);
  padding: 0.8rem 1.5rem;
  border-radius: 25px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.2s;
  backdrop-filter: blur(5px);

  &:hover {
    background: rgba(255, 255, 255, 0.3);
    transform: translateY(-2px);
  }

  &:active {
    transform: translateY(0);
  }
`

interface ControlsProps {
  game: any
}

const Controls: React.FC<ControlsProps> = ({ game }) => {
  useEffect(() => {
    if (!game) return

    const handleKeyPress = (event: KeyboardEvent) => {
      switch (event.key.toLowerCase()) {
        case 'arrowleft':
        case 'a':
          event.preventDefault()
          game.move_left()
          break
        case 'arrowright':
        case 'd':
          event.preventDefault()
          game.move_right()
          break
        case 'arrowdown':
        case 's':
          event.preventDefault()
          game.soft_drop()
          break
        case 'arrowup':
        case 'w':
          event.preventDefault()
          game.rotate()
          break
        case ' ':
          event.preventDefault()
          game.hard_drop()
          break
        case 'p':
          event.preventDefault()
          game.pause()
          break
        case 'r':
          event.preventDefault()
          game.reset()
          break
      }
    }

    window.addEventListener('keydown', handleKeyPress)
    return () => window.removeEventListener('keydown', handleKeyPress)
  }, [game])

  if (!game) return null

  return (
    <ControlsContainer>
      <ControlButton onClick={() => game.move_left()}>
        ← Move Left
      </ControlButton>
      <ControlButton onClick={() => game.move_right()}>
        Move Right →
      </ControlButton>
      <ControlButton onClick={() => game.rotate()}>
        ↑ Rotate
      </ControlButton>
      <ControlButton onClick={() => game.soft_drop()}>
        ↓ Soft Drop
      </ControlButton>
      <ControlButton onClick={() => game.hard_drop()}>
        Hard Drop
      </ControlButton>
      <ControlButton onClick={() => game.pause()}>
        Pause
      </ControlButton>
      <ControlButton onClick={() => game.reset()}>
        Restart
      </ControlButton>
    </ControlsContainer>
  )
}

export default Controls