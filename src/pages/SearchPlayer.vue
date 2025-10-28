<template>
  <div class="search-container">
    <div class="search-input-wrapper">
      <input
        type="text"
        v-model="searchText"
        @input="handleSearch"
        placeholder="Cerca giocatore per nome..."
        class="search-input"
      />
      <div v-if="loading" class="loading">Ricerca in corso...</div>
    </div>

    <div v-if="displayedResults.length > 0" class="results-container">
      <div 
        v-for="player in displayedResults" 
        :key="player.id"
        class="player-item"
        @click="selectPlayer(player)"
      >
        <div class="player-name">{{ player.name }}</div>
        <div class="player-details">
          <span class="player-category">{{ getCategoryText(player.category) }}</span>
          <span class="player-size">{{ getSizeText(player.size) }}</span>
          <span class="player-email">{{ player.email }}</span>
        </div>
      </div>
    </div>

    <div v-if="searchText && searchResults.length === 0 && !loading" class="no-results">
      Nessun giocatore trovato per "{{ searchText }}"
    </div>

    <div v-if="selectedPlayer" class="player-details-card">
      <h3>Dettagli Giocatore</h3>

      <div div v-if="!isEditing" class="view-mode">
        <div class="detail-item">
          <strong>Nome: </strong> {{ selectedPlayer.name }}
        </div>

        <div class="detail-item">
          <strong>Email: </strong> {{ selectedPlayer.email }}
        </div>

        <div class="detail-item">
          <strong>Telefono: </strong> {{ selectedPlayer.phone_number }}
        </div>

        <div class="detail-item">
          <strong>Categoria: </strong> 
          <span class="value-badge category-badge">{{ getCategoryText(selectedPlayer.category) }}</span>
        </div>

        <div class="detail-item">
          <strong>Disponibilità: </strong> {{ selectedPlayer.availability }}
        </div>

        <div class="detail-item">
          <strong>Taglia: </strong> 
          <span class="value-badge size-badge">{{ getSizeText(selectedPlayer.size) }}</span>
        </div>

        <div class="actions">
          <button @click="enableEditing" class="edit-btn">Modifica</button>
          <button @click="selectedPlayer = null" class="close-btn">Chiudi</button>
        </div>
      </div>

      <div v-else class="edit-mode">

        <div class="detail-item">
          <strong>Nome: </strong>
          <input v-model="editedPlayer.name" type="text" class="edit-input" />
        </div>

        <div class="detail-item">
          <strong>Email: </strong> {{ selectedPlayer.email }} <em> (non modificabile)</em>
        </div>

        <div class="detail-item">
          <strong>Telefono: </strong>
          <input v-model="editedPlayer.phone_number" type="text" class="edit-input" />
        </div>

        <div class="detail-item">
          <strong>Categoria: </strong>
          <select v-model="editedPlayer.category" class="edit-select">
            <option :value="0">Sconosciuta</option>
            <option :value="1">E</option>
            <option :value="2">D</option>
            <option :value="3">C</option>
            <option :value="4">B1</option>
            <option :value="5">B2</option>
            <option :value="6">A1</option>
            <option :value="7">A2</option>
          </select>
        </div>

        <div class="detail-item availability-item">
          <strong>Disponibilità: </strong>
          <div class="availability-container">
            <div class="availability-checkboxes">
              <label v-for="option in availabilityOptions" :key="option.value" class="checkbox-label">
                <input 
                  type="checkbox" 
                  :value="option.value" 
                  v-model="selectedAvailability" 
                  class="checkbox-input"
                />
                <span class="checkbox-text">{{ option.label }}</span>
              </label>
            </div>
          </div>
        </div>

        <div class="detail-item">
          <strong>Taglia: </strong>
          <select v-model="editedPlayer.size" class="edit-select">
            <option :value="0">XS</option>
            <option :value="1">S</option>
            <option :value="2">M</option>
            <option :value="3">L</option>
            <option :value="4">XL</option>
          </select>
        </div>
        
        <div class="actions">
          <button @click="saveChanges" class="save-btn">Salva Modifiche</button>
          <button @click="cancelEditing" class="cancel-btn">Annulla</button>
        </div>

      </div>

    </div>

  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const searchText = ref('')
const searchResults = ref([])
const selectedPlayer = ref(null)
const loading = ref(false)
const maxResults = ref(10)
const editedPlayer = ref(null)
const isEditing = ref(false)

const availabilityOptions = [
  { value: 1, label: 'Sabato Pomeriggio' },
  { value: 2, label: 'Domenica Mattina' }, 
  { value: 4, label: 'Domenica Pomeriggio' }
]

const selectedAvailability = ref([])

