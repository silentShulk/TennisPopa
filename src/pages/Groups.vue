<script setup>
import { invoke } from '@tauri-apps/api/core';
import Group from '../components/Group.vue';
import { ref, watch, onUnmounted, computed } from 'vue';
import { useRoute } from 'vue-router';
import { save } from '@tauri-apps/plugin-dialog';


const route = useRoute();
const category = ref('');
const groups = ref([]);
const anyModalOpen = ref(false);
const swapMode = ref(false); // Stato della modalità swap
const selectedPlayer1Id = ref(null); // ID del primo giocatore selezionato
const selectedPlayer2Id = ref(null); // ID del secondo giocatore selezionato

// Called on category change
async function selectedCategory() {
  if (!category.value) {
    groups.value = [];
    return;
  }
  try {
    const categoryInt = parseInt(category.value);
    const result = await invoke('groups_in_category', { category: categoryInt });
    groups.value = Array.isArray(result) ? result : [];
    console.log('Loaded groups:', groups.value);
    // Resetta la modalità swap e le selezioni quando cambia categoria
    swapMode.value = false;
    selectedPlayer1Id.value = null;
    selectedPlayer2Id.value = null;
  } catch (error) {
    alert('Errore durante la chiamata a groups_in_category: ' + error);
    console.error('Errore durante la chiamata a groups_in_category:', error);
    groups.value = [];
  }
}

async function create_excel() {
  try {
    const filePath = await save({
      filters: [{ name: 'File Excel', extensions: ['xlsx'] }],
      defaultPath: 'gruppi_torneo.xlsx',
    });
    if (filePath) {
      await invoke('create_excel_group', { path: filePath });
      alert('File Excel creato con successo!');
    } else {
      console.log("Operazione annullata dall'utente.");
    }
  } catch (error) {
    alert('Errore durante la creazione del file Excel: ' + error);
    console.error('Errore durante la creazione del file Excel:', error);
  }
}

function onModalOpened() {
  anyModalOpen.value = true;
}

function onModalClosed() {
  anyModalOpen.value = false;
}

// Computed: only complete groups (≥ 4 players)
const completeGroups = computed(() => {
  return groups.value.filter(group => Array.isArray(group.players) && group.players.length >= 4);
});

// Computed: players from incomplete groups (< 4 players)
const incompletePlayers = computed(() => {
  return groups.value
    .filter(group => Array.isArray(group.players) && group.players.length < 4)
    .flatMap(group => group.players.map(p => ({
      id: p.id ?? null,
      name: p.name ?? p
    }))); // Include ID e nome
});

// Watch modal state
watch(anyModalOpen, (isOpen) => {
  const mainContent = document.querySelector('.main-content');
  if (mainContent) {
    if (isOpen) {
      mainContent.classList.add('modal-open');
    } else {
      mainContent.classList.remove('modal-open');
    }
  }
  const body = document.body;
  if (isOpen) {
    body.style.overflow = 'hidden';
  } else {
    body.style.overflow = '';
  }
});

// Watch for route changes to close modal early
watch(() => route.path, (newPath) => {
  if (newPath !== '/Groups' && anyModalOpen.value) {
    anyModalOpen.value = false;
  }
});

// Watch per resettare swap mode al cambio categoria
watch(category, () => {
  swapMode.value = false;
  selectedPlayer1Id.value = null;
  selectedPlayer2Id.value = null;
});

// Cleanup on unmount
onUnmounted(() => {
  const mainContent = document.querySelector('.main-content');
  if (mainContent) {
    mainContent.classList.remove('modal-open');
  }
  document.body.style.overflow = '';
  anyModalOpen.value = false;
  swapMode.value = false;
  selectedPlayer1Id.value = null;
  selectedPlayer2Id.value = null;
});

async function create_groups() {
  try {
    await invoke('create_groups', {});
  } catch (error) {
    console.error('Errore durante la creazione dei gruppi:', error);
  }
}

