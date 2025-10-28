<script setup>
import { invoke } from '@tauri-apps/api/core'
import { ref, watch } from 'vue'

const day = ref('')
const curtsSchedule = ref([])

const saturdayCurtsSchedule = [
  { value: 1, label: '13:00' },
  { value: 2, label: '14:00' },
  { value: 4, label: '15:00' },
  { value: 8, label: '16:00' },
  { value: 16, label: '17:00' },
  { value: 32, label: '18:00' },
  { value: 64, label: '19:00' }
]

const sundayCurtsSchedule = [
  { value: 128, label: '8:00' },
  { value: 256, label: '9:00' },
  { value: 512, label: '10:00' },
  { value: 1024, label: '11:00' },
  { value: 2048, label: '12:00' },
  { value: 4096, label: '13:00' },
  { value: 8192, label: '14:00' },
  { value: 16384, label: '15:00' },
  { value: 32768, label: '16:00' },
  { value: 65536, label: '17:00' },
  { value: 131072, label: '18:00' },
  { value: 262144, label: '19:00' }
]

const selectCourt1Schedule = ref([])
const selectCourt2Schedule = ref([])
const selectCourt3Schedule = ref([])
const selectCourt4Schedule = ref([])
const selectCourt6Schedule = ref([])
const selectCourtG1Schedule = ref([])
const selectCourtG2Schedule = ref([])

// Variables for each court, containing the total sum of selected values across both days
const court1Selections = ref(0)
const court2Selections = ref(0)
const court3Selections = ref(0)
const court4Selections = ref(0)
const court6Selections = ref(0)
const courtG1Selections = ref(0)
const courtG2Selections = ref(0)

// Temporary storage for selections per day
const saturdaySelections = ref({
  court1: [], court2: [], court3: [], court4: [], court6: [], courtG1: [], courtG2: []
})
const sundaySelections = ref({
  court1: [], court2: [], court3: [], court4: [], court6: [], courtG1: [], courtG2: []
})

// Helper function to calculate the sum of values in an array
const sumValues = (array) => {
  return array.reduce((sum, value) => sum + value, 0)
}

// Function to update the schedule based on the selected day
const selectDay = () => {
  curtsSchedule.value = day.value === '0' ? saturdayCurtsSchedule : sundayCurtsSchedule
  // Load saved selections for the selected day
  const selections = day.value === '0' ? saturdaySelections.value : sundaySelections.value
  selectCourt1Schedule.value = [...selections.court1]
  selectCourt2Schedule.value = [...selections.court2]
  selectCourt3Schedule.value = [...selections.court3]
  selectCourt4Schedule.value = [...selections.court4]
  selectCourt6Schedule.value = [...selections.court6]
  selectCourtG1Schedule.value = [...selections.courtG1]
  selectCourtG2Schedule.value = [...selections.courtG2]
}

// Function to save selections and update sums
const saveSelections = () => {
  // Update the selections for the current day
  const target = day.value === '0' ? saturdaySelections.value : sundaySelections.value
  target.court1 = [...selectCourt1Schedule.value]
  target.court2 = [...selectCourt2Schedule.value]
  target.court3 = [...selectCourt3Schedule.value]
  target.court4 = [...selectCourt4Schedule.value]
  target.court6 = [...selectCourt6Schedule.value]
  target.courtG1 = [...selectCourtG1Schedule.value]
  target.courtG2 = [...selectCourtG2Schedule.value]

  // Update the total sums for each court (Saturday + Sunday)
  court1Selections.value = sumValues(saturdaySelections.value.court1) + sumValues(sundaySelections.value.court1)
  court2Selections.value = sumValues(saturdaySelections.value.court2) + sumValues(sundaySelections.value.court2)
  court3Selections.value = sumValues(saturdaySelections.value.court3) + sumValues(sundaySelections.value.court3)
  court4Selections.value = sumValues(saturdaySelections.value.court4) + sumValues(sundaySelections.value.court4)
  court6Selections.value = sumValues(saturdaySelections.value.court6) + sumValues(sundaySelections.value.court6)
  courtG1Selections.value = sumValues(saturdaySelections.value.courtG1) + sumValues(sundaySelections.value.courtG1)
  courtG2Selections.value = sumValues(saturdaySelections.value.courtG2) + sumValues(sundaySelections.value.courtG2)
}

// Function to get all selections
const getAllSelections = () => {
  return {
    court1: court1Selections.value,
    court2: court2Selections.value,
    court3: court3Selections.value,
    court4: court4Selections.value,
    court6: court6Selections.value,
    courtG1: courtG1Selections.value,
    courtG2: courtG2Selections.value
  }
}

// Watch for changes in selections and save automatically
watch(
  [
    selectCourt1Schedule, selectCourt2Schedule, selectCourt3Schedule,
    selectCourt4Schedule, selectCourt6Schedule, selectCourtG1Schedule,
    selectCourtG2Schedule
  ],
  () => {
    if (day.value !== '') {
      saveSelections()
    }
  },
  { deep: true }
)

// Save selections to backend
const showSelections = () => {
  invoke('save_availability_court', {
    c1: court1Selections.value,
    c2: court2Selections.value,
    c3: court3Selections.value,
    c4: court4Selections.value,
    c6: court6Selections.value,
    cg1: courtG1Selections.value,
    cg2: courtG2Selections.value
  })
}
</script>

