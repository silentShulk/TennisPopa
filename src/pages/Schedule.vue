<script setup>
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const curtsSchedule = [
  { value: 1, label: '8:00' },
  { value: 2, label: '9:00' },
  { value: 4, label: '10:00' },
  { value: 8, label: '11:00' },
  { value: 16, label: '12:00' },
  { value: 32, label: '13:00' },
  { value: 64, label: '14:00' },
  { value: 128, label: '15:00' },
  { value: 256, label: '16:00' },
  { value: 512, label: '17:00' },
  { value: 1024, label: '18:00' },
  { value: 2048, label: '19:00' }
]

const selectCourt1Schedule = ref([])
const selectCourt2Schedule = ref([])
const selectCourt3Schedule = ref([])
const selectCourt4Schedule = ref([])
const selectCourt6Schedule = ref([])
const selectCourtG1Schedule = ref([])
const selectCourtG2Schedule = ref([])

</script>

<script>
export default {
  name: 'Schedule'
}
</script>

<template>
  <div class="page-container">
    <div class="page-header">
      <h1>Orari Campi</h1>
    </div>
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
  <div>
    <h1>Campo 6:</h1>
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
  <div>
    <h1>Campo G1:</h1>
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
  <div>
    <h1>Campo G2:</h1>
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

.page-content {
  padding: 40px;
}

.content-card {
  background: white;
  border-radius: 10px;
  padding: 30px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
  max-width: 1000px;
  margin: 0 auto;
}

.action-buttons {
  display: flex;
  gap: 15px;
  margin: 25px 0;
  flex-wrap: wrap;
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
  background: #3498db;
  color: white;
}

.btn-secondary {
  background: #95a5a6;
  color: white;
}

.btn-success {
  background: #27ae60;
  color: white;
}

.btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-top: 30px;
}

.stat-card {
  background: #f8f9fa;
  padding: 25px;
  border-radius: 8px;
  text-align: center;
  border-top: 4px solid #e74c3c;
}

.stat-card h3 {
  color: #7f8c8d;
  margin-bottom: 10px;
  font-size: 1rem;
}

.stat-number {
  font-size: 2.5rem;
  font-weight: bold;
  color: #2c3e50;
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
  justify-content: flex-start;
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

/* Stile per i titoli dei campi */
h1 {
  color: #2c3e50;
  font-size: 1.5rem;
  margin-bottom: 5px;
  padding-left: 10px;
}

/* Aggiungi un po' di padding al container principale */
.page-container > div {
  margin-bottom: 25px;
  padding: 0 20px;
}
</style>