function toggleSwapMode() {
  swapMode.value = !swapMode.value;
  // Resetta selezioni quando si attiva/disattiva la modalità
  selectedPlayer1Id.value = null;
  selectedPlayer2Id.value = null;
}

async function handlePlayerSelection(player) {
  if (!swapMode.value || anyModalOpen.value) return;

  if (!selectedPlayer1Id.value) {
    selectedPlayer1Id.value = player.id;
    console.log('Primo giocatore selezionato:', player.name, player.id);
  } else if (!selectedPlayer2Id.value && player.id !== selectedPlayer1Id.value) {
    selectedPlayer2Id.value = player.id;
    console.log('Secondo giocatore selezionato:', player.name, player.id);
    // Esegui lo swap
    try {
      await invoke('swap_group_for_players', {
        p1Id: selectedPlayer1Id.value,
        p2Id: selectedPlayer2Id.value
      });
      alert('Giocatori swappati con successo!');
      // Ricarica i gruppi per riflettere lo swap
      await selectedCategory();
    } catch (error) {
      alert('Errore durante lo swap: ' + error);
      console.error('Errore durante lo swap:', error);
    } finally {
      // Resetta la modalità swap e le selezioni
      swapMode.value = false;
      selectedPlayer1Id.value = null;
      selectedPlayer2Id.value = null;
    }
  }
}

</script>

<template>
  <div class="page-container">
    <div class="page-header">
      <h1>Gironi Torneo</h1>
    </div>

    <div class="category-section">
      <div class="category-card">
        <label for="categoriaComboBox" class="category-label">Seleziona Categoria:</label>
        <select id="categoriaComboBox" class="category-select" v-model="category" @change="selectedCategory">
          <option value="" selected disabled>Seleziona una categoria</option>
          <option :value="6">A1</option>
          <option :value="7">A2</option>
          <option :value="4">B1</option>
          <option :value="5">B2</option>
          <option :value="3">C</option>
          <option :value="2">D</option>
          <option :value="1">E</option>
        </select>
        <button @click="create_groups">Crea gironi</button>
        <button @click="create_excel">Crea excel</button>
        <button :class="{ 'swap-active': swapMode }" @click="toggleSwapMode">
          {{ swapMode ? 'Disattiva Swap' : 'Attiva Swap' }}
        </button>
      </div>
    </div>

    <div class="groups-section">
      <div class="groups-grid">
        <div v-for="(group, index) in completeGroups" :key="index" class="group-wrapper">
          <Group
            :players="group.players"
            :any-modal-open="anyModalOpen"
            :swap-mode="swapMode"
            @modal-opened="onModalOpened"
            @modal-closed="onModalClosed"
            @player-selected="handlePlayerSelection"
          />
        </div>
        <div v-if="category && completeGroups.length === 0" class="no-groups-message">
          Nessun girone completo disponibile per questa categoria.
        </div>
      </div>
    </div>

    <!-- Incomplete groups players list -->
    <div v-if="incompletePlayers.length > 0" class="incomplete-section">
      <h2>Giocatori senza gruppo completo</h2>
      <ul>
        <li
          v-for="(player, index) in incompletePlayers"
          :key="index"
          :class="{ 'player-selected': player.id === selectedPlayer1Id || player.id === selectedPlayer2Id }"
          @click="handlePlayerSelection(player)"
          class="player-item"
        >
          {{ player.name }}
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>
.no-groups-message {
  grid-column: 1 / -1;
  text-align: center;
  padding: 2rem;
  color: #7f8c8d;
  font-size: 1.1rem;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 10px;
  backdrop-filter: blur(5px);
}

:global(.main-content.modal-open) {
  filter: blur(2px);
}

.page-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #4facfe 0%, #00acb5 100%);
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  padding-bottom: 2rem;
}

.page-header {
  background: rgba(255, 255, 255, 0.95);
  padding: 2rem;
  text-align: center;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(10px);
}

.page-header h1 {
  color: #2c3e50;
  margin: 0 0 0.5rem 0;
  font-size: 2.8rem;
  font-weight: 300;
  letter-spacing: -0.5px;
}