let searchTimeout = null

const displayedResults = computed(() => {
  return searchResults.value.slice(0, maxResults.value)
})


const getSizeText = (sizeNumber) => {
  switch (sizeNumber) {
    case 0: return 'XS'
    case 1: return 'S'
    case 2: return 'M'
    case 3: return 'L'
    case 4: return 'XL'
    default: return 'Unknown'
  }
}

const getCategoryText = (categoryNumber) => {
  switch (categoryNumber) {
    case 0: return 'Sconosciuta'
    case 1: return 'E' 
    case 2: return 'D'
    case 3: return 'C'
    case 4: return 'B1'
    case 5: return 'B2' 
    case 6: return 'A1'
    case 7: return 'A2'
    default: return 'Unknown'
  }
}

// Non so come farlo funzionare :3
const getAvailabilityText = (availabilityValue) => {
  switch (availabilityValue) {
    case 0b000: return 'Nessuna'
    case 0b001: return 'Sabato Mattina' 
    case 0b010: return 'Domenica mattina'
    case 0b011: return 'Sabato mattine | Domenica Mattina'
    case 0b100: return 'Domenica pomeriggio'
    case 0b101: return 'Sabato mattine | Domenica pomeriggio' 
    case 0b110: return 'Domenica mattina | Domenica pomeriggio'
    case "SatAft | SunMon | SunAft": return 'Sabato mattina | Domenica mattina | Domenica pomeriggio'
    default: return 'Unknown'
  }
}

const calculateAvailabilityValue = () => {
  if (!selectedAvailability.value || !Array.isArray(selectedAvailability.value)) return 0
  return selectedAvailability.value.reduce((total, currentValue) => total + currentValue, 0)
}

const getAvailabilityBinary = () => {
  const value = calculateAvailabilityValue()
  return '0b' + value.toString(2).padStart(4, '0')
}


const handleSearch = () => {
  clearTimeout(searchTimeout)
  
  if (searchText.value.trim() === '') {
    searchResults.value = []
    return
  }

  loading.value = true
  
  searchTimeout = setTimeout(async () => {
    try {
      const results = await invoke('find_player', {
        name: searchText.value
      })
      
      searchResults.value = results
    } catch (error) {
      console.error('Errore nella ricerca:', error)
      searchResults.value = []
    } finally {
      loading.value = false
    }
  }, 300)
}

const selectPlayer = (player) => {
  selectedPlayer.value = player
  isEditing.value = false
}

const enableEditing = () => {
  isEditing.value = true
  editedPlayer.value = { ...selectedPlayer.value }

  selectedAvailability.value = []
  availabilityOptions.forEach(option => {
    if ((editedPlayer.value.availability & option.value) !== 0) {
      selectedAvailability.value.push(option.value)
    }
  })
}

const cancelEditing = () => {
  isEditing.value = false
  editedPlayer.value = null
}

const saveChanges = async () => {
  try {
    if (!editedPlayer.value.name || editedPlayer.value.name.trim() === '') {
      alert('Il nome non può essere vuoto')
      return
    }

    editedPlayer.value.availability = calculateAvailabilityValue()
    editedPlayer.value.email = selectedPlayer.value.email
    
    await invoke('update_spec_player', { updatePlayer: editedPlayer.value })

    selectedPlayer.value = { ...editedPlayer.value }
    isEditing.value = false
    editedPlayer.value = null
    selectedAvailability.value = []
    
    console.log('Player aggiornato con successo')
  } catch (error) {
    handleSaveError(error)
  }

  // DEBUG _______________________________________________
  const handleSaveError = (error) => {
  let errorMessage = 'Errore nel salvataggio delle modifiche'
  if (typeof error === 'string') {
    if (error.includes('missing required key')) {
      errorMessage = 'Errore di configurazione: parametri mancanti per aggiornare il player.'
    } else if (error.includes('invalid args')) {
      errorMessage = 'Errore di configurazione: parametri non validi per aggiornare il player.'
    } else {
      errorMessage = `Errore: ${error}`
    }
  } else if (error && error.message) {
    const errMsg = error.message.toLowerCase()
    
    if (errMsg.includes('missing required key') || errMsg.includes('invalid args')) {
      errorMessage = 'Errore di configurazione: parametri non validi per aggiornare il player.'
    } else if (errMsg.includes('network') || errMsg.includes('fetch')) {
      errorMessage = 'Errore di connessione. Verifica la tua connessione internet.'
    } else if (errMsg.includes('timeout')) {
      errorMessage = 'Timeout della connessione. Riprova.'
    } else {
      errorMessage = `Errore: ${error.message}`
    }
  }
  
  console.error('Errore dettagliato nel salvataggio:', error)
}
}
</script>

