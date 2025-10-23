<template>
  <div class="group-container">
    <div
      class="group"
      @click="handleClick"
      :class="{ disabled: isVisible || anyModalOpen }"
    >
      <!-- Row 1 -->
      <p class="name">Nome 1</p>
      <div class="cell filled"></div>
      <div class="cell"></div>
      <div class="cell"></div>
      <div class="cell"></div>
      <!-- Row 2 -->
      <p class="name">Nome 2</p>
      <div class="cell"></div>
      <div class="cell filled"></div>
      <div class="cell"></div>
      <div class="cell"></div>
      <!-- Row 3 -->
      <p class="name">Nome 3</p>
      <div class="cell"></div>
      <div class="cell"></div>
      <div class="cell filled"></div>
      <div class="cell"></div>
      <!-- Row 4 -->
      <p class="name">Nome 4</p>
      <div class="cell"></div>
      <div class="cell"></div>
      <div class="cell"></div>
      <div class="cell filled"></div>
    </div>
    <!-- Modal -->
    <div v-if="isVisible" class="modal" @click="handleModalClick">
      <div class="modal-content">
        <h3>Inserisci i punteggi</h3>
        <label for="score1">Punteggio giocatore 1</label>
        <input id="score1" type="number" v-model="score1" min="0" max="7" />
        <label for="score2">Punteggio giocatore 2</label>
        <input id="score2" type="number" v-model="score2" min="0" max="7" />
        <div class="modal-buttons">
          <button class="confirm" @click.stop="submitScores">OK</button>
          <button class="cancel" @click.stop="closeModal">Annulla</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'Group',
  props: {
    anyModalOpen: {
      type: Boolean,
      default: false
    }
  },
  data() {
    return {
      isVisible: false,
      score1: '',
      score2: ''
    };
  },
  methods: {
    handleClick(event) {
      // Ignore clicks if modal is open or any modal is open
      if (this.isVisible || this.anyModalOpen) return;
      // Only open modal if a non-filled cell was clicked
      if (event.target.classList.contains('cell') && !event.target.classList.contains('filled')) {
        this.isVisible = true;
        this.$emit('modal-opened');
      }
    },
    handleModalClick(event) {
      if (event.target.classList.contains('modal')) {
        this.closeModal();
      }
    },
    closeModal() {
      this.$emit('modal-closed');
      this.isVisible = false;
      this.score1 = '';
      this.score2 = '';
    },
    submitScores() {
      console.log('Score 1:', this.score1, 'Score 2:', this.score2);
      this.closeModal();
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
}

.cell {
  border: 1px solid rgba(0, 0, 0, 0.1);
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: white;
  cursor: pointer;
  transition: background-color 0.25s ease, transform 0.15s ease;
}

.cell:hover {
  background-color: rgba(79, 172, 254, 0.15);
  transform: scale(1.05);
}

.filled {
  background: linear-gradient(135deg, #4facfe, #00f2fe);
  border: none;
  box-shadow: inset 0 0 6px rgba(0, 0, 0, 0.2);
  cursor: default;
  /* Non-clickable */
  pointer-events: none;
  /* Disable clicking */
}

/* Disable group interaction when modal is open */
.disabled {
  pointer-events: none;
  opacity: 0.5;
  filter: blur(1px);
}

/* ===== Modal Styles ===== */
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
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  text-align: center;
}

.modal-content h3 {
  color: #2c3e50;
  margin-bottom: 1rem;
  font-weight: 600;
}

.modal-content label {
  display: block;
  margin-top: 10px;
  color: #34495e;
  font-weight: 600;
  text-align: left;
  font-size: 0.9rem;
}

.modal-content input {
  width: 100%;
  padding: 10px;
  margin-top: 4px;
  border-radius: 8px;
  border: 1px solid #ccc;
  font-size: 1rem;
  outline: none;
  transition: border-color 0.3s ease, box-shadow 0.3s ease;
}

.modal-content input:focus {
  border-color: #4facfe;
  box-shadow: 0 0 0 3px rgba(79, 172, 254, 0.15);
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

/* ===== Responsive ===== */
@media (max-width: 600px) {
  .group {
    grid-template-columns: 100px repeat(4, 45px);
    grid-template-rows: repeat(4, 45px);
  }

  .modal-content {
    width: 90%;
    padding: 1.5rem;
  }
}
</style>