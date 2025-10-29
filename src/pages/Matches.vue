<script setup>
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';

// Stato reattivo per memorizzare le partite
const matches = ref([]);

// Input di ricerca per il nome del giocatore
const searchTerm = ref('');

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

// Computed per le partite filtrate
const filteredMatches = computed(() => {
  if (!searchTerm.value.trim()) {
    return matches.value;
  }
  const term = searchTerm.value.toLowerCase();
  return matches.value.filter(match => 
    match.player_1.name.toLowerCase().includes(term) ||
    match.player_2.name.toLowerCase().includes(term)
  );
});

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
    5: 'Campo G1',
    6: 'Campo G2'
    // Aggiungi altri campi se necessario
  };
  return courtNames[courtNumber] || `Campo ${courtNumber}`;
};

// Funzione per formattare l'orario basato sul valore di CourtSlots (potenza di 2)
const formatTime = (slotValue) => {
  switch(slotValue) {
    case "SATH13":
      return "Sabato, 13:00";
      break;
    case "SATH14":
      return "Sabato, 14:00";
      break;
    case "SATH15":
      return "Sabato, 15:00";
      break;
    case "SATH16":
      return "Sabato, 16:00";
      break;
    case "SATH17":
      return "Sabato, 17:00";
      break;
    case "SATH18":
      return "Sabato, 18:00";
      break;
    case "SATH19":
      return "Sabato, 19:00";
      break;
    case "SUNH8":
      return "Domenica, 8:00";
      break;
    case "SUNH9":
      return "Domenica, 9:00";
      break;
    case "SUNH10":
      return "Domenica, 10:00";
      break;
    case "SUNH11":
      return "Domenica, 11:00";
      break;
    case "SUNH12":
      return "Domenica, 12:00";
      break;
    case "SUNH13":
      return "Domenica, 13:00";
      break;
    case "SUNH14":
      return "Domenica, 14:00";
      break;
    case "SUNH15":
      return "Domenica, 15:00";
      break;
    case "SUNH16":
      return "Domenica, 16:00";
      break;
    case "SUNH17":
      return "Domenica, 17:00";
      break;
    case "SUNH18":
      return "Domenica, 18:00";
      break;
    case "SUNH19":
      return "Domenica, 19:00";
      break;
  }
  return 'Orario non valido';
};
</script>

<template>
  <div class="page-container">
    <div class="page-header">
      <h1>Partite</h1>
    </div>
    <button @click="createMatches">Crea partite</button>
    
    <div class="matches-container">
      <!-- Input di ricerca -->
      <div class="search-section">
        <input 
          v-model="searchTerm" 
          type="text" 
          placeholder="Cerca per nome del giocatore..." 
          class="search-input"
          @keyup.enter="fetchMatches"
        />
      </div>
      
      <div v-if="filteredMatches.length === 0" class="no-matches">
        <span v-if="!searchTerm.trim()">
          Nessuna partita programmata.
        </span>
        <span v-else>
          Nessuna partita trovata per "{{ searchTerm }}".
        </span>
      </div>
      <div v-else class="matches-list">
        <div v-for="(match, index) in filteredMatches" :key="index" class="match-card">
          <div class="match-info">
            <h3>{{ match.player_1.name }} vs {{ match.player_2.name }}</h3>
            <p><strong>Orario:</strong> {{ formatTime(match.scheduled_time) }}</p>
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

.search-section {
  margin-bottom: 20px;
  text-align: center;
}

.search-input {
  padding: 10px 15px;
  width: 300px;
  max-width: 100%;
  border: 1px solid #ddd;
  border-radius: 5px;
  font-size: 1rem;
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
  
  .search-input {
    width: 100%;
  }
}
</style>