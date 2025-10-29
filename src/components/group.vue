<template>
  <div class="group-container">
    <div
      class="group"
      @click="handleClick"
      :class="{ disabled: isVisible || anyModalOpen }"
    >
      <!-- Row 0 (Player 1) -->
      <p
        class="name"
        :class="{ 'player-selected': players[0]?.id === $parent.selectedPlayer1Id || players[0]?.id === $parent.selectedPlayer2Id }"
        @click.stop="handlePlayerClick(0)"
      >
        {{ players[0]?.name || 'Nome 1' }}
      </p>
      <div class="cell" :class="{ filled: gridScores[0][0] !== '' || 0 === 0 }" data-row="0" data-col="0">
        <span>{{ gridScores[0][0] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[0][1] !== '' }" data-row="0" data-col="1">
        <span>{{ gridScores[0][1] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[0][2] !== '' }" data-row="0" data-col="2">
        <span>{{ gridScores[0][2] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[0][3] !== '' }" data-row="0" data-col="3">
        <span>{{ gridScores[0][3] }}</span>
      </div>

      <!-- Row 1 (Player 2) -->
      <p
        class="name"
        :class="{ 'player-selected': players[1]?.id === $parent.selectedPlayer1Id || players[1]?.id === $parent.selectedPlayer2Id }"
        @click.stop="handlePlayerClick(1)"
      >
        {{ players[1]?.name || 'Nome 2' }}
      </p>
      <div class="cell" :class="{ filled: gridScores[1][0] !== '' }" data-row="1" data-col="0">
        <span>{{ gridScores[1][0] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[1][1] !== '' || 1 === 1 }" data-row="1" data-col="1">
        <span>{{ gridScores[1][1] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[1][2] !== '' }" data-row="1" data-col="2">
        <span>{{ gridScores[1][2] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[1][3] !== '' }" data-row="1" data-col="3">
        <span>{{ gridScores[1][3] }}</span>
      </div>

      <!-- Row 2 (Player 3) -->
      <p
        class="name"
        :class="{ 'player-selected': players[2]?.id === $parent.selectedPlayer1Id || players[2]?.id === $parent.selectedPlayer2Id }"
        @click.stop="handlePlayerClick(2)"
      >
        {{ players[2]?.name || 'Nome 3' }}
      </p>
      <div class="cell" :class="{ filled: gridScores[2][0] !== '' }" data-row="2" data-col="0">
        <span>{{ gridScores[2][0] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[2][1] !== '' }" data-row="2" data-col="1">
        <span>{{ gridScores[2][1] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[2][2] !== '' || 2 === 2 }" data-row="2" data-col="2">
        <span>{{ gridScores[2][2] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[2][3] !== '' }" data-row="2" data-col="3">
        <span>{{ gridScores[2][3] }}</span>
      </div>

      <!-- Row 3 (Player 4) -->
      <p
        class="name"
        :class="{ 'player-selected': players[3]?.id === $parent.selectedPlayer1Id || players[3]?.id === $parent.selectedPlayer2Id }"
        @click.stop="handlePlayerClick(3)"
      >
        {{ players[3]?.name || 'Nome 4' }}
      </p>
      <div class="cell" :class="{ filled: gridScores[3][0] !== '' }" data-row="3" data-col="0">
        <span>{{ gridScores[3][0] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[3][1] !== '' }" data-row="3" data-col="1">
        <span>{{ gridScores[3][1] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[3][2] !== '' }" data-row="3" data-col="2">
        <span>{{ gridScores[3][2] }}</span>
      </div>
      <div class="cell" :class="{ filled: gridScores[3][3] !== '' || 3 === 3 }" data-row="3" data-col="3">
        <span>{{ gridScores[3][3] }}</span>
      </div>
    </div>

    <!-- Modal (teleported to body) -->
    <Teleport to="body">
      <div v-if="isVisible" class="modal">
        <div class="modal-content">
          <div class="match-header">
            <span class="player-name">{{ selectedPlayer1?.name || 'Giocatore 1' }}</span>
            <label class="toggle-switch">
              <input type="checkbox" v-model="mainSwitch" />
              <span class="switch-slider"></span>
            </label>
            <span class="player-name">{{ selectedPlayer2?.name || 'Giocatore 2' }}</span>
          </div>
          <h3>Inserisci i punteggi</h3>

          <div class="game-row">
            <span class="game-label">Game 1:</span>
            <div class="score-inputs">
              <input type="number" v-model.number="game1ScoreA" min="0" max="7" @input="clampToSeven($event, 'game1ScoreA')" class="score-box" />
              <span class="dash">-</span>
              <input type="number" v-model.number="game1ScoreB" min="0" max="7" @input="clampToSeven($event, 'game1ScoreB')" class="score-box" />
            </div>
          </div>

          <div class="game-row">
            <span class="game-label">Game 2:</span>
            <div class="score-inputs">
              <input type="number" v-model.number="game2ScoreA" min="0" max="7" @input="clampToSeven($event, 'game2ScoreA')" class="score-box" />
              <span class="dash">-</span>
              <input type="number" v-model.number="game2ScoreB" min="0" max="7" @input="clampToSeven($event, 'game2ScoreB')" class="score-box" />
            </div>
          </div>

          <div class="game-row">
            <span class="game-label">Tie:</span>
            <div class="score-inputs">
              <input type="number" v-model.number="tieScoreA" min="0" class="score-box" />
              <span class="dash">-</span>
              <input type="number" v-model.number="tieScoreB" min="0" class="score-box" />
            </div>
          </div>

          <div class="modal-buttons">
            <button class="confirm" @click.stop="submitScores">OK</button>
            <button class="cancel" @click.stop="closeModal">Annulla</button>
          </div>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core';

export default {
  name: 'Group',
  props: {
    players: { type: Array, default: () => [] },
    anyModalOpen: { type: Boolean, default: false },
    swapMode: { type: Boolean, default: false }
  },
  data() {
    return {
      isVisible: false,
      selectedPlayer1: null,
      selectedPlayer2: null,
      gridScores: Array(4).fill().map(() => Array(4).fill('')),
      selectedRow: -1,
      selectedCol: -1,
      game1ScoreA: 0,
      game1ScoreB: 0,
      game2ScoreA: 0,
      game2ScoreB: 0,
      tieScoreA: 0,
      tieScoreB: 0,
      mainSwitch: false
    };
  },
  methods: {
    clampToSeven(event, property) {
      const value = parseInt(event.target.value) || 0;
      const clamped = Math.min(value, 7);
      this[property] = clamped;
      event.target.value = clamped;
    },

    handleClick(event) {
      if (this.isVisible || this.anyModalOpen || this.swapMode) return;
      if (event.target.classList.contains('cell') && !event.target.classList.contains('filled')) {
        const row = parseInt(event.target.dataset.row);
        const col = parseInt(event.target.dataset.col);
        if (this.gridScores[row][col] !== '') return;

        this.selectedRow = row;
        this.selectedCol = col;
        this.selectedPlayer1 = this.players[row];
        this.selectedPlayer2 = this.players[col];
        this.game1ScoreA = 0;
        this.game1ScoreB = 0;
        this.game2ScoreA = 0;
        this.game2ScoreB = 0;
        this.tieScoreA = 0;
        this.tieScoreB = 0;
        this.mainSwitch = false;
        this.isVisible = true;
        this.$emit('modal-opened');
      }
    },

    closeModal() {
      this.$emit('modal-closed');
      this.isVisible = false;
      this.selectedPlayer1 = null;
      this.selectedPlayer2 = null;
      this.selectedRow = -1;
      this.selectedCol = -1;
      this.game1ScoreA = 0;
      this.game1ScoreB = 0;
      this.game2ScoreA = 0;
      this.game2ScoreB = 0;
      this.tieScoreA = 0;
      this.tieScoreB = 0;
      this.mainSwitch = false;
    },

    async submitScores() {
      // 1. Update the grid UI
      if (this.selectedRow !== -1 && this.selectedCol !== -1) {
        const g1 = `${this.game1ScoreA} - ${this.game1ScoreB}`;
        const g2 = `${this.game2ScoreA} - ${this.game2ScoreB}`;
        const tie = `${this.tieScoreA} - ${this.tieScoreB}`;
        this.gridScores[this.selectedRow][this.selectedCol] = `${g1}\n${g2}\n${tie}`;
      }

      // 2. Validate player IDs
      const p1_id = Number(this.selectedPlayer1?.id);
      const p2_id = Number(this.selectedPlayer2?.id);

      if (!Number.isInteger(p1Id) || !Number.isInteger(p2Id) || p1Id < 0 || p2Id < 0) {
        alert('Errore: gli ID dei giocatori non sono validi.');
        this.closeModal();
        return;
      }

      // 3. Handle main switch (swap scores if toggled)
      const [set1A, set1B] = this.mainSwitch
        ? [this.game1ScoreB, this.game1ScoreA]
        : [this.game1ScoreA, this.game1ScoreB];

      const [set2A, set2B] = this.mainSwitch
        ? [this.game2ScoreB, this.game2ScoreA]
        : [this.game2ScoreA, this.game2ScoreB];

      const [tieA, tieB] = this.mainSwitch
        ? [this.tieScoreB, this.tieScoreA]
        : [this.tieScoreA, this.tieScoreB];

      // 4. Build payload with EXACT snake_case keys
      const payload = {
        p1Id: p1_id,
        p2Id: p2_id,
        set1: [set1A, set1B],
        set2: [set2A, set2B],
        tie: [tieA, tieB]
      };

      alert('Sending to Rust:', payload); // ← Debug here

      // 5. Call Tauri command
      try {
        await invoke('save_match_result', payload);
        alert("BACKEND CALLED");
      } catch (error) {
        console.error('Tauri invoke failed:', error);
        alert('Errore durante il salvataggio del risultato: ' + error);
      } finally {
        this.closeModal();
      }
    },

    handlePlayerClick(index) {
      if (this.swapMode && !this.anyModalOpen) {
        const player = this.players[index];
        this.$emit('player-selected', {
          id: player.id ?? null,
          name: player.name ?? `Giocatore ${index + 1}`
        });
      }
    }
  }
};
</script>

<style scoped>
.group-container {
  display: flex;
  justify-content: center;
  align-items: center;
  position: relative;
}

.group {
  display: grid;
  grid-template-columns: 120px repeat(4, 60px);
  grid-template-rows: repeat(4, 60px);
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(12px);
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  overflow: hidden;
  border: 1px solid rgba(255, 255, 255, 0.3);
  transition: transform 0.3s ease, box-shadow 0.3s ease;
  position: relative;
}

.group:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
}

.name {
  background: rgba(255, 255, 255, 0.7);
  color: #2c3e50;
  font-weight: 600;
  font-size: 1rem;
  border-right: 1px solid rgba(0, 0, 0, 0.05);
  display: flex;
  align-items: center;
  padding-left: 1rem;
  margin: 0;
  cursor: pointer;
  transition: background 0.3s ease;
}

.name:hover {
  background: rgba(79, 172, 254, 0.15);
}

.player-selected {
  background: rgba(79, 172, 254, 0.3);
}

.cell {
  border: 1px solid rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: white;
  cursor: pointer;
  transition: background-color 0.25s ease, transform 0.15s ease;
  font-size: 0.7rem;
  line-height: 1.1;
}

.cell:hover:not(.filled) {
  background-color: rgba(79, 172, 254, 0.15);
  transform: scale(1.05);
}

.cell span {
  font-weight: bold;
  color: #2c3e50;
  white-space: pre-line;
  text-align: center;
}

.filled {
  background: linear-gradient(135deg, #4facfe, #00f2fe);
  border: none;
  box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.2);
  cursor: default;
  pointer-events: none;
}

.disabled {
  pointer-events: none;
  opacity: 0.5;
  filter: blur(1px);
}

.modal {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(44, 62, 80, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  backdrop-filter: blur(6px);
}

.modal-content {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 16px;
  padding: 2rem;
  width: 300px;
  max-width: 90%;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  text-align: center;
  filter: none !important;
  -webkit-filter: none !important;
  isolation: isolate;
  transform: translateZ(0);
  will-change: transform;
  z-index: 1001;
}

.match-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
  padding: 0.5rem;
  background: rgba(79, 172, 254, 0.1);
  border-radius: 8px;
}

.player-name {
  font-weight: 600;
  color: #2c3e50;
  font-size: 1.1rem;
}

.toggle-switch {
  position: relative;
  display: inline-block;
  width: 50px;
  height: 24px;
}

.toggle-switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.switch-slider {
  position: absolute;
  cursor: pointer;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: #ccc;
  transition: 0.3s;
  border-radius: 24px;
}

.switch-slider:before {
  position: absolute;
  content: "";
  height: 18px;
  width: 18px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  transition: 0.3s;
  border-radius: 50%;
}

input:checked + .switch-slider {
  background-color: #4facfe;
}

input:checked + .switch-slider:before {
  transform: translateX(26px);
}

.game-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
  padding: 0.5rem;
  background: rgba(255, 255, 255, 0.5);
  border-radius: 8px;
}

.game-label {
  font-weight: 600;
  color: #2c3e50;
  min-width: 60px;
}

.score-inputs {
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.score-box {
  width: 40px;
  height: 40px;
  border: 2px solid #ccc;
  border-radius: 8px;
  text-align: center;
  font-size: 1rem;
  font-weight: bold;
  padding: 0;
  transition: border-color 0.3s ease;
}

.score-box:focus {
  border-color: #4facfe;
  outline: none;
  box-shadow: 0 0 0 2px rgba(79, 172, 254, 0.2);
}

.dash {
  font-weight: bold;
  color: #2c3e50;
  font-size: 1.2rem;
}

.modal-content h3 {
  color: #2c3e50;
  margin: 1rem 0 0.5rem 0;
  font-weight: 600;
}

.modal-buttons {
  display: flex;
  justify-content: center;
  gap: 1rem;
  margin-top: 1.5rem;
}

.modal-buttons button {
  padding: 0.6rem 1.4rem;
  border: none;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: background 0.3s ease, transform 0.2s ease;
}

.modal-buttons button.confirm {
  background: linear-gradient(135deg, #4facfe, #00f2fe);
  color: white;
}

.modal-buttons button.cancel {
  background: #f44336;
  color: white;
}

.modal-buttons button:hover {
  transform: scale(1.05);
}

@media (max-width: 600px) {
  .group {
    grid-template-columns: 100px repeat(4, 45px);
    grid-template-rows: repeat(4, 45px);
  }
  .modal-content {
    width: 90%;
    padding: 1.5rem;
  }
  .game-row {
    flex-direction: column;
    gap: 0.5rem;
    align-items: stretch;
  }
  .score-inputs {
    justify-content: center;
  }
  .match-header {
    flex-direction: column;
    gap: 0.5rem;
  }
}
</style>