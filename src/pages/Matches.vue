<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';

// Stato reattivo per memorizzare le partite
const matches = ref([]);

// Funzione per ottenere le partite dal backend
async function fetchMatches() {
  try {
    const result = await invoke('get_all_scheduled_matches', {});
    matches.value = result;
  } catch (error) {
    console.error('Errore nel recupero delle partite:', error);
  }
}

// Funzione per creare nuove partite
async function createMatches() {
  try {
    await invoke('schedule_matches_for_all_players', {});
    // Dopo aver creato le partite, aggiorna la lista
    await fetchMatches();
  } catch (error) {
    console.error('Errore nella creazione delle partite:', error);
  }
}

// Chiama fetchMatches quando il componente viene montato
onMounted(() => {
  fetchMatches();
});

// Funzione per formattare il nome del campo
const getCourtName = (courtNumber) => {
  const courtNames = {
    0: 'Campo 1',
    1: 'Campo 2',
    2: 'Campo 3',
    3: 'Campo 4',
    4: 'Campo 6',
    5: 'Campo g1',
    6: 'Campo g2'
    // Aggiungi altri campi se necessario
  };
  return courtNames[courtNumber] || `Campo ${courtNumber}`;
};

// Funzione per formattare l'orario
const formatTime = (time) => {
  const date = new Date(time);
  return date.toLocaleString('it-IT', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
};
</script>

<template>
  <div class="page-container">
    <div class="page-header">
      <h1>Partite</h1>
    </div>
    <button @click="createMatches">Crea partite</button>
    
    <div class="matches-container">
      <div v-if="matches.length === 0" class="no-matches">
        Nessuna partita programmata.
      </div>
      <div v-else class="matches-list">
        <div v-for="(match, index) in matches" :key="index" class="match-card">
          <div class="match-info">
            <h3>{{ match.player1 }} vs {{ match.player2 }}</h3>
            <p><strong>Orario:</strong> {{ formatTime(match.time) }}</p>
            <p><strong>Campo:</strong> {{ getCourtName(match.court) }}</p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #4facfe 0%, #00acb5 100%);
}

.page-header {
  background: rgba(255, 255, 255, 0.95);
  padding: 20px;
  text-align: center;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.page-header h1 {
  color: #2c3e50;
  margin-bottom: 10px;
  font-size: 2.5rem;
}

.matches-container {
  padding: 40px;
  max-width: 1000px;
  margin: 0 auto;
}

.no-matches {
  text-align: center;
  font-size: 1.2rem;
  color: #2c3e50;
}

.matches-list {
  display: grid;
  gap: 20px;
}

.match-card {
  background: white;
  border-radius: 10px;
  padding: 20px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
  border-left: 4px solid #27ae60;
}

.match-info h3 {
  margin: 0 0 10px;
  color: #2c3e50;
  font-size: 1.5rem;
}

.match-info p {
  margin: 5px 0;
  color: #34495e;
}

button {
  display: block;
  margin: 20px auto;
  padding: 10px 20px;
  background: #27ae60;
  color: white;
  border: none;
  border-radius: 5px;
  font-size: 1rem;
  cursor: pointer;
}

button:hover {
  background: #219653;
}

@media (max-width: 768px) {
  .matches-container {
    padding: 20px;
  }
}
</style>