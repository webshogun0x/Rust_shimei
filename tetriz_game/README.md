# Tetris Game - Rust WASM + React

A modern, modular Tetris game built with Rust WebAssembly backend and React frontend.

## Architecture

### Rust WASM Core (`src/`)
- **`lib.rs`**: Main WASM entry point and bindings
- **`game.rs`**: Core game logic and state management
- **`board.rs`**: Game board, collision detection, line clearing
- **`pieces.rs`**: Tetris pieces, rotations, and movement
- **`scoring.rs`**: Score calculation, levels, and speed control

### React Frontend (`src/`)
- **`App.tsx`**: Main application component
- **`hooks/useGame.ts`**: WASM integration and game state management
- **`components/GameBoard.tsx`**: Game grid rendering
- **`components/GameInfo.tsx`**: Score, next piece, game over UI
- **`components/Controls.tsx`**: Input handling and control buttons

## Features

- ✅ Complete Tetris gameplay mechanics
- ✅ All 7 standard Tetris pieces (I, O, T, S, Z, J, L)
- ✅ Piece rotation with wall kicks
- ✅ Line clearing with scoring
- ✅ Progressive difficulty levels
- ✅ Modern glassmorphism UI design
- ✅ Keyboard and button controls
- ✅ Game pause/resume
- ✅ Game over screen with restart

## Setup & Development

### Prerequisites
- Rust (latest stable)
- Node.js (v18+)
- wasm-pack

### Installation

1. **Install wasm-pack**:
   ```bash
   curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
   ```

2. **Build WASM module**:
   ```bash
   npm run build-wasm
   ```

3. **Install frontend dependencies**:
   ```bash
   npm install
   ```

4. **Start development server**:
   ```bash
   npm run dev
   ```

## Controls

- **Arrow Keys / WASD**: Move and rotate pieces
- **Space**: Hard drop
- **P**: Pause/Resume
- **R**: Restart game

## System Architecture Benefits

This modular design demonstrates:

1. **Separation of Concerns**: Game logic (Rust) vs UI (React)
2. **Performance**: Critical game logic runs in WASM at near-native speed
3. **Type Safety**: Rust's type system prevents common game bugs
4. **Modern Web**: React with hooks and styled-components
5. **Cross-Platform**: Runs in any modern web browser

## Build for Production

```bash
npm run build-wasm
npm run build
```

The built files will be in the `dist/` directory, ready for deployment.