.page-header p {
  color: #7f8c8d;
  font-size: 1.1rem;
  margin: 0;
}

.category-section {
  padding: 2rem;
  display: flex;
  justify-content: center;
}

.category-card {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 16px;
  padding: 1.5rem 2rem;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(10px);
  border: 1px solid rgba(255, 255, 255, 0.2);
  display: flex;
  align-items: center;
  gap: 1rem;
  min-width: 300px;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.category-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
}

.category-label {
  color: #2c3e50;
  font-size: 1.2rem;
  font-weight: 600;
  white-space: nowrap;
}

.category-select {
  flex: 1;
  padding: 0.75rem 1rem;
  border: 2px solid #e9ecef;
  border-radius: 10px;
  font-size: 1rem;
  background: white;
  color: #2c3e50;
  cursor: pointer;
  transition: border-color 0.3s ease, box-shadow 0.3s ease;
  appearance: none;
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e");
  background-position: right 0.5rem center;
  background-repeat: no-repeat;
  background-size: 1.5em 1.5em;
  padding-right: 2.5rem;
}

.category-select:hover {
  border-color: #4facfe;
  box-shadow: 0 0 0 3px rgba(79, 172, 254, 0.1);
}

.category-select:focus {
  outline: none;
  border-color: #4facfe;
  box-shadow: 0 0 0 3px rgba(79, 172, 254, 0.2);
}

.groups-section {
  padding: 0 2rem 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.groups-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 2rem;
  padding: 1rem 0;
}

.group-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
}

.group-wrapper:hover {
  transform: translateY(-4px);
}

/* New incomplete players list styling */
.incomplete-section {
  max-width: 800px;
  margin: 2rem auto;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 12px;
  padding: 1.5rem 2rem;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  backdrop-filter: blur(10px);
}

.incomplete-section h2 {
  margin-bottom: 1rem;
  text-align: center;
  color: #2c3e50;
  font-weight: 600;
}

.incomplete-section ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.incomplete-section li {
  padding: 0.5rem 0;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  text-align: center;
  color: #34495e;
  font-size: 1.05rem;
}

.incomplete-section li:last-child {
  border-bottom: none;
}

@media (max-width: 768px) {
  .category-section {
    padding: 1rem;
  }

  .category-card {
    min-width: auto;
    flex-direction: column;
    gap: 0.75rem;
    text-align: center;
  }

  .groups-grid {
    grid-template-columns: 1fr;
    gap: 1.5rem;
    padding: 0.5rem;
  }

  .page-header h1 {
    font-size: 2.2rem;
  }
}

button {
  background: linear-gradient(135deg, #4facfe 0%, #00acb5 100%);
  color: white;
  font-size: 1rem;
  font-weight: 600;
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  transition: transform 0.3s ease, box-shadow 0.3s ease, background 0.3s ease;
}

button:hover {
  background: linear-gradient(135deg, #66b6ff 0%, #00c4cc 100%);
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.2);
  transform: translateY(-2px);
}

button:active {
  transform: translateY(0);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

button:focus {
  outline: none;
  box-shadow: 0 0 0 3px rgba(79, 172, 254, 0.3);
}

.swap-active {
  background: linear-gradient(135deg, #ff6b6b, #ff8e53);
}

.swap-active:hover {
  background: linear-gradient(135deg, #ff8787, #ffa270);
}

/* Stile per i giocatori selezionati */
.player-selected {
  background: rgba(79, 172, 254, 0.3);
  border-radius: 5px;
  padding: 0.5rem;
  cursor: pointer;
}

/* Stile per gli elementi della lista dei giocatori */
.player-item {
  cursor: pointer;
  transition: background 0.3s ease;
}

.player-item:hover {
  background: rgba(79, 172, 254, 0.15);
}

@media (max-width: 480px) {
  .groups-section {
    padding: 0 1rem 1rem;
  }
}
</style>