<template>
  <div class="page-container">
    <div class="page-header">
      <h1>Orari Campi</h1>
    </div>

    <div class="category-card">
      <select
        id="categoriaComboBox"
        class="category-select"
        v-model="day"
        @change="selectDay"
      >
        <option value="" disabled>Seleziona un giorno</option>
        <option value="0">Sabato</option>
        <option value="1">Domenica</option>
      </select>
      <div v-if="curtsSchedule.length > 0">
        <button class="btn btn-primary" @click="showSelections">Salva orari</button>
      </div>
      
    </div>

    <div v-if="curtsSchedule.length > 0">
      <!-- Campo 1 -->
      <div>
        <h1>Campo 1:</h1>
        <div class="courtsSchedule">
          <label v-for="option in curtsSchedule" :key="option.value" class="checkbox-label">
            <input
              type="checkbox"
              :value="option.value"
              v-model="selectCourt1Schedule"
              class="checkbox-input"
            />
            <span class="checkbox-text">{{ option.label }}</span>
          </label>
        </div>
      </div>

      <!-- Campo 2 -->
      <div>
        <h1>Campo 2:</h1>
        <div class="courtsSchedule">
          <label v-for="option in curtsSchedule" :key="option.value" class="checkbox-label">
            <input
              type="checkbox"
              :value="option.value"
              v-model="selectCourt2Schedule"
              class="checkbox-input"
            />
            <span class="checkbox-text">{{ option.label }}</span>
          </label>
        </div>
      </div>

      <!-- Campo 3 -->
      <div>
        <h1>Campo 3:</h1>
        <div class="courtsSchedule">
          <label v-for="option in curtsSchedule" :key="option.value" class="checkbox-label">
            <input
              type="checkbox"
              :value="option.value"
              v-model="selectCourt3Schedule"
              class="checkbox-input"
            />
            <span class="checkbox-text">{{ option.label }}</span>
          </label>
        </div>
      </div>

      <!-- Campo 4 -->
      <div>
        <h1>Campo 4:</h1>
        <div class="courtsSchedule">
          <label v-for="option in curtsSchedule" :key="option.value" class="checkbox-label">
            <input
              type="checkbox"
              :value="option.value"
              v-model="selectCourt4Schedule"
              class="checkbox-input"
            />
            <span class="checkbox-text">{{ option.label }}</span>
          </label>
        </div>
      </div>

      <!-- Campo 6 -->
      <div>
        <h1>Campo 6:</h1>
        <div class="courtsSchedule">
          <label v-for="option in curtsSchedule" :key="option.value" class="checkbox-label">
            <input
              type="checkbox"
              :value="option.value"
              v-model="selectCourt6Schedule"
              class="checkbox-input"
            />
            <span class="checkbox-text">{{ option.label }}</span>
          </label>
        </div>
      </div>

      <!-- Campo G1 -->
      <div>
        <h1>Campo G1:</h1>
        <div class="courtsSchedule">
          <label v-for="option in curtsSchedule" :key="option.value" class="checkbox-label">
            <input
              type="checkbox"
              :value="option.value"
              v-model="selectCourtG1Schedule"
              class="checkbox-input"
            />
            <span class="checkbox-text">{{ option.label }}</span>
          </label>
        </div>
      </div>

      <!-- Campo G2 -->
      <div>
        <h1>Campo G2:</h1>
        <div class="courtsSchedule">
          <label v-for="option in curtsSchedule" :key="option.value" class="checkbox-label">
            <input
              type="checkbox"
              :value="option.value"
              v-model="selectCourtG2Schedule"
              class="checkbox-input"
            />
            <span class="checkbox-text">{{ option.label }}</span>
          </label>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #4facfe 0%, #00acb5 100%);
  display: flex;
  flex-direction: column;
}

.page-header {
  background: rgba(255, 255, 255, 0.95);
  padding: 40px 20px;
  text-align: center;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  min-height: 100px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.page-header h1 {
  color: #2c3e50;
  margin-bottom: 10px;
  font-size: 2.5rem;
}

.page-content {
  padding: 40px;
}

.category-card {
  margin: 10px;
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 20px;
}

.category-select {
  padding: 10px;
  font-size: 1rem;
  border-radius: 6px;
  border: 1px solid #ccc;
  background: white;
  cursor: pointer;
}

.courtsSchedule {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;
  gap: 15px;
  margin-top: 10px;
  padding: 10px;
  background: rgba(255, 255, 255, 0.8);
  border-radius: 8px;
}

.checkbox-label {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  padding: 8px;
  min-height: 50px;
  width: 70px;
  border: 1px solid #e0e0e0;
  border-radius: 6px;
  transition: all 0.2s ease;
  background: white;
}

.checkbox-label:hover {
  background-color: #f8f9fa;
  border-color: #007acc;
  transform: translateY(-2px);
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
}

.checkbox-input {
  margin: 0;
  width: 16px;
  height: 16px;
  cursor: pointer;
}

.checkbox-text {
  font-size: 12px;
  color: #333;
  user-select: none;
  text-align: center;
  font-weight: 500;
  line-height: 1.2;
}

h1 {
  color: #2c3e50;
  font-size: 1.5rem;
  margin-bottom: 5px;
  padding-left: 10px;
}

.page-container > div {
  margin-bottom: 20px;
  padding: 0 20px;
}

.page-container > div:last-child {
  margin-bottom: 0;
}

.btn {
  padding: 12px 24px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 1rem;
  transition: all 0.3s ease;
}

.btn-primary {
  background: #ffc107;
  color: black;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}
</style>