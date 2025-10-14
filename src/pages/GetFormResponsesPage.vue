<script setup>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

const consoleOutput = ref([]);

// Clears console and adds an initial message
const clearAndInitialize = () => {
    consoleOutput.value = [];
};

function consoleLog(resultType, functionResult) {
  const formatted =
    typeof functionResult === 'object'
      ? JSON.stringify(functionResult, null, 2)
      : functionResult;

  consoleOutput.value.push({
    type: resultType,
    message: `Result: ${formatted}`,
  });
}

// Calls the Rust backend command
async function runBackendCommand(commandName, commandArgs = {}) {
    try {
        const result = await invoke(commandName, {...commandArgs});

        return result
    } catch (error) {
        consoleLog('error', error)

        throw error
    } 
}

async function handleRegistrationResponsesClick() {
    const registrationFormType = await runBackendCommand('get_registration_form_type')

    runBackendCommand('main_get_forms_responses', { formType: registrationFormType})

}
async function handleAvailabilityResponsesClick() {
    const availabilityFormType = await runBackendCommand('get_availability_form_type')

    runBackendCommand('main_get_forms_responses', { formType: availabilityFormType})

}
</script>

<template>
    <div class="page-container">
        <div class="page-header">
        <h1>Ottieni Riposte Form</h1>
        </div>

        <div class="controls">
            <button class="success" @click="handleRegistrationResponsesClick()">
            Ottieni risposte registrazione
            </button>
            <button class="failure" @click="handleAvailabilityResponsesClick()">
            Ottieni risposte disponibilità
            </button>
        </div>

        <div class="console">
            <div v-for="(item, index) in consoleOutput" :key="index" :class="['log-entry', item.type]">
                {{ item.message }}
            </div>
        </div>
    </div>
  
</template>

<script>
export default {
  name: 'Page3'
}
</script>

<style scoped>
.console {
  height: 200px;
  padding: 5px;
  background-color: #1e1e1e;
  border: 1px solid #555;
  overflow-y: auto;
  font-size: 0.85em;
}

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

.charts-container {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 30px;
  margin: 25px 0;
}

.chart-placeholder {
  background: #f8f9fa;
  padding: 20px;
  border-radius: 8px;
}

.mock-chart {
  display: flex;
  align-items: end;
  height: 200px;
  gap: 10px;
  padding: 20px;
  background: white;
  border: 1px solid #ddd;
}

.bar {
  flex: 1;
  background: #3498db;
  border-radius: 4px 4px 0 0;
  min-height: 20px;
}

.data-grid {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.data-card {
  background: #f8f9fa;
  padding: 20px;
  border-radius: 8px;
  text-align: center;
  border-left: 4px solid #27ae60;
}

.data-value {
  font-size: 1.8rem;
  font-weight: bold;
  color: #2c3e50;
  margin: 5px 0;
}

.data-percent {
  color: #27ae60;
  font-weight: 600;
}

.reports-section {
  margin-top: 30px;
}

.report-list {
  margin-top: 15px;
}

.report-item {
  display: flex;
  justify-content: space-between;
  padding: 15px;
  background: #f8f9fa;
  margin-bottom: 10px;
  border-radius: 6px;
  border-left: 4px solid #f39c12;
}

.report-title {
  font-weight: 600;
  color: #2c3e50;
}

.report-date {
  color: #7f8c8d;
  font-size: 0.9rem;
}
.search-input{
  width: 50%;
  padding: 12px 20px;
  border: 2px solid #ddd;
  border-radius: 10px;
  font-size: 14px;
}
.controls {
  display: flex;
  gap: 8px;
  margin-bottom: 10px;
}

.controls button {
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 12px 24px;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  font-family: inherit;
}

@media (max-width: 768px) {
  .charts-container {
    grid-template-columns: 1fr;
  }
}
</style>