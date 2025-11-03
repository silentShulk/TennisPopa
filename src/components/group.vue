<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref, watch, onMounted } from 'vue';

// --- INTERFACCIA PARTITA ---
interface PlayerMatch {
  player_1: number;
  player_2: number;
  set_1_p1: number;
  set_1_p2: number;
  set_2_p1: number;
  set_2_p2: number;
  tie_p1: number;
  tie_p2: number;
}

interface Player {
  id?: number;
  name?: string;
}

// --- PROPS ---
const props = defineProps<{
  players: Player[];
  anyModalOpen: boolean;
  swapMode: boolean;
  allMatches: PlayerMatch[];
  selectedPlayer1Id: number | null;
  selectedPlayer2Id: number | null;
}>();

const emit = defineEmits<{
  (e: 'modal-opened'): void;
  (e: 'modal-closed'): void;
  (e: 'player-selected', player: { id: number | null; name: string }): void;
  (e: 'match-saved'): void;
}>();

// --- DATA ---
const isVisible = ref(false);
const selectedPlayer1 = ref<Player | null>(null);
const selectedPlayer2 = ref<Player | null>(null);
const gridScores = ref<string[][]>(Array(4).fill('').map(() => Array(4).fill('')));
const selectedRow = ref(-1);
const selectedCol = ref(-1);
const game1ScoreA = ref(0);
const game1ScoreB = ref(0);
const game2ScoreA = ref(0);
const game2ScoreB = ref(0);
const tieScoreA = ref(0);
const tieScoreB = ref(0);
const mainSwitch = ref(false);

// --- METODI ---
function clampToSeven(this: any, event: Event, property: string) {
  const input = event.target as HTMLInputElement;
  const value = parseInt(input.value) || 0;
  const clamped = Math.min(value, 7);
  (this as any)[property] = clamped;
  input.value = clamped.toString();
}

function handleClick(event: MouseEvent) {
  if (isVisible.value || props.anyModalOpen || props.swapMode) return;
  const target = event.target as HTMLElement;
  if (target.classList.contains('cell') && !target.classList.contains('diagonal')) {
    const row = parseInt(target.dataset.row!);
    const col = parseInt(target.dataset.col!);

    selectedRow.value = row;
    selectedCol.value = col;
    selectedPlayer1.value = props.players[row];
    selectedPlayer2.value = props.players[col];

    const existingScore = gridScores.value[row][col];
    if (existingScore) {
      parseExistingScore(existingScore);
    } else {
      resetScores();
    }

    isVisible.value = true;
    emit('modal-opened');
  }
}

function resetScores() {
  game1ScoreA.value = 0;
  game1ScoreB.value = 0;
  game2ScoreA.value = 0;
  game2ScoreB.value = 0;
  tieScoreA.value = 0;
  tieScoreB.value = 0;
  mainSwitch.value = false;
}

// --- NUOVA FUNZIONE: Precompila i campi dal punteggio esistente ---
function parseExistingScore(scoreStr: string) {
  resetScores();

  const lines = scoreStr.split('\n').map(l => l.trim()).filter(Boolean);
  if (lines.length === 0) return;

  let set1A = 0, set1B = 0, set2A = 0, set2B = 0, tieA = 0, tieB = 0;

  // Game 1
  if (lines[0]) {
    [set1A, set1B] = lines[0].split('-').map(s => parseInt(s.trim()) || 0);
  }
  // Game 2
  if (lines.length > 1 && lines[1]) {
    [set2A, set2B] = lines[1].split('-').map(s => parseInt(s.trim()) || 0);
  }
  // Tie-break
  if (lines.length > 2 && lines[2]) {
    [tieA, tieB] = lines[2].split('-').map(s => parseInt(s.trim()) || 0);
  }

  // Determina se è stato usato lo switch:
  // Se il giocatore 1 ha perso il primo set → probabilmente è stato swappato
  const p1WonSet1 = set1A > set1B;
  //const p1WonSet2 = set2A > set2B;

  if (!p1WonSet1 || (set2A === 0 && set2B === 0 && tieA > 0)) {
    // Probabilmente swappato: inverti i valori
    mainSwitch.value = true;
    game1ScoreA.value = set1B;
    game1ScoreB.value = set1A;
    game2ScoreA.value = set2B;
    game2ScoreB.value = set2A;
    tieScoreA.value = tieB;
    tieScoreB.value = tieA;
  } else {
    mainSwitch.value = false;
    game1ScoreA.value = set1A;
    game1ScoreB.value = set1B;
    game2ScoreA.value = set2A;
    game2ScoreB.value = set2B;
    tieScoreA.value = tieA;
    tieScoreB.value = tieB;
  }
}

function closeModal() {
  emit('modal-closed');
  isVisible.value = false;
  selectedPlayer1.value = null;
  selectedPlayer2.value = null;
  selectedRow.value = -1;
  selectedCol.value = -1;
  resetScores();
}