<style scoped>
.availability-item {
  align-items: flex-start;
}

.availability-container {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-left: 8px;
  flex: 1;
}

/* Checkbox allineate in colonna */
.availability-checkboxes {
  display: flex;
  flex-direction: column;
  gap: 8px;
  width: 100%;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  padding: 4px 0;
  min-height: 24px;
}

.checkbox-input {
  margin: 0;
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.checkbox-text {
  font-size: 14px;
  color: #333;
  user-select: none;
}

.availability-info {
  margin-top: 8px;
  font-size: 0.85em;
  color: #666;
  font-style: italic;
  padding: 8px 12px;
  background-color: #f5f5f5;
  border-radius: 4px;
  border-left: 3px solid #007acc;
}

/* Altri stili invariati */
.search-container {
  max-width: 600px;
  margin: 0 auto;
  padding: 20px;
}

.search-input-wrapper {
  position: relative;
  margin-bottom: 20px;
}

.search-input {
  width: 100%;
  padding: 12px 16px;
  border: 2px solid #e1e5e9;
  border-radius: 8px;
  font-size: 16px;
}

.search-input:focus {
  outline: none;
  border-color: #007acc;
}

.loading {
  position: absolute;
  right: 10px;
  top: 50%;
  transform: translateY(-50%);
  color: #666;
  font-size: 14px;
}

/* Risultati */
.results-container {
  border: 1px solid #e1e5e9;
  border-radius: 8px;
  max-height: 300px;
  overflow-y: auto;
}

.player-item {
  padding: 12px 16px;
  border-bottom: 1px solid #f0f0f0;
  cursor: pointer;
  transition: background-color 0.2s;
}

.player-item:hover {
  background-color: #f8f9fa;
}

.player-item:last-child {
  border-bottom: none;
}

.player-name {
  font-weight: bold;
  margin-bottom: 4px;
}

.player-details {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 14px;
  color: #666;
}

.player-category {
  background: #4CAF50;
  color: white;
  padding: 2px 8px;
  border-radius: 12px;
  font-weight: bold;
  font-size: 12px;
}

.player-size {
  background: #2196F3;
  color: white;
  padding: 2px 8px;
  border-radius: 12px;
  font-weight: bold;
  font-size: 12px;
}

.player-availability {
  background: #FF9800;
  color: white;
  padding: 2px 8px;
  border-radius: 12px;
  font-weight: bold;
  font-size: 12px;
}

.more-results {
  padding: 10px 16px;
  text-align: center;
  color: #666;
  font-style: italic;
  background: #fafafa;
  border-top: 1px solid #f0f0f0;
}

/* Nessun risultato */
.no-results {
  padding: 20px;
  text-align: center;
  color: #666;
  background: #f8f9fa;
  border-radius: 8px;
}

/* Dettagli giocatore */
.player-details-card {
  margin-top: 20px;
  padding: 20px;
  border: 1px solid #e1e5e9;
  border-radius: 8px;
  background: #f8f9fa;
}

.player-details-card h3 {
  margin-top: 0;
  margin-bottom: 16px;
}

.detail-item {
  display: flex;
  align-items: center;
  margin-bottom: 8px;
  padding: 8px 0;
  border-bottom: 1px solid #e9ecef;
}

.detail-item:last-child {
  border-bottom: none;
  margin-bottom: 16px;
}

.value-badge {
  margin-left: 8px;
  padding: 4px 12px;
  border-radius: 12px;
  font-weight: bold;
  color: white;
}

.category-badge {
  background: #4CAF50;
}

.size-badge {
  background: #2196F3;
}

/* Stili per la modifica */
.edit-input, .edit-select {
  margin-left: 8px;
  padding: 4px 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-size: 14px;
}

.edit-input {
  flex: 1;
  max-width: 200px;
}

.edit-select {
  background: white;
}

/* Pulsanti */
.actions {
  display: flex;
  gap: 10px;
  margin-top: 20px;
}

.edit-btn, .save-btn, .cancel-btn, .close-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.edit-btn {
  background: #ffc107;
  color: black;
}

.edit-btn:hover {
  background: #e0a800;
}

.save-btn {
  background: #28a745;
  color: white;
}

.save-btn:hover {
  background: #218838;
}

.cancel-btn {
  background: #6c757d;
  color: white;
}

.cancel-btn:hover {
  background: #5a6268;
}

.close-btn {
  background: #007acc;
  color: white;
}

.close-btn:hover {
  background: #005a9e;
}

/* Stili per la modalità modifica */
.view-mode, .edit-mode {
  width: 100%;
}
</style>