async function submitScores() {
  const [set1A, set1B] = mainSwitch.value
    ? [game1ScoreB.value, game1ScoreA.value]
    : [game1ScoreA.value, game1ScoreB.value];
  const [set2A, set2B] = mainSwitch.value
    ? [game2ScoreB.value, game2ScoreA.value]
    : [game2ScoreA.value, game2ScoreB.value];
  const [tieA, tieB] = mainSwitch.value
    ? [tieScoreB.value, tieScoreA.value]
    : [tieScoreA.value, tieScoreB.value];

  if (selectedRow.value !== -1 && selectedCol.value !== -1) {
    const g1 = `${set1A} - ${set1B}`;
    const g2 = `${set2A} - ${set2B}`;
    const tie = tieA > 0 || tieB > 0 ? `${tieA} - ${tieB}` : '';
    const scoreStr = [g1, g2, tie].filter(Boolean).join('\n');
    gridScores.value[selectedRow.value][selectedCol.value] = scoreStr;

    if (selectedRow.value !== selectedCol.value) {
      const revG1 = `${set1B} - ${set1A}`;
      const revG2 = `${set2B} - ${set2A}`;
      const revTie = tieA > 0 || tieB > 0 ? `${tieB} - ${tieA}` : '';
      const revScoreStr = [revG1, revG2, revTie].filter(Boolean).join('\n');
      gridScores.value[selectedCol.value][selectedRow.value] = revScoreStr;
    }
  }

  const p1_id = Number(selectedPlayer1.value?.id);
  const p2_id = Number(selectedPlayer2.value?.id);
  if (!Number.isInteger(p1_id) || !Number.isInteger(p2_id) || p1_id <= 0 || p2_id <= 0) {
    alert('Errore: ID giocatori non validi.');
    closeModal();
    return;
  }

  const payload = {
    p1Id: p1_id,
    p2Id: p2_id,
    set1: [set1A, set1B],
    set2: [set2A, set2B],
    tie: [tieA, tieB]
  };

  try {
    await invoke('save_match_result', payload);
    emit('match-saved');
  } catch (error) {
    console.error('Errore salvataggio:', error);
  } finally {
    closeModal();
  }
}

function handlePlayerClick(index: number) {
  if (props.swapMode && !props.anyModalOpen) {
    const player = props.players[index];
    emit('player-selected', {
      id: player.id ?? null,
      name: player.name ?? `Giocatore ${index + 1}`
    });
  }
}

// --- CARICAMENTO PARTITE ---
function populateGridFromMatches() {
  gridScores.value = Array(4).fill('').map(() => Array(4).fill(''));

  if (!Array.isArray(props.players) || props.players.length === 0) return;

  const idToIndex: Record<number, number> = {};
  props.players.forEach((player, index) => {
    if (player?.id) idToIndex[player.id] = index;
  });

  props.allMatches.forEach(match => {
    const p1Index = idToIndex[match.player_1];
    const p2Index = idToIndex[match.player_2];
    if (p1Index === undefined || p2Index === undefined) return;

    const g1 = `${match.set_1_p1} - ${match.set_1_p2}`;
    const g2 = `${match.set_2_p1} - ${match.set_2_p2}`;
    const tie = match.tie_p1 > 0 || match.tie_p2 > 0 ? `${match.tie_p1} - ${match.tie_p2}` : '';
    const scoreStr = [g1, g2, tie].filter(Boolean).join('\n');

    gridScores.value[p1Index][p2Index] = scoreStr;

    const revG1 = `${match.set_1_p2} - ${match.set_1_p1}`;
    const revG2 = `${match.set_2_p2} - ${match.set_2_p1}`;
    const revTie = match.tie_p1 > 0 || match.tie_p2 > 0 ? `${match.tie_p2} - ${match.tie_p1}` : '';
    const revScoreStr = [revG1, revG2, revTie].filter(Boolean).join('\n');

    gridScores.value[p2Index][p1Index] = revScoreStr;
  });
}

onMounted(populateGridFromMatches);
watch(
  () => [props.players, props.allMatches],
  populateGridFromMatches,
  { deep: true }
);
</script>

<template>
  <div class="group-container">
    <div
      class="group"
      @click="handleClick"
      :class="{ disabled: isVisible || anyModalOpen }"
    >
      <template v-for="row in 4" :key="row - 1">
        <p
          class="name"
          :class="{ 'player-selected': props.players[row - 1]?.id === props.selectedPlayer1Id || props.players[row - 1]?.id === props.selectedPlayer2Id }"
          @click.stop="handlePlayerClick(row - 1)"
        >
          {{ props.players[row - 1]?.name || 'Nome ' + row }}
        </p>
        <div
          v-for="col in 4"
          :key="col - 1"
          class="cell"
          :class="{ diagonal: (row - 1) === (col - 1), scored: gridScores[row - 1][col - 1] !== '' && (row - 1) !== (col - 1) }"
          :data-row="row - 1"
          :data-col="col - 1"
        >
          <span>{{ gridScores[row - 1][col - 1] }}</span>
        </div>
      </template>
    </div>

    <!-- Modal -->
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
.cell:hover:not(.diagonal) {
  background-color: rgba(79, 172, 254, 0.15);
  transform: scale(1.05);
}
.cell span {
  font-weight: bold;
  color: #2c3e50;
  white-space: pre-line;
  text-align: center;
}
.diagonal {
  background: linear-gradient(135deg, #4facfe, #00f2fe);
  border: none;
  box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.2);
  cursor: default;
  pointer-events: none;
}
.scored {
  background: #d4edda; /* Verde chiaro per "compilato ma modificabile" */
  border: 1px solid #c3e6cb;
  cursor: pointer;
}
.scored:hover {
  background: rgba(79, 172, 254, 0.25) !important;
  transform: scale(1.02);
}
.disabled {
  pointer-events: none;
  opacity: 0.5;
  filter: blur(1px);
}

/* Modal styles (invariati